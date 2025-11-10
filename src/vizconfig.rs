use sdlrig::gfxinfo::MidiEvent;
#[allow(unused_imports)]
use sdlrig::gfxinfo::{MIDI_CONTROL_CHANGE, MIDI_NOTE_OFF, MIDI_NOTE_ON};
use sdlrig::renderspec::{CopyEx, HudText, Mix, MixInput, Reset, SendCmd, SendMidi, SendValue};
use sdlrig::Adjustable;
use sdlrig::{
    gfxinfo::{Asset, GfxEvent, KeyCode, KeyEvent, Knob, Vid, VidInfo, VidMixer},
    renderspec::RenderSpec,
    seek,
};

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;
use std::hash::Hash;
use std::sync::LazyLock;
use std::{error::Error, i64, io::Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct LoopEvent {
    pub frame: i64,
    pub diffs: Vec<StreamSettingsAllFieldsChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct Loop {
    pub tween: bool,
    pub events: Vec<LoopEvent>,
    pub end: i64,
}

impl Loop {
    const fn new() -> Self {
        Self {
            tween: false,
            events: vec![],
            end: i64::MIN,
        }
    }
}

const LOOP_COUNT: usize = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct LoopSettings {
    pub saved: [Loop; LOOP_COUNT],
    pub playing: [bool; LOOP_COUNT],
    pub selected_loop: usize,
    pub record_buffer: Option<Loop>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct PresetSettings {
    pub baseline: StreamSettings,
    pub saved: [Vec<StreamSettingsAllFieldsChange>; 10],
    pub original: Vec<StreamSettingsAllFieldsChange>,
    pub selected_preset: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct PlaybackSettings {
    pub was_reset: bool,
    pub stream: StreamSettings,
    pub presets: PresetSettings,
    pub loops: LoopSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[repr(C)]
pub struct MixConfig {
    pub def: VidMixer,
    pub mix: Mix,
}

pub struct MixerGraph {
    main_mix: MixConfig,
    feedback: MixConfig,
    overlay: MixConfig,
}

impl MixerGraph {
    pub fn new<S: AsRef<str>>(name: S, width: u32, height: u32) -> Self {
        Self {
            main_mix: MixConfig {
                def: VidMixer::builder()
                    .name(format!("{}_main_mix", name.as_ref()))
                    .width(width)
                    .height(height)
                    .header(include_str!("glsl/utils.glsl"))
                    .body(include_str!("glsl/mixer.glsl"))
                    .build(),
                mix: Mix::builder()
                    .name(format!("{}_main_mix", name.as_ref()))
                    .mixed(format!("{}_feedback", name.as_ref()))
                    .mixed(format!("{}_mix", name.as_ref()))
                    .mixed("neutral_mix") // TODO update this in the playback
                    .mixed("neutral_mix") // TODO update this in the playback
                    .mixed("neutral_mix") // TODO update this in the playback
                    .mixed("neutral_mix") // TODO update this in the playback
                    .no_display(true)
                    .build(),
            },
            feedback: MixConfig {
                def: VidMixer::builder()
                    .name(format!("{}_feedback", name.as_ref()))
                    .width(width)
                    .height(height)
                    .build(),
                mix: Mix::builder()
                    .name(format!("{}_feedback", name.as_ref()))
                    .mixed(format!("{}_main_mix", name.as_ref()))
                    .no_display(true)
                    .build(),
            },
            overlay: MixConfig {
                def: VidMixer::builder()
                    .name(format!("{}_overlay", name.as_ref()))
                    .width(width)
                    .height(height)
                    .header(include_str!("glsl/utils.glsl"))
                    .body(include_str!("glsl/overlay.glsl"))
                    .build(),
                mix: Mix::builder()
                    .name(format!("{}_overlay", name.as_ref()))
                    .mixed(format!("{}_main_mix", name.as_ref()))
                    .mixed("blank_mix") // TODO update this in the playback fn
                    .no_display(false)
                    .build(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct AllSettings {
    pub stream_defs: Vec<Vid>,
    pub mix_configs: HashMap<String, MixConfig>,
    pub playback_names: Vec<String>,
    pub distort_names: Vec<(String, String)>,
    pub distort_edge_types: Vec<String>,
    pub overlay_names: Vec<String>,
    pub lut_names: Vec<String>,
    pub blend_modes: Vec<String>,
    pub colors: Vec<(String, String)>,
    pub asset_path: String,
    pub active_idx: usize,
    pub display_idx: usize,
    pub scan_idx: usize,
    pub clipboard: StreamSettings,
    pub selected_knobs: usize,
    pub playback: Vec<PlaybackSettings>,
    pub initial_reset_complete: Vec<bool>,
}

impl AllSettings {
    pub fn colors() -> &'static [(&'static str, &'static str)] {
        &[("black", "0x000000")]
    }

    pub fn distort_names() -> &'static [(&'static str, &'static str)] {
        &[
            //("none", "none"), removed to reduce complexity
            ("neutral", "neutral"),
            ("caustic_adjusted_dx", "caustic_adjusted_dy"),
            ("drops_dx", "drops_dy"),
            ("bend", "neutral"),
            ("stretch_5_dx", "stretch_5_dy"),
            ("shrink_5_dx", "shrink_5_dy"),
            ("digital_color", "digital_color"),
            ("digital_white", "digital_white"),
        ]
    }

    pub fn distort_edge_types() -> &'static [&'static str] {
        &[
            "smear",
            "wrap",
            "mirror",
            "blank",
            "wrap_lr",
            "wrap_ud",
            "mirror_lr",
            "mirror_ud",
        ]
    }

    pub fn overlay_vids() -> &'static [Vid] {
        static OVERLAY_VIDS: LazyLock<Vec<Vid>> = LazyLock::new(|| {
            let mut v = vec![
                Vid::builder()
                    .name("blank")
                    .path(format!("REPLACE/overlays/blank.mp4"))
                    .resolution((1280, 720))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(false)
                    .build(),
                Vid::builder()
                    .name("vhs_overlay")
                    .path(format!("REPLACE/overlays/vhs_overlay.mp4"))
                    .resolution((1280, 720))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(false)
                    .build(),
                Vid::builder()
                    .name("film_dust")
                    .path(format!("REPLACE/overlays/film_dust.mp4"))
                    .resolution((1280, 720))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(false)
                    .build(),
                Vid::builder()
                    .name("tracking")
                    .path(format!("REPLACE/overlays/tracking.mp4"))
                    .resolution((1280, 720))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(false)
                    .build(),
                Vid::builder()
                    .name("bottom")
                    .path(format!("REPLACE/overlays/bottom.mp4"))
                    .resolution((1280, 720))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(false)
                    .build(),
                Vid::builder()
                    .name("colorful")
                    .path(format!("REPLACE/overlays/colorful.mp4"))
                    .resolution((1280, 720))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(false)
                    .build(),
            ];

            let overlay_batch = [
                ("Sci Fi HUD", 13),
                ("VHS", 10),        // down to 7
                ("VHS_Glitch", 10), // remove
                ("VHS_Line", 10),
                ("VHS_N", 5), // remove
                ("VHS_Noise", 7),
                ("VHS_Old", 10),
            ];
            for (name, count) in overlay_batch.iter() {
                for i in 1..=*count {
                    v.push(
                        Vid::builder()
                            .name(format!("{name}_{i}"))
                            .path(format!("REPLACE/overlays/{name}_{i}.mp4"))
                            .resolution((960, 540))
                            .tbq((1, 12800))
                            .pix_fmt("yuv420p")
                            .repeat(true)
                            .realtime(false)
                            .hardware_decode(false)
                            .build(),
                    );
                }
            }

            v
        });
        OVERLAY_VIDS.as_ref()
    }

    pub fn lut_names() -> &'static [&'static str] {
        &[
            "none",
            "creepy",
            "rad",
            "midas",
            "blackwhite",
            "blue",
            "redzone",
            "riso",
            "plague",
            "hyper",
            "sepia",
            "red_pop",
            "green_pop",
            "blue_pop",
            "blacklight",
            "whiteblack",
        ]
    }

    pub fn blend_modes() -> &'static [&'static str] {
        &[
            "disable",
            "addition",
            "and",
            "average",
            "darken",
            "difference",
            "divide",
            "lighten",
            "or",
            "overlay",
            "screen",
            "subtract",
            "xor",
            "alpha",
        ]
    }

    pub fn new<
        S: AsRef<str>,
        SI: IntoIterator,
        VI: IntoIterator<Item = Vid>,
        MCI: IntoIterator<Item = MixConfig>,
    >(
        stream_defs: VI,
        mix_configs: MCI,
        playback_names: SI,
        asset_path: S,
    ) -> Self
    where
        SI::Item: AsRef<str>,
    {
        let playback_names = playback_names
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<_>>();
        let mut stream_defs = stream_defs.into_iter().collect::<Vec<_>>();
        let mut mix_configs = mix_configs
            .into_iter()
            .map(|mc| (mc.def.name.clone(), mc))
            .collect::<HashMap<_, _>>();
        let asset_path = asset_path.as_ref().to_string();

        let distort_names = Self::distort_names()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<Vec<_>>();

        let distort_set = distort_names
            .iter()
            .map(|(x, y)| vec![x, y])
            .flatten()
            .cloned()
            .collect::<HashSet<_>>();

        stream_defs.extend(
            distort_set
                .iter()
                .map(|s| {
                    Vid::builder()
                        .name(s)
                        .path(&format!("{asset_path}/distorts/{s}.mp4"))
                        .resolution((720, 480))
                        .tbq((1, 12800))
                        .pix_fmt("yuv420p")
                        .repeat(true)
                        .realtime(false)
                        .hardware_decode(false)
                        .build()
                })
                .collect::<Vec<_>>(),
        );
        mix_configs.extend(distort_set.iter().map(|s| {
            (
                format!("{s}_mix"),
                MixConfig {
                    def: VidMixer::builder()
                        .name(format!("{s}_mix"))
                        .width(720)
                        .height(480)
                        .build(),
                    mix: Mix::builder()
                        .name(format!("{s}_mix"))
                        .video(s)
                        .no_display(true)
                        .build(),
                },
            )
        }));
        let blend_modes = Self::blend_modes()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let distort_edge_types = Self::distort_edge_types()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let mut overlay_vids = Vec::from(Self::overlay_vids());
        for overlay_vid in &mut overlay_vids {
            overlay_vid.path = overlay_vid.path.replace("REPLACE", &asset_path);
        }

        let overlay_names = overlay_vids
            .iter()
            .map(|v| v.name.clone())
            .collect::<Vec<_>>();

        stream_defs.extend(overlay_vids);
        mix_configs.extend(overlay_names.iter().map(|s| {
            (
                format!("{s}_mix"),
                MixConfig {
                    def: VidMixer::builder()
                        .name(format!("{s}_mix"))
                        .width(720)
                        .height(480)
                        .build(),
                    mix: Mix::builder()
                        .name(format!("{s}_mix"))
                        .video(s)
                        .no_display(false)
                        .build(),
                },
            )
        }));
        let lut_names = Self::lut_names()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let colors = Self::colors()
            .iter()
            .map(|(n, c)| (n.to_string(), c.to_string()))
            .collect::<Vec<_>>();

        let mut playback = vec![];
        for name in &playback_names {
            let input_mix = mix_configs.get_mut(&format!("{name}_mix")).cloned();

            if input_mix.is_none() {
                panic!(
                    "Playback mix {} not found in mix_configs, using default",
                    name
                );
            }

            let input_mix = input_mix.unwrap();

            let mixer_graph = MixerGraph::new(name, input_mix.def.width, input_mix.def.height);

            mix_configs.insert(
                mixer_graph.main_mix.def.name.clone(),
                mixer_graph.main_mix.clone(),
            );
            mix_configs.insert(
                mixer_graph.feedback.def.name.clone(),
                mixer_graph.feedback.clone(),
            );
            mix_configs.insert(
                mixer_graph.overlay.def.name.clone(),
                mixer_graph.overlay.clone(),
            );

            let first_video = match &input_mix.mix.inputs.get(0) {
                Some(MixInput::Video(s)) => s.clone(),
                _ => String::new(),
            };

            let pb = PlaybackSettings {
                was_reset: true,
                stream: StreamSettings::new(
                    name.clone(),
                    first_video.clone(),
                    input_mix.def.name.clone(),
                    mixer_graph.main_mix.def.name.clone(),
                    mixer_graph.feedback.def.name.clone(),
                    mixer_graph.overlay.def.name.clone(),
                ),
                presets: PresetSettings {
                    baseline: StreamSettings::new(
                        name.clone(),
                        first_video.clone(),
                        input_mix.def.name.clone(),
                        mixer_graph.main_mix.def.name.clone(),
                        mixer_graph.feedback.def.name.clone(),
                        mixer_graph.overlay.def.name.clone(),
                    ),
                    saved: [const { vec![] }; 10],
                    original: vec![],
                    selected_preset: None,
                },
                loops: LoopSettings {
                    record_buffer: None,
                    saved: [Loop::new(), Loop::new(), Loop::new(), Loop::new()],
                    playing: [false, false, false, false],
                    selected_loop: 0,
                },
            };

            playback.push(pb);
        }

        let playback_len = playback.len();

        Self {
            stream_defs,
            mix_configs,
            playback_names,
            blend_modes,
            overlay_names,
            distort_names,
            distort_edge_types,
            lut_names,
            colors,
            asset_path,
            clipboard: StreamSettings::new("", "", "", "", "", ""),
            selected_knobs: 1,
            playback,
            initial_reset_complete: vec![false; playback_len],
            active_idx: 0,
            scan_idx: 0,
            display_idx: 0,
        }
    }

    pub fn update_record_and_get_specs<F>(
        &mut self,
        reg_events: &[GfxEvent],
        frame: i64,
        midi_callback: Option<F>,
    ) -> Result<Vec<RenderSpec>, Box<dyn Error>>
    where
        F: Fn(&mut AllSettings, &MidiEvent),
    {
        static ONCE: std::sync::Once = std::sync::Once::new();

        ONCE.call_once(|| {
            self.initial_reset_complete = vec![false; self.playback.len()];
        });

        let orig = if reg_events.contains(&GfxEvent::ReloadEvent()) {
            let mut tmp = self.playback.clone();
            for i in 0..tmp.len() {
                tmp[i].stream.reset();
            }
            tmp
        } else {
            self.playback.clone()
        };

        let mut specs = vec![];
        // Always capture live events even while recording is playing
        self.update(reg_events, frame, midi_callback)?;

        let active_diff = self.playback[self.active_idx]
            .stream
            .diff(&orig[self.active_idx].stream)
            .into_iter()
            .collect::<Vec<_>>();
        if self.playback[self.active_idx].was_reset || active_diff.len() > 0 {
            self.playback[self.active_idx].was_reset = false;
            specs.append(&mut self.reload_encoders_for_active_idx());
        }

        for i in 0..self.playback.len() {
            let diffs = orig[i]
                .stream
                .diff(&self.playback[i].stream)
                .into_iter()
                .collect::<Vec<_>>();
            specs.append(
                &mut self.playback[i]
                    .stream
                    .get_commands(&diffs.iter().map(|d| d.field).collect::<Vec<_>>()),
            );
            if let Some(buf) = self.playback[i].loops.record_buffer.as_mut() {
                let filtered_diffs = diffs
                    .into_iter()
                    .filter(|d| StreamSettings::should_record(&d.field))
                    .collect::<Vec<_>>();
                // Save the diffs for this frame
                if filtered_diffs.len() > 0 {
                    buf.events.push(LoopEvent {
                        frame,
                        diffs: filtered_diffs,
                    });
                }
            }
        }

        for i in 0..self.playback.len() {
            for j in 0..self.playback[i].loops.saved.len() {
                if self.playback[i].loops.playing[j] {
                    //send events for recorded loop at this frame
                    let lp = &self.playback[i].loops.saved[j];
                    if lp.events.len() > 0 {
                        let start = lp.events[0].frame;
                        let lp_len = lp.end - start;
                        let curr = (frame % lp_len) + start;
                        let mut prev_frame = start;
                        let mut next = lp.end;
                        let mut diffs = vec![];
                        let mut found = false;
                        let mut prev_state = HashMap::new();
                        for event in &lp.events {
                            if !found && event.frame >= curr {
                                next = event.frame;
                                diffs = event.diffs.clone();
                                found = true;
                            } else if !found {
                                prev_frame = event.frame;
                            } else {
                                // collect all the state from the back end of the loop
                                for diff in &event.diffs {
                                    prev_state.insert(diff.field, diff.new_value.clone());
                                }
                            }
                        }

                        if !found {
                            // if we didn't find a next event, act like we will loop to the first event
                            next = lp.end;
                            diffs = lp.events[0].diffs.clone();
                            // prev will be set from above
                        }

                        // overwrite the prev_state with any changes from the front of the loop
                        for event in &lp.events {
                            if event.frame < next {
                                for diff in &event.diffs {
                                    prev_state.insert(diff.field, diff.new_value.clone());
                                }
                            }
                        }

                        if curr == next {
                            self.playback[i].stream.apply_diff(&diffs);
                        } else if lp.tween {
                            let p = ((curr - prev_frame) as f64 / (next - prev_frame) as f64).abs();
                            let mut tween_diffs = vec![];
                            for diff in &diffs {
                                if let Some(prev_value) = prev_state.get(&diff.field).cloned() {
                                    if let Some(tweened) =
                                        &self.playback[i].stream.tween_diff(prev_value, *diff, p)
                                    {
                                        tween_diffs.push(*tweened);
                                    }
                                }
                            }
                            diffs = tween_diffs;
                            self.playback[i].stream.apply_diff(&diffs);
                        }

                        specs.append(
                            &mut self.playback[i]
                                .stream
                                .get_commands(&diffs.iter().map(|d| d.field).collect::<Vec<_>>()),
                        );
                    }
                }
            }
        }

        specs.push(
            HudText {
                text: self.hud(&VidInfo::default()),
            }
            .into(),
        );

        Ok(specs)
    }

    pub fn update<F>(
        &mut self,
        reg_events: &[GfxEvent],
        frame: i64,
        midi_callback: Option<F>,
    ) -> Result<(), Box<dyn Error>>
    where
        F: Fn(&mut AllSettings, &MidiEvent),
    {
        //Update steams for incoming frame events
        for ge in reg_events {
            match ge {
                GfxEvent::FrameEvent(fe) => {
                    if let Some((eidx, _)) = self
                        .playback
                        .iter()
                        .enumerate()
                        .find(|(_, s)| s.stream.first_video() == fe.stream)
                    {
                        self.playback[eidx].stream.real_ts = fe.real_ts;
                        self.playback[eidx].stream.continuous_ts = fe.continuous_ts;
                    }
                }
                GfxEvent::ReloadEvent() => (), // needs to be handled elsewhere
                GfxEvent::MidiEvent(me) => {
                    match (me.device.as_str(), me.kind, me.channel, me.key, me.velocity) {
                        (MFT, MIDI_CONTROL_CHANGE, 3, 8, _) => {
                            self.scan_idx = (self.scan_idx + 1).clamp(0, self.playback.len() - 1)
                        }
                        (MFT, MIDI_CONTROL_CHANGE, 3, 9, _) => {
                            self.scan_idx = ((self.scan_idx as i32) - 1)
                                .clamp(0, self.playback.len() as i32 - 1)
                                as usize
                        }
                        (MFT, MIDI_CONTROL_CHANGE, 3, 11, _) => {
                            self.active_idx = self.scan_idx;
                        }
                        (MFT, MIDI_CONTROL_CHANGE, 3, 12, _) => {
                            self.display_idx = self.scan_idx;
                        }
                        _ => (),
                    }
                    if let Some(cb) = &midi_callback {
                        cb(self, me);
                    }
                    self.video_fight_cb(me);
                }
                GfxEvent::KeyEvent(ke) => {
                    let selected_idx = self.active_idx;
                    match ke {
                        //DUMP
                        KeyEvent {
                            key: KeyCode::SDLK_d,
                            down: true,
                            shift,
                            ctl,
                            ..
                        } => {
                            if *shift {
                                let ser = serde_json::to_string_pretty(&self.playback)?;
                                match std::fs::File::create("/tmp/viz/playback_dump.json") {
                                    Ok(mut f) => write!(f, "{}", ser)?,
                                    Err(e) => eprintln!("{}:{} - {}", file!(), line!(), e),
                                }
                            } else if *ctl {
                                match std::fs::File::open("/tmp/viz/playback_dump.json") {
                                    Ok(f) => {
                                        let playback =
                                            serde_json::from_reader::<_, Vec<PlaybackSettings>>(f)?;
                                        if playback.len() != self.playback.len() {
                                            eprintln!(
                                                "Playback dump length mismatch: expected {}, got {}",
                                                self.playback.len(),
                                                playback.len()
                                            );
                                            continue;
                                        }

                                        for (i, pb) in playback.into_iter().enumerate() {
                                            let ident = self.playback[i].stream.ident.clone();
                                            self.playback[i] = pb;
                                            self.playback[i].stream.ident = ident;
                                        }
                                    }
                                    Err(e) => eprintln!("{}:{} - {}", file!(), line!(), e),
                                }
                            } else {
                                let ser = serde_json::to_string_pretty(&self.playback)?;
                                eprintln!("{}", ser);
                            }
                        }
                        //RESET
                        KeyEvent {
                            key: KeyCode::SDLK_r,
                            down: true,
                            ..
                        } => {
                            self.playback[selected_idx].stream.reset();
                            self.playback[selected_idx].was_reset = true;
                        }
                        //COPY
                        KeyEvent {
                            key: KeyCode::SDLK_c,
                            down: true,
                            ..
                        } => {
                            self.clipboard = self.playback[selected_idx].stream.clone();
                        }
                        //PASTE
                        KeyEvent {
                            key: KeyCode::SDLK_v,
                            down: true,
                            ..
                        } => {
                            let ident = self.playback[selected_idx].stream.ident.clone();
                            self.playback[selected_idx].stream = self.clipboard.clone();
                            self.playback[selected_idx].stream.ident = ident;
                        }
                        //UPDATE aka refresh
                        KeyEvent {
                            key: KeyCode::SDLK_u,
                            down: true,
                            ..
                        } => {
                            eprintln!("This did nothing")
                        }
                        // ExCHANGE
                        KeyEvent {
                            key: KeyCode::SDLK_x,
                            down: true,
                            ..
                        } => {
                            // swap preserving IDs from before the swap
                            let ident = self.playback[selected_idx].stream.ident.clone();
                            let temp = self.playback[selected_idx].stream.clone();
                            self.playback[selected_idx].stream = self.clipboard.clone();
                            self.playback[selected_idx].stream.ident = ident;
                            self.clipboard = temp;
                        }
                        // Save as baseline for preset creation
                        KeyEvent {
                            key: KeyCode::SDLK_b,
                            down: true,
                            ctl,
                            ..
                        } => {
                            if *ctl {
                                let diffs = self.playback[selected_idx]
                                    .stream
                                    .diff(&self.playback[selected_idx].presets.baseline)
                                    .into_iter()
                                    .filter(|d| StreamSettings::should_record(&d.field))
                                    .collect::<Vec<_>>();
                                self.playback[selected_idx].stream.apply_diff(&diffs);
                            } else {
                                self.playback[selected_idx].presets.baseline =
                                    self.playback[selected_idx].stream.clone();
                            }
                        }
                        // Start/Stop/Set/Paste a preset
                        KeyEvent {
                            key:
                                KeyCode::SDLK_1
                                | KeyCode::SDLK_2
                                | KeyCode::SDLK_3
                                | KeyCode::SDLK_4
                                | KeyCode::SDLK_5
                                | KeyCode::SDLK_6
                                | KeyCode::SDLK_7
                                | KeyCode::SDLK_8,
                            repeat,
                            shift,
                            down,
                            ctl,
                            alt,
                            ..
                        } => {
                            let selected_preset =
                                ke.key.clone() as u8 as usize - KeyCode::SDLK_0 as u8 as usize;
                            let saved_diff =
                                self.playback[selected_idx].presets.saved[selected_preset].clone();
                            if *shift && *down {
                                // save preset
                                self.playback[selected_idx].presets.saved[selected_preset] = self
                                    .playback[selected_idx]
                                    .presets
                                    .baseline
                                    .diff(&self.playback[selected_idx].stream)
                                    .into_iter()
                                    .filter(|d| StreamSettings::should_record(&d.field))
                                    .collect::<Vec<_>>();
                            } else if *alt && *down && !*ctl && !*shift {
                                // save time
                                self.playback[selected_idx].presets.saved[selected_preset] =
                                    vec![StreamSettingsAllFieldsChange {
                                        field: StreamSettingsAllFieldsEnum::EXACT_SEC,
                                        new_value: self.playback[self.active_idx].stream.real_ts.0
                                            as f64
                                            / self.playback[self.active_idx].stream.real_ts.1
                                                as f64,
                                    }];
                            } else if *ctl
                                && *down
                                && self.playback[selected_idx]
                                    .presets
                                    .selected_preset
                                    .is_none()
                            {
                                // paste preset
                                self.playback[selected_idx].stream.apply_diff(&saved_diff);
                            } else if !shift && !ctl && !repeat && !alt && *down {
                                let mut applied = self.playback[selected_idx].stream.clone();
                                applied.apply_diff(&saved_diff);
                                let mut reverse_diff =
                                    applied.diff(&self.playback[selected_idx].stream);
                                self.playback[selected_idx]
                                    .presets
                                    .original
                                    .append(&mut reverse_diff);
                                self.playback[selected_idx].stream = applied;
                                self.playback[selected_idx]
                                    .presets
                                    .selected_preset
                                    .replace(selected_preset);
                            } else if !shift
                                && !ctl
                                && !repeat
                                && !alt
                                && !down
                                && Some(selected_preset)
                                    == self.playback[selected_idx].presets.selected_preset.clone()
                            {
                                //restore original, reverse order diffs to apply
                                self.playback[selected_idx].presets.original.reverse();
                                let diff = self.playback[selected_idx].presets.original.clone();
                                self.playback[selected_idx].stream.apply_diff(&diff);
                                self.playback[selected_idx].presets.selected_preset.take();
                                self.playback[selected_idx].presets.original.clear();
                            }
                        }
                        // LOOPS
                        KeyEvent {
                            key: KeyCode::SDLK_l,
                            down: true,
                            shift,
                            ..
                        } => {
                            let selected_loop = self.playback[selected_idx].loops.selected_loop;
                            if *shift {
                                self.playback[selected_idx].loops.record_buffer = None;
                                self.playback[selected_idx].loops.saved[selected_loop] =
                                    Loop::new();
                            } else if let Some(mut buf) =
                                self.playback[selected_idx].loops.record_buffer.take()
                            {
                                if buf.events.len() > 0 {
                                    buf.end = frame;
                                    self.playback[selected_idx].loops.saved[selected_loop] = buf;
                                }
                            } else {
                                self.playback[selected_idx].loops.record_buffer = Some(Loop {
                                    tween: self.playback[selected_idx].stream.tween != 0,
                                    events: vec![],
                                    end: i64::MIN,
                                });
                            }
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_s,
                            down: true,
                            ..
                        } => {
                            let mix = self.playback[self.active_idx].stream.overlay_mix();
                            let specs = self.get_playback_specs(mix, (0, 0, 1, 1), (0, 0, 1, 1));
                            let last = specs
                                .iter()
                                .filter_map(|s| {
                                    if let RenderSpec::Mix(mix) = s {
                                        if let Some(MixInput::Video(v)) = mix.inputs.get(0) {
                                            Some(v.clone())
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                })
                                .last();
                            if let Some(last) = last {
                                for i in 0..self.playback.len() {
                                    if self.playback[i].stream.first_video() == last {
                                        self.playback[i].stream.exact_sec = 0.01;
                                        break;
                                    }
                                }
                            }
                        }
                        // PLAYBACK MODE
                        KeyEvent {
                            key: KeyCode::SDLK_p,
                            down: true,
                            ..
                        } => {
                            let selected_loop = self.playback[selected_idx].loops.selected_loop;
                            self.playback[selected_idx].loops.playing[selected_loop] =
                                !self.playback[selected_idx].loops.playing[selected_loop];
                        }
                        // PAUSE
                        KeyEvent {
                            key: KeyCode::SDLK_SPACE,
                            down: true,
                            ..
                        } => {
                            self.playback[self.active_idx].stream.toggle_pause();
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_h,
                            down: true,
                            ..
                        } => {
                            // self.scan_idx = (self.scan_idx as i64 + 1)
                            //     .clamp(0, self.playback.len() as i64 - 1)
                            //     as usize;
                            // self.active_idx = self.scan_idx;
                            // self.display_idx = self.scan_idx;
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_g,
                            down: true,
                            ..
                        } => {
                            // self.scan_idx = (self.scan_idx as i64 - 1)
                            //     .clamp(0, self.playback.len() as i64 - 1)
                            //     as usize;
                            // self.active_idx = self.scan_idx;
                            // self.display_idx = self.scan_idx;
                        }

                        // Adjust Settings
                        KeyEvent {
                            key:
                                KeyCode::SDLK_F1
                                | KeyCode::SDLK_F2
                                | KeyCode::SDLK_F3
                                | KeyCode::SDLK_F4
                                | KeyCode::SDLK_F5
                                | KeyCode::SDLK_F6
                                | KeyCode::SDLK_F7
                                | KeyCode::SDLK_F8
                                | KeyCode::SDLK_F9
                                | KeyCode::SDLK_F10
                                | KeyCode::SDLK_F11
                                | KeyCode::SDLK_F12,
                            down: true,
                            ..
                        } => {
                            self.selected_knobs =
                                (ke.key.clone() as u32 - KeyCode::SDLK_F1 as u32) as usize + 1;
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_F13 | KeyCode::SDLK_F14 | KeyCode::SDLK_F15,
                            down: true,
                            ..
                        } => {
                            self.selected_knobs =
                                (ke.key.clone() as u32 - KeyCode::SDLK_F13 as u32) as usize + 13;
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_DOWN,
                            down: true,
                            ..
                        } => {
                            self.selected_knobs = (self.selected_knobs + 1).clamp(1, 12);
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_UP,
                            down: true,
                            ..
                        } => {
                            self.selected_knobs =
                                (self.selected_knobs as i32 - 1 as i32).clamp(1, 12) as usize;
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_LEFT,
                            down: true,
                            shift,
                            ..
                        } => {
                            let mix = self.playback[self.active_idx].stream.overlay_mix();
                            let specs = self.get_playback_specs(mix, (0, 0, 1, 1), (0, 0, 1, 1));
                            let last = specs
                                .iter()
                                .filter_map(|s| {
                                    if let RenderSpec::Mix(mix) = s {
                                        if let Some(MixInput::Video(v)) = mix.inputs.get(0) {
                                            Some(v.clone())
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                })
                                .last();
                            if let Some(last) = last {
                                for i in 0..self.playback.len() {
                                    if self.playback[i].stream.first_video() == last {
                                        self.playback[i].stream.adjust_delta_sec(if *shift {
                                            -10.0
                                        } else {
                                            -1.0
                                        });
                                        break;
                                    }
                                }
                            }
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_RIGHT,
                            down: true,
                            shift,
                            ..
                        } => {
                            let mix = self.playback[self.active_idx].stream.overlay_mix();
                            let specs = self.get_playback_specs(mix, (0, 0, 1, 1), (0, 0, 1, 1));
                            let last = specs
                                .iter()
                                .filter_map(|s| {
                                    if let RenderSpec::Mix(mix) = s {
                                        if let Some(MixInput::Video(v)) = mix.inputs.get(0) {
                                            Some(v.clone())
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                })
                                .last();
                            if let Some(last) = last {
                                for i in 0..self.playback.len() {
                                    if self.playback[i].stream.first_video() == last {
                                        self.playback[i].stream.adjust_delta_sec(if *shift {
                                            10.0
                                        } else {
                                            1.0
                                        });
                                        break;
                                    }
                                }
                            }
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_COMMA,
                            shift,
                            down: true,
                            ..
                        } => {
                            self.adjust(Knob::B, *shift, -1.0);
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_PERIOD,
                            shift,
                            down: true,
                            ..
                        } => {
                            self.adjust(Knob::B, *shift, 1.0);
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_SLASH,
                            shift,
                            down: true,
                            ..
                        } => {
                            self.adjust(Knob::CB, *shift, 1.0);
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_9,
                            shift,
                            down: true,
                            ..
                        } => {
                            self.adjust(Knob::L, *shift, -1.0);
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_0,
                            shift,
                            down: true,
                            ..
                        } => {
                            self.adjust(Knob::L, *shift, 1.0);
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_MINUS,
                            shift,
                            down: true,
                            ..
                        } => {
                            self.adjust(Knob::CL, *shift, 1.0);
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_LEFTBRACKET,
                            shift,
                            down: true,
                            ..
                        } => {
                            self.adjust(Knob::R, *shift, -1.0);
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_RIGHTBRACKET,
                            shift,
                            down: true,
                            ..
                        } => {
                            self.adjust(Knob::R, *shift, 1.0);
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_BACKSLASH,
                            shift,
                            down: true,
                            ..
                        } => {
                            self.adjust(Knob::CR, *shift, 1.0);
                        }
                        _ => (),
                    }
                }
            }
        }

        // scroll all the settings one frame
        for i in 0..self.playback.len() {
            let stream = &mut self.playback[i].stream;
            stream.scrolled_h += stream.scroll_h;
            stream.scrolled_v += stream.scroll_v;
        }
        Ok(())
    }

    pub fn adjust(&mut self, kn: Knob, shift: bool, inc: f64) {
        let playback = &mut self.playback[self.active_idx];
        let inc = if shift { inc * 10.0 } else { inc };
        match (kn, self.selected_knobs) {
            (Knob::B, 1) => {
                playback.loops.selected_loop =
                    (playback.loops.selected_loop as f64 + inc).clamp(0.0, 3.0) as usize;
            }
            (Knob::B, 10) => {
                self.scan_idx = (self.scan_idx as i64 + inc as i64)
                    .clamp(0, self.playback.len() as i64 - 1)
                    as usize;
            }
            (Knob::CB, 10) => {
                if shift {
                    self.display_idx = self.active_idx;
                } else {
                    self.active_idx = self.scan_idx;
                    self.playback[self.active_idx].was_reset = true;
                }
            }
            _ => {
                playback.stream.adjust(kn, self.selected_knobs, inc);
            }
        }
    }

    pub fn clean_up_by_specs(&mut self, specs: &mut Vec<RenderSpec>) {
        // RESET SEEK
        for i in 0..self.playback.len() {
            self.playback[i].stream.set_delta_sec(0.0);
            self.playback[i].stream.set_scrub(0.0);
            self.playback[i].stream.set_exact_sec(0.0);
        }

        let referenced = specs
            .iter()
            .filter_map(|s| {
                if let RenderSpec::Mix(mix) = s {
                    Some(mix.name.clone())
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();

        for i in 0..self.playback.len() {
            if !referenced.contains(&self.playback[i].stream.main_mix())
                && self.initial_reset_complete[i] == true
            {
                eprintln!("unloading {i}");
                self.initial_reset_complete[i] = false;
                specs.push(
                    Reset {
                        target: self.playback[i].stream.main_mix(),
                    }
                    .into(),
                );
            }
        }
    }

    pub fn hud(&self, vid_info: &VidInfo) -> String {
        macro_rules! get {
            ($name:ident) => {{
                self.playback[self.active_idx].stream.$name
            }};
        }

        let tc = self.playback[self.active_idx].stream.real_ts.0 as f64
            / self.playback[self.active_idx].stream.real_ts.1 as f64;

        let hour = (tc / 3600.0).floor() as u32;
        let minute = ((tc % 3600.0) / 60.0).floor() as u32;
        let second = (tc % 60.0).floor() as u32;
        let frame = ((tc - tc.floor()) * 24.0 as f64).round() as u32;

        format!(
            r#"
loops: [{}], loop capture: {}

{}[1] Loop Ctl Loop:   {:<04.3}
              tween: {}
              flash: {}

{}[2]  Usr {} Toggle {}
      Mix rr {} | rg {} | rb {} | ra {}
          gr {} | gg {} | gb {} | ga {}
          br {} | bg {} | bb {} | ba {}
          ar {} | ag {} | ab {} | aa {}

{}[3] Boost: {:0.2}
     Threshold: {:0.4}
     Distort Level: {}  Warp Level: {}

{}[4] skew  {}, {}
           {}, {}

{}[5] Colorkey sim: {:.3} blend: {:.3}  enable: {}
     color({:10}) = {:10} >{:10}< {:10}

{}[6] Scroll h: {:0.4} v: {:0.4}

{}[7] rh={} rv={} gh={} gv={} bh={} bv={} ah={} av={}
     negate={}

{}[8] Distort: dx = {:.3} dy = {:.3}
     Rotation: {:.1}

{}[9] Edge({:10}): {:10} >{:10}< {:10}
     Warp({:14}): {:14} >{:14}< {:14}
     Map({:14}): {:14} >{:14}< {:14}
    
{}[10] LUT({:10}): {:10} >{:10}< {:10}
      Stream({:10}): {:10} >{:10}< {:10}
      Displayed({})
      Style({:.0}): {:.0}

{}[11] Scanlines({:10}): {:10} >{:10}< {:10}
      Blend({:10}): {:10} >{:10}< {:10}
      Overlay({:10}): {:10} >{:10}< {:10}

{}[12] Scrub {:.3}
 Selected {}
 Duration: {}
 Timecode: {:02}:{:02}:{:02}:{:02}
 ts: {:-6}/{:-6} = {:.3}
"#,
            self.playback[self.active_idx]
                .loops
                .playing
                .iter()
                .map(|p| p.then_some(" ON").unwrap_or("OFF"))
                .collect::<Vec<_>>()
                .join(","),
            if self.playback[self.active_idx].loops.record_buffer.is_some() {
                "Recording"
            } else {
                "n/a"
            },
            if self.selected_knobs == 1 { ">" } else { " " },
            // TIMING
            self.playback[self.active_idx].loops.selected_loop,
            get!(tween),
            get!(flash_enable) as u8,
            // USR
            if self.selected_knobs == 2 { ">" } else { " " },
            get!(usr_var) as i32,
            get!(usr_toggle),
            // MIX
            if get!(color_mix_selected) as u8 == 0 {
                format!("({:+.04})", get!(rr))
            } else {
                format!(" {:+.04} ", get!(rr))
            },
            if get!(color_mix_selected) as u8 == 1 {
                format!("({:+.04})", get!(rg))
            } else {
                format!(" {:+.04} ", get!(rg))
            },
            if get!(color_mix_selected) as u8 == 2 {
                format!("({:+.04})", get!(rb))
            } else {
                format!(" {:+.04} ", get!(rb))
            },
            if get!(color_mix_selected) as u8 == 3 {
                format!("({:+.04})", get!(ra))
            } else {
                format!(" {:+.04} ", get!(ra))
            },
            if get!(color_mix_selected) as u8 == 4 {
                format!("({:+.04})", get!(gr))
            } else {
                format!(" {:+.04} ", get!(gr))
            },
            if get!(color_mix_selected) as u8 == 5 {
                format!("({:+.04})", get!(gg))
            } else {
                format!(" {:+.04} ", get!(gg))
            },
            if get!(color_mix_selected) as u8 == 6 {
                format!("({:+.04})", get!(gb))
            } else {
                format!(" {:+.04} ", get!(gb))
            },
            if get!(color_mix_selected) as u8 == 7 {
                format!("({:+.04})", get!(ga))
            } else {
                format!(" {:+.04} ", get!(ga))
            },
            if get!(color_mix_selected) as u8 == 8 {
                format!("({:+.04})", get!(br))
            } else {
                format!(" {:+.04} ", get!(br))
            },
            if get!(color_mix_selected) as u8 == 9 {
                format!("({:+.04})", get!(bg))
            } else {
                format!(" {:+.04} ", get!(bg))
            },
            if get!(color_mix_selected) as u8 == 10 {
                format!("({:+.04})", get!(bb))
            } else {
                format!(" {:+.04} ", get!(bb))
            },
            if get!(color_mix_selected) as u8 == 11 {
                format!("({:+.04})", get!(ba))
            } else {
                format!(" {:+.04} ", get!(ba))
            },
            if get!(color_mix_selected) as u8 == 12 {
                format!("({:+.04})", get!(ar))
            } else {
                format!(" {:+.04} ", get!(ar))
            },
            if get!(color_mix_selected) as u8 == 13 {
                format!("({:+.04})", get!(ag))
            } else {
                format!(" {:+.04} ", get!(ag))
            },
            if get!(color_mix_selected) as u8 == 14 {
                format!("({:+.04})", get!(ab))
            } else {
                format!(" {:+.04} ", get!(ab))
            },
            if get!(color_mix_selected) as u8 == 15 {
                format!("({:+.04})", get!(aa))
            } else {
                format!(" {:+.04} ", get!(aa))
            },
            //BOOST
            if self.selected_knobs == 3 { ">" } else { " " },
            get!(boost),
            get!(threshold),
            if get!(distort_warp_select) == 0 {
                format!(">{:0.3}", get!(distort_level))
            } else {
                format!(" {:0.3}", get!(distort_level))
            },
            if get!(distort_warp_select) == 0 {
                format!(" {:0.3}", get!(warp_level))
            } else {
                format!(">{:0.3}", get!(warp_level))
            },
            //SKEW
            if self.selected_knobs == 4 { ">" } else { " " },
            if get!(skew_selected) as u8 == 0 {
                format!(">({:1.02}, {:1.02})", get!(skew_x0), get!(skew_y0))
            } else {
                format!(" ({:1.02}, {:1.02})", get!(skew_x0), get!(skew_y0))
            },
            if get!(skew_selected) as u8 == 1 {
                format!(">({:1.02}, {:1.02})", get!(skew_x1), get!(skew_y1))
            } else {
                format!(" ({:1.02}, {:1.02})", get!(skew_x1), get!(skew_y1))
            },
            if get!(skew_selected) as u8 == 2 {
                format!(">({:1.02}, {:1.02})", get!(skew_x2), get!(skew_y2))
            } else {
                format!(" ({:1.02}, {:1.02})", get!(skew_x2), get!(skew_y2))
            },
            if get!(skew_selected) as u8 == 3 {
                format!(">({:1.02}, {:1.02})", get!(skew_x3), get!(skew_y3))
            } else {
                format!(" ({:1.02}, {:1.02})", get!(skew_x3), get!(skew_y3))
            },
            // COLOR KEY
            if self.selected_knobs == 5 { ">" } else { " " },
            get!(sim),
            get!(blend),
            get!(video_key_enable) as u8,
            self.colors[get!(video_key_color_selected) as usize].0,
            if get!(video_key_color_scan) >= 1.0 {
                &self.colors[(get!(video_key_color_scan) - 1.0) as usize].0
            } else {
                ""
            },
            &self.colors[get!(video_key_color_scan) as usize].0,
            if get!(video_key_color_scan) < self.colors.len() as f64 - 1.0 {
                &self.colors[(get!(video_key_color_scan) + 1.0) as usize].0
            } else {
                ""
            },
            //SCROLL
            if self.selected_knobs == 6 { ">" } else { " " },
            get!(scroll_h),
            get!(scroll_v),
            //RGB SHIFT
            if self.selected_knobs == 7 { ">" } else { " " },
            if get!(shift_select) as usize == 0 {
                format!("{:0.3}<", get!(rh))
            } else {
                format!("{:0.3} ", get!(rh))
            },
            if get!(shift_select) as usize == 1 {
                format!("{:0.3}<", get!(rv))
            } else {
                format!("{:0.3} ", get!(rv))
            },
            if get!(shift_select) as usize == 2 {
                format!("{:0.3}<", get!(gh))
            } else {
                format!("{:0.3} ", get!(gh))
            },
            if get!(shift_select) as usize == 3 {
                format!("{:0.3}<", get!(gv))
            } else {
                format!("{:0.3} ", get!(gv))
            },
            if get!(shift_select) as usize == 4 {
                format!("{:0.3}<", get!(bh))
            } else {
                format!("{:0.3} ", get!(bh))
            },
            if get!(shift_select) as usize == 5 {
                format!("{:0.3}<", get!(bv))
            } else {
                format!("{:0.3} ", get!(bv))
            },
            if get!(shift_select) as usize == 6 {
                format!("{:0.3}<", get!(ah))
            } else {
                format!("{:0.3} ", get!(ah))
            },
            if get!(shift_select) as usize == 7 {
                format!("{:0.3}<", get!(av))
            } else {
                format!("{:0.3} ", get!(av))
            },
            get!(negate),
            //DISTORT POSITION
            if self.selected_knobs == 8 { ">" } else { " " },
            get!(dx),
            get!(dy),
            get!(feedback_rotation).to_degrees(),
            //DISTORT METHOD
            if self.selected_knobs == 9 { ">" } else { " " },
            self.distort_edge_types[get!(distort_edge_selected) as usize],
            if get!(distort_edge_scan) >= 1.0 {
                &self.distort_edge_types[(get!(distort_edge_scan) - 1.0) as usize]
            } else {
                ""
            },
            self.distort_edge_types[get!(distort_edge_scan) as usize],
            if get!(distort_edge_scan) < self.distort_edge_types.len() as f64 - 1.0 {
                &self.distort_edge_types[(get!(distort_edge_scan) + 1.0) as usize]
            } else {
                ""
            },
            self.distort_names[get!(warp_selected) as usize].0,
            if get!(warp_scan) >= 1.0 {
                &self.distort_names[(get!(warp_scan) - 1.0) as usize].0
            } else {
                ""
            },
            &self.distort_names[get!(warp_scan) as usize].0,
            if get!(warp_scan) < self.distort_names.len() as f64 - 1.0 {
                &self.distort_names[(get!(warp_scan) + 1.0) as usize].0
            } else {
                ""
            },
            &self.distort_names[get!(distort_selected) as usize].0,
            if get!(distort_scan) >= 1.0 {
                &self.distort_names[(get!(distort_scan) - 1.0) as usize].0
            } else {
                ""
            },
            &self.distort_names[get!(distort_scan) as usize].0,
            if get!(distort_scan) < self.distort_names.len() as f64 - 1.0 {
                &self.distort_names[(get!(distort_scan) + 1.0) as usize].0
            } else {
                ""
            },
            //LUTS
            if self.selected_knobs == 10 { ">" } else { " " },
            &self.lut_names[get!(lut_selected) as usize],
            if get!(lut_scan) >= 1.0 {
                &self.lut_names[(get!(lut_scan) - 1.0) as usize]
            } else {
                ""
            },
            &self.lut_names[get!(lut_scan) as usize],
            if get!(lut_scan) < self.lut_names.len() as f64 - 1.0 {
                &self.lut_names[(get!(lut_scan) + 1.0) as usize]
            } else {
                ""
            },
            //STREAM
            self.playback_names[self.active_idx as usize],
            if self.scan_idx >= 1 {
                &self.playback_names[self.scan_idx - 1]
            } else {
                ""
            },
            &self.playback_names[self.scan_idx],
            if (self.scan_idx as i64) < self.playback_names.len() as i64 - 1 {
                &self.playback_names[self.scan_idx + 1]
            } else {
                ""
            },
            &self.playback_names[self.display_idx],
            get!(feedback_style_selected),
            get!(feedback_style_scan),
            //SCANLINES MODES
            if self.selected_knobs == 11 { ">" } else { " " },
            &self.blend_modes[get!(scanlines_selected) as usize],
            if get!(scanlines_scan) >= 1.0 {
                &self.blend_modes[(get!(scanlines_scan) - 1.0) as usize]
            } else {
                ""
            },
            &self.blend_modes[get!(scanlines_scan) as usize],
            if get!(scanlines_scan) < self.blend_modes.len() as f64 - 1.0 {
                &self.blend_modes[(get!(scanlines_scan) + 1.0) as usize]
            } else {
                ""
            },
            //BLEND MODES
            &self.blend_modes[get!(blend_selected) as usize],
            if get!(blend_scan) >= 1.0 {
                &self.blend_modes[(get!(blend_scan) - 1.0) as usize]
            } else {
                ""
            },
            &self.blend_modes[get!(blend_scan) as usize],
            if get!(blend_scan) < self.blend_modes.len() as f64 - 1.0 {
                &self.blend_modes[(get!(blend_scan) + 1.0) as usize]
            } else {
                ""
            },
            //OVERLAYS
            &self.overlay_names[get!(overlay_selected) as usize],
            if get!(overlay_scan) >= 1.0 {
                &self.overlay_names[(get!(overlay_scan) - 1.0) as usize]
            } else {
                ""
            },
            &self.overlay_names[get!(overlay_scan) as usize],
            if get!(overlay_scan) < self.overlay_names.len() as f64 - 1.0 {
                &self.overlay_names[(get!(overlay_scan) + 1.0) as usize]
            } else {
                ""
            },
            // SCRUB
            if self.selected_knobs == 12 { ">" } else { " " },
            self.playback[self.active_idx]
                .stream
                .scrub
                .abs()
                .max(self.playback[self.active_idx].stream.delta_sec.abs())
                .max(self.playback[self.active_idx].stream.exact_sec.abs()),
            self.selected_knobs,
            (vid_info.duration_tbu_q.0 as f64 / vid_info.duration_tbu_q.1 as f64)
                * (vid_info.timebase_q.0 as f64 / vid_info.timebase_q.1 as f64),
            hour,
            minute,
            second,
            frame,
            self.playback[self.active_idx].stream.real_ts.0 as f64,
            self.playback[self.active_idx].stream.real_ts.1 as f64,
            self.playback[self.active_idx].stream.real_ts.0 as f64
                / self.playback[self.active_idx].stream.real_ts.1 as f64
        )
    }

    pub fn asset_list(&self, _app_fps: i64) -> Vec<Asset> {
        let mut assets = vec![];

        for vid_def in &self.stream_defs {
            assets.push(vid_def.clone().into());
        }

        for (_, mix_config) in &self.mix_configs {
            assets.push(mix_config.def.clone().into());
        }

        return assets;
    }

    pub fn get_playback_specs<S: AsRef<str>>(
        &mut self,
        root: S,
        src: (i32, i32, u32, u32),
        dst: (i32, i32, u32, u32),
    ) -> Vec<RenderSpec> {
        let mut mixes = vec![];
        let mut added = HashSet::new();
        let mut stack = vec![root.as_ref().to_string()];

        let mut main_mixes = vec![];
        let mut paused = vec![];
        //recursively add all the other mixes needed to display the overlay
        while let Some(mix_name) = stack.pop() {
            if let Some(mix_config) = self.mix_configs.get(&mix_name) {
                let mut mix = mix_config.mix.clone();
                mix.no_display = true; // we will set the top mix to display later

                if let Some((fidx, _)) = self
                    .playback
                    .iter()
                    .enumerate()
                    .find(|(_, p)| p.stream.input_mix() == mix_name)
                {
                    if self.playback[fidx].stream.pause != 0
                        && self.playback[fidx].stream.delta_sec == 0.0
                        && self.playback[fidx].stream.exact_sec == 0.0
                        && self.playback[fidx].stream.scrub == 0.0
                    {
                        paused.push(fidx);
                        continue;
                    }

                    let lut = &self.lut_names[self.playback[fidx].stream.lut_selected as usize];
                    if lut != "none" {
                        mix.lut = Some(format!("{}/luts/{}.cube", self.asset_path, lut));
                    }
                }

                if let Some((fidx, _)) = self
                    .playback
                    .iter()
                    .enumerate()
                    .find(|(_, p)| p.stream.main_mix() == mix_name)
                {
                    main_mixes.push(fidx);
                    let (distort_x, distort_y) = self.distort_names
                        [self.playback[fidx].stream.distort_selected as usize]
                        .clone();
                    let (warp_x, warp_y) = self.distort_names
                        [self.playback[fidx].stream.warp_selected as usize]
                        .clone();
                    mix.inputs[2] = MixInput::Mixed(format!("{distort_x}_mix"));
                    mix.inputs[3] = MixInput::Mixed(format!("{distort_y}_mix"));
                    mix.inputs[4] = MixInput::Mixed(format!("{warp_x}_mix"));
                    mix.inputs[5] = MixInput::Mixed(format!("{warp_y}_mix"));
                }

                if let Some((fidx, _)) = self
                    .playback
                    .iter()
                    .enumerate()
                    .find(|(_, p)| p.stream.overlay_mix() == mix_name)
                {
                    let overly =
                        &self.overlay_names[self.playback[fidx].stream.overlay_selected as usize];
                    mix.inputs[1] = MixInput::Mixed(format!("{overly}_mix"));
                }
                for input in &mix.inputs {
                    match input {
                        MixInput::Mixed(m) => {
                            if !added.contains(m) {
                                added.insert(m.clone());
                                stack.push(m.clone());
                            }
                        }
                        _ => (),
                    }
                }
                mixes.push(mix);
            } else {
                eprintln!("No mix found for {mix_name}");
            }
        }

        let mut specs = mixes.into_iter().map(RenderSpec::from).collect::<Vec<_>>();
        for spec in &mut specs {
            if let RenderSpec::Mix(mix) = spec {
                if mix.name == root.as_ref().to_string() {
                    mix.target = Some(CopyEx::builder().src(src).dst(dst).build());
                    mix.no_display = false; // this is the top mix we want to display
                }
            }
        }

        for i in main_mixes.iter().chain(paused.iter()) {
            if self.initial_reset_complete[*i] == false {
                self.initial_reset_complete[*i] = true;
                specs.extend(
                    (&StreamSettings::ALL_STREAMSETTINGS_UPDATERS)
                        .iter()
                        .map(|f| f(&self.playback[*i].stream))
                        .flatten(),
                );
            }
        }
        specs
    }
}

const MFT: &str = "Midi Fighter Twister";
macro_rules! send_midi_mft {
    ($k:expr, $v:expr) => {
        RenderSpec::SendMidi(SendMidi {
            event: MidiEvent {
                device: MFT.to_string(),
                channel: 0,
                kind: MIDI_CONTROL_CHANGE,
                key: $k,
                velocity: $v,
                timestamp: 0,
            },
        })
    };
}

impl AllSettings {
    pub fn reload_encoders_for_active_idx(&self) -> Vec<RenderSpec> {
        let stream = &self.playback[self.active_idx].stream;
        vec![
            //ROW 1
            send_midi_mft!(0, (stream.threshold_pct() * 127.0) as u8),
            send_midi_mft!(1, (stream.distort_level_pct() * 127.0) as u8),
            send_midi_mft!(2, (stream.warp_level_pct() * 127.0) as u8),
            send_midi_mft!(3, (stream.sim_pct() * 127.0) as u8),
            // ROW 2
            send_midi_mft!(4, (stream.distort_edge_scan_pct() * 127.0) as u8),
            send_midi_mft!(5, (stream.distort_scan_pct() * 127.0) as u8),
            send_midi_mft!(6, (stream.warp_scan_pct() * 127.0) as u8),
            send_midi_mft!(7, (stream.lut_scan_pct() * 127.0) as u8),
            // ROW 3
            send_midi_mft!(8, (stream.scroll_h_pct() * 127.0) as u8),
            send_midi_mft!(9, (stream.scroll_v_pct() * 127.0) as u8),
            send_midi_mft!(10, (stream.dx_pct() * 127.0) as u8),
            send_midi_mft!(11, (stream.dy_pct() * 127.0) as u8),
            // ROW 4
            send_midi_mft!(12, (stream.feedback_rotation_pct() * 127.0) as u8),
            send_midi_mft!(13, (stream.scanlines_scan_pct() * 127.0) as u8),
            send_midi_mft!(14, (stream.blend_scan_pct() * 127.0) as u8),
            send_midi_mft!(15, (stream.overlay_scan_pct() * 127.0) as u8),
            // ROW 5
            send_midi_mft!(16, (stream.rr_pct() * 127.0) as u8),
            send_midi_mft!(17, (stream.rg_pct() * 127.0) as u8),
            send_midi_mft!(18, (stream.rb_pct() * 127.0) as u8),
            send_midi_mft!(19, (stream.ra_pct() * 127.0) as u8),
            // ROW 6
            send_midi_mft!(20, (stream.gr_pct() * 127.0) as u8),
            send_midi_mft!(21, (stream.gg_pct() * 127.0) as u8),
            send_midi_mft!(22, (stream.gb_pct() * 127.0) as u8),
            send_midi_mft!(23, (stream.ga_pct() * 127.0) as u8),
            // ROW 7
            send_midi_mft!(24, (stream.br_pct() * 127.0) as u8),
            send_midi_mft!(25, (stream.bg_pct() * 127.0) as u8),
            send_midi_mft!(26, (stream.bb_pct() * 127.0) as u8),
            send_midi_mft!(27, (stream.ba_pct() * 127.0) as u8),
            // ROW 8
            send_midi_mft!(28, (stream.ar_pct() * 127.0) as u8),
            send_midi_mft!(29, (stream.ag_pct() * 127.0) as u8),
            send_midi_mft!(30, (stream.ab_pct() * 127.0) as u8),
            send_midi_mft!(31, (stream.aa_pct() * 127.0) as u8),
            // ROW 9
            send_midi_mft!(32, (stream.rh_pct() * 127.0) as u8),
            send_midi_mft!(33, (stream.rv_pct() * 127.0) as u8),
            send_midi_mft!(34, (stream.skew_x0_pct() * 127.0) as u8),
            send_midi_mft!(35, (stream.skew_y0_pct() * 127.0) as u8),
            //ROW 10
            send_midi_mft!(36, (stream.gh_pct() * 127.0) as u8),
            send_midi_mft!(37, (stream.gv_pct() * 127.0) as u8),
            send_midi_mft!(38, (stream.skew_x1_pct() * 127.0) as u8),
            send_midi_mft!(39, (stream.skew_y1_pct() * 127.0) as u8),
            // ROW 11
            send_midi_mft!(40, (stream.bh_pct() * 127.0) as u8),
            send_midi_mft!(41, (stream.bv_pct() * 127.0) as u8),
            send_midi_mft!(42, (stream.skew_x2_pct() * 127.0) as u8),
            send_midi_mft!(43, (stream.skew_y2_pct() * 127.0) as u8),
            // ROW 12
            send_midi_mft!(44, (stream.ah_pct() * 127.0) as u8),
            send_midi_mft!(45, (stream.av_pct() * 127.0) as u8),
            send_midi_mft!(46, (stream.skew_x3_pct() * 127.0) as u8),
            send_midi_mft!(47, (stream.skew_y3_pct() * 127.0) as u8),
        ]
    }
    pub fn video_fight_cb(&mut self, event: &MidiEvent) {
        let idx = self.active_idx;
        let stream = &mut self.playback[idx].stream;

        if event.device != MFT || event.kind != MIDI_CONTROL_CHANGE {
            return;
        }

        match (event.channel, event.key, event.velocity) {
            //ROW 1
            (0, 0, v) => stream.scale_threshold(v as f64 / 127.0),
            (1, 0, 127) => stream.set_threshold(0.0),

            (0, 1, v) => stream.scale_distort_level(v as f64 / 127.0),
            (1, 1, 127) => stream.set_distort_level(0.2),

            (0, 2, v) => stream.scale_warp_level(v as f64 / 127.0),
            (1, 2, 127) => stream.set_warp_level(0.2),

            (0, 3, v) => stream.scale_sim(v as f64 / 127.0),
            (1, 3, 127) => stream.toggle_video_key_enable(),

            // ROW 2
            (0, 4, v) => stream.scale_distort_edge_scan(v as f64 / 127.0),
            (1, 4, 127) => stream.set_distort_edge_selected(stream.distort_edge_scan()),

            (0, 5, v) => stream.scale_distort_scan(v as f64 / 127.0),
            (1, 5, 127) => stream.set_distort_selected(stream.distort_scan()),

            (0, 6, v) => stream.scale_warp_scan(v as f64 / 127.0),
            (1, 6, 127) => stream.set_warp_selected(stream.warp_scan()),

            (0, 7, v) => stream.scale_lut_scan(v as f64 / 127.0),
            (1, 7, 127) => stream.set_lut_selected(stream.lut_scan()),

            // ROW 3
            (0, 8, v) => stream.scale_scroll_h(v as f64 / 127.0),
            (1, 8, 127) => stream.set_scroll_h(0.0),

            (0, 9, v) => stream.scale_scroll_v(v as f64 / 127.0),
            (1, 9, 127) => stream.set_scroll_v(0.0),

            (0, 10, v) => stream.scale_dx(v as f64 / 127.0),
            (1, 10, 127) => stream.set_dx(0.0),

            (0, 11, v) => stream.scale_dy(v as f64 / 127.0),
            (1, 11, 127) => stream.set_dy(0.0),

            // ROW 4
            (0, 12, v) => stream.scale_feedback_rotation(v as f64 / 127.0),
            (1, 12, 127) => stream.set_feedback_rotation(0.0),

            (0, 13, v) => stream.scale_scanlines_scan(v as f64 / 127.0),
            (1, 13, 127) => stream.set_scanlines_selected(stream.scanlines_scan()),

            (0, 14, v) => stream.scale_blend_scan(v as f64 / 127.0),
            (1, 14, 127) => stream.set_blend_selected(stream.blend_scan()),

            (0, 15, v) => stream.scale_overlay_scan(v as f64 / 127.0),
            (1, 15, 127) => stream.set_overlay_selected(stream.overlay_scan()),

            // ROW 5
            (0, 16, v) => stream.scale_rr(v as f64 / 127.0),
            (1, 16, 127) => stream.set_rr(1.0),

            (0, 17, v) => stream.scale_rg(v as f64 / 127.0),
            (1, 17, 127) => stream.set_rg(0.0),

            (0, 18, v) => stream.scale_rb(v as f64 / 127.0),
            (1, 18, 127) => stream.set_rb(0.0),

            (0, 19, v) => stream.scale_ra(v as f64 / 127.0),
            (1, 19, 127) => stream.set_ra(0.0),

            // ROW 6
            (0, 20, v) => stream.scale_gr(v as f64 / 127.0),
            (1, 20, 127) => stream.set_gr(0.0),

            (0, 21, v) => stream.scale_gg(v as f64 / 127.0),
            (1, 21, 127) => stream.set_gg(1.0),

            (0, 22, v) => stream.scale_gb(v as f64 / 127.0),
            (1, 22, 127) => stream.set_gb(0.0),

            (0, 23, v) => stream.scale_ga(v as f64 / 127.0),
            (1, 23, 127) => stream.set_ga(0.0),

            // ROW 7
            (0, 24, v) => stream.scale_br(v as f64 / 127.0),
            (1, 24, 127) => stream.set_br(0.0),

            (0, 25, v) => stream.scale_bg(v as f64 / 127.0),
            (1, 25, 127) => stream.set_bg(0.0),

            (0, 26, v) => stream.scale_bb(v as f64 / 127.0),
            (1, 26, 127) => stream.set_bb(1.0),

            (0, 27, v) => stream.scale_ba(v as f64 / 127.0),
            (1, 27, 127) => stream.set_ba(0.0),

            // ROW 8
            (0, 28, v) => stream.scale_ar(v as f64 / 127.0),
            (1, 28, 127) => stream.set_ar(0.0),

            (0, 29, v) => stream.scale_ag(v as f64 / 127.0),
            (1, 29, 127) => stream.set_ag(0.0),

            (0, 30, v) => stream.scale_ab(v as f64 / 127.0),
            (1, 30, 127) => stream.set_ab(0.0),

            (0, 31, v) => stream.scale_aa(v as f64 / 127.0),
            (1, 31, 127) => stream.set_aa(1.0),

            // ROW 9
            (0, 32, v) => stream.scale_rh(v as f64 / 127.0),
            (1, 32, 127) => stream.set_rh(0.0),

            (0, 33, v) => stream.scale_rv(v as f64 / 127.0),
            (1, 33, 127) => stream.set_rv(0.0),

            (0, 34, v) => stream.scale_skew_x0(v as f64 / 127.0),
            (1, 34, 127) => stream.set_skew_x0(0.0),

            (0, 35, v) => stream.scale_skew_y0(v as f64 / 127.0),
            (1, 35, 127) => stream.set_skew_y0(0.0),

            //ROW 10
            (0, 36, v) => stream.scale_gh(v as f64 / 127.0),
            (1, 36, 127) => stream.set_gh(0.0),

            (0, 37, v) => stream.scale_gv(v as f64 / 127.0),
            (1, 37, 127) => stream.set_gv(0.0),

            (0, 38, v) => stream.scale_skew_x1(v as f64 / 127.0),
            (1, 38, 127) => stream.set_skew_x1(1.0),

            (0, 39, v) => stream.scale_skew_y1(v as f64 / 127.0),
            (1, 39, 127) => stream.set_skew_y1(0.0),

            // ROW 11
            (0, 40, v) => stream.scale_bh(v as f64 / 127.0),
            (1, 40, 127) => stream.set_bh(0.0),

            (0, 41, v) => stream.scale_bv(v as f64 / 127.0),
            (1, 41, 127) => stream.set_bv(0.0),

            (0, 42, v) => stream.scale_skew_x2(v as f64 / 127.0),
            (1, 42, 127) => stream.set_skew_x2(0.0),

            (0, 43, v) => stream.scale_skew_y2(v as f64 / 127.0),
            (1, 43, 127) => stream.set_skew_y2(1.0),

            // ROW 12
            (0, 44, v) => stream.scale_ah(v as f64 / 127.0),
            (1, 44, 127) => stream.set_ah(0.0),

            (0, 45, v) => stream.scale_av(v as f64 / 127.0),
            (1, 45, 127) => stream.set_av(0.0),

            (0, 46, v) => stream.scale_skew_x3(v as f64 / 127.0),
            (1, 46, 127) => stream.set_skew_x3(1.0),

            (0, 47, v) => stream.scale_skew_y3(v as f64 / 127.0),
            (1, 47, 127) => stream.set_skew_y3(1.0),
            _ => (),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[repr(C)]
pub struct StreamIdent {
    pub name: String,
    pub first_video: String,
    pub input_mix: String,
    pub main_mix: String,
    pub feedback_mix: String,
    pub overlay_mix: String,
}

#[derive(Debug, Clone, Adjustable, Serialize, Deserialize)]
#[repr(C)]
pub struct StreamSettings {
    pub ident: StreamIdent,
    // These are reserved because we catch these inputs and use them to change the
    // All settings config
    #[adjustable(k = B, idx = 1)]
    reserved1: f64,
    #[adjustable(k = B, idx = 10)]
    reserved2: f64,
    #[adjustable(k = CB, idx = 10)]
    reserved3: f64,

    #[adjustable(k = R, idx = 10)]
    reserved4: f64,
    #[adjustable(k = CR, idx = 10)]
    reserved5: f64,

    pub real_ts: (i32, i32),
    pub continuous_ts: (i32, i32),
    // FLASH
    #[adjustable(kind = toggle, k = CB, idx = 1, command_simple = (self.main_mix(), "flash_enable", Unsigned))]
    flash_enable: u8,
    // LOOP CONTROL
    #[adjustable(k = CL, idx = 1, kind = toggle)]
    tween: u8,
    // USER
    #[adjustable(k = L, idx = 2, min = -101.0, max = 101.0, step = 1.0, setter=set_usr_var, command_simple = (self.input_mix(), "usr_var", Integer))]
    usr_var: f64,
    #[adjustable(k = CL, idx = 2, kind = toggle, command_simple = (self.input_mix(), "usr_toggle", Integer))]
    usr_toggle: u8,
    //VIDEO MIXER
    #[adjustable(k = R, idx = 2, min = 0.0, max = 15, do_not_record = true)]
    color_mix_selected: f64,
    #[adjustable(k = B, idx = 2, min = -2.0, max = 2.0, step = 0.01, ty = f64, getter = color_mix, setter = set_color_mix)]
    color_mix: (),
    #[adjustable(min =  -2.0, max = 2.0, step = 0.01, command_simple = (self.main_mix(), "mix_rr", Float), tween = true)]
    rr: f64,
    #[adjustable(min =  -2.0, max = 2.0, step = 0.01, command_simple = (self.main_mix(), "mix_rg", Float), tween = true)]
    rg: f64,
    #[adjustable(min =  -2.0, max = 2.0, step = 0.01, command_simple = (self.main_mix(), "mixrb", Float), tween = true)]
    rb: f64,
    #[adjustable(min =  -2.0, max = 2.0, step = 0.01, command_simple = (self.main_mix(), "mix_ra", Float), tween = true)]
    ra: f64,
    #[adjustable(min =  -2.0, max = 2.0, step = 0.01, command_simple = (self.main_mix(), "mix_gr", Float), tween = true)]
    gr: f64,
    #[adjustable(min =  -2.0, max = 2.0, step = 0.01, command_simple = (self.main_mix(), "mix_gg", Float), tween = true)]
    gg: f64,
    #[adjustable(min =  -2.0, max = 2.0, step = 0.01, command_simple = (self.main_mix(), "mix_gb", Float), tween = true)]
    gb: f64,
    #[adjustable(min =  -2.0, max = 2.0, step = 0.01, command_simple = (self.main_mix(), "mix_ga", Float), tween = true)]
    ga: f64,
    #[adjustable(min =  -2.0, max = 2.0, step = 0.01, command_simple = (self.main_mix(), "mix_br", Float), tween = true)]
    br: f64,
    #[adjustable(min =  -2.0, max = 2.0, step = 0.01, command_simple = (self.main_mix(), "mix_bg", Float), tween = true)]
    bg: f64,
    #[adjustable(min =  -2.0, max = 2.0, step = 0.01, command_simple = (self.main_mix(), "mix_bb", Float), tween = true)]
    bb: f64,
    #[adjustable(min =  -2.0, max = 2.0, step = 0.01, command_simple = (self.main_mix(), "mix_ba", Float), tween = true)]
    ba: f64,
    #[adjustable(min =  -2.0, max = 2.0, step = 0.01, command_simple = (self.main_mix(), "mix_ar", Float), tween = true)]
    ar: f64,
    #[adjustable(min =  -2.0, max = 2.0, step = 0.01, command_simple = (self.main_mix(), "mix_ag", Float), tween = true)]
    ag: f64,
    #[adjustable(min =  -2.0, max = 2.0, step = 0.01, command_simple = (self.main_mix(), "mix_ab", Float), tween = true)]
    ab: f64,
    #[adjustable(min =  -2.0, max = 2.0, step = 0.01, command_simple = (self.main_mix(), "mix_aa", Float), tween = true)]
    aa: f64,

    #[adjustable(k = L, idx = 3, min = -2.0, max = 2.0, step = 0.0087, command_simple = (self.main_mix(), "boost", Float))]
    boost: f64,
    #[adjustable(k = R, idx = 3, min = 0.0, max = 1.0, step = 0.01, command_simple = (self.main_mix(), "thresh", Float))]
    threshold: f64,
    #[adjustable(tween = true, min = 0.0, max = 1.0, command_simple = (self.main_mix(), "distort_level", Float))]
    distort_level: f64,
    #[adjustable(tween = true,  min = 0.0, max = 1.0, command_simple = (self.main_mix(), "warp_level", Float))]
    warp_level: f64,
    #[adjustable(k = CB, idx = 3, kind = toggle, do_not_record = true)]
    distort_warp_select: u8,
    #[adjustable(k = B, idx = 3, min = 0.0, max = 1.0, step=0.005, ty = f64, getter = distort_or_warp_level, setter = set_distort_or_warp_level)]
    distort_or_warp_level: (),
    #[adjustable(k = CL, idx = 4, setter = set_skew_selected, do_not_record = true)]
    skew_selected: f64,
    #[adjustable(k = L, idx = 4, min = -1.0, max = 1.0, step = 0.001, ty = f64, getter = skew_dx, setter = set_skew_dx)]
    skew_dx: (),
    #[adjustable(k = R, idx = 4, min = -1.0, max = 1.0, step = 0.001, ty = f64, getter = skew_dy, setter = set_skew_dy)]
    skew_dy: (),
    #[adjustable(k = B, idx = 4, kind = custom, ty = ((f64, f64), (f64, f64) (f64, f64), (f64, f64)), getter = skew_all, setter = set_skew_all)]
    skew_all: (),
    #[adjustable(min = -2.5, max = 2.5, command_fn = skew_update, tween = true)]
    skew_x0: f64,
    #[adjustable(min = -2.5, max = 2.5, command_fn = skew_update, tween = true)]
    skew_y0: f64,
    #[adjustable(min = -2.5, max = 2.5, command_fn = skew_update, tween = true)]
    skew_x1: f64,
    #[adjustable(min = -2.5, max = 2.5, command_fn = skew_update, tween = true)]
    skew_y1: f64,
    #[adjustable(min = -2.5, max = 2.5,   command_fn = skew_update, tween = true)]
    skew_x2: f64,
    #[adjustable(min = -2.5, max = 2.5, command_fn = skew_update, tween = true)]
    skew_y2: f64,
    #[adjustable(min = -2.5, max = 2.5, command_fn = skew_update, tween = true)]
    skew_x3: f64,
    #[adjustable(min = -2.5, max = 2.5, command_fn = skew_update, tween = true)]
    skew_y3: f64,

    // VIDEO KEY
    #[adjustable(k = B, idx = 5, min = 0.001, max = 1.0, step = 0.001, command_simple = (self.main_mix(), "color_key_sim", Float))]
    sim: f64,
    #[adjustable(k = L, idx = 5, min = 0.0, max = 1.0, step = 0.001, command_simple = (self.main_mix(), "color_key_blend", Float))]
    blend: f64,
    #[adjustable(kind = toggle, k = CB, idx = 5, command_simple = (self.main_mix(), "color_key_enable", Unsigned))]
    video_key_enable: u8,

    // VIDEO KEY COLOR
    #[adjustable(k = R, idx = 5, min = 0.0, max = (AllSettings::colors().len() - 1), step = 1, do_not_record = true)]
    video_key_color_scan: f64,
    #[adjustable(kind = assign, k = CR, idx = 5, from = self.video_key_color_scan, command_fn = video_key_color_update)]
    video_key_color_selected: f64,

    // SCROLL
    #[adjustable(k = L, idx = 6, min = -1.0, max = 1.0, step = 0.0001,)]
    scroll_h: f64,
    #[adjustable(k = R, idx = 6, min = -1.0, max = 1.0, step = 0.0001,)]
    scroll_v: f64,
    #[adjustable(command_simple = (self.main_mix(), "scrolled_h", Float), do_not_record = true)]
    scrolled_h: f64,
    #[adjustable(command_simple = (self.main_mix(), "scrolled_v", Float), do_not_record = true)]
    scrolled_v: f64,

    // COLOR SHIFT
    #[adjustable(k = R, idx = 7, min = 0.0, max = 7.0, step = 1.0, do_not_record = true)]
    shift_select: f64,
    #[adjustable(k = B, idx = 7, min = -1.0, max = 1.0, step = 0.001, ty = f64, setter = set_color_shift, getter = color_shift)]
    color_shift: (),
    #[adjustable(tween = true, min = -0.5, max = 0.5, command_simple = (self.main_mix(), "shift_rh", Float))]
    rh: f64,
    #[adjustable(tween = true, min = -0.5, max = 0.5, command_simple = (self.main_mix(), "shift_rv", Float))]
    rv: f64,
    #[adjustable(tween = true, min = -0.5, max = 0.5, command_simple = (self.main_mix(), "shift_gh", Float))]
    gh: f64,
    #[adjustable(tween = true, min = -0.5, max = 0.5, command_simple = (self.main_mix(), "shift_gv", Float))]
    gv: f64,
    #[adjustable(tween = true, min = -0.5, max = 0.5, command_simple = (self.main_mix(), "shift_bh", Float))]
    bh: f64,
    #[adjustable(tween = true, min = -0.5, max = 0.5, command_simple = (self.main_mix(), "shift_bv", Float))]
    bv: f64,
    #[adjustable(tween = true,min = -0.5, max = 0.5,  command_simple = (self.main_mix(), "shift_ah", Float))]
    ah: f64,
    #[adjustable(tween = true, min = -0.5, max = 0.5, command_simple = (self.main_mix(), "shift_av", Float))]
    av: f64,
    #[adjustable(kind = toggle, k = CR, idx = 7)]
    shift_enable: u8,

    // COMPOSITE
    #[adjustable(k = L, idx = 7, min = 1.0, max = 1800.0)]
    composite_freq: f64,

    // NEGATE
    #[adjustable(kind = toggle, k = CL, idx = 7, command_simple = (self.main_mix(), "negate", Unsigned))]
    negate: u8,

    //DISTORT
    #[adjustable(k = L, idx = 8, min = -1.0, max = 1.0, step = 0.001, tween = true, command_simple = (self.main_mix(), "distort_dx", Float))]
    dx: f64,
    #[adjustable(k = R, idx = 8, min = -1.0, max = 1.0, step = 0.001, tween = true,  command_simple = (self.main_mix(), "distort_dy", Float))]
    dy: f64,

    #[adjustable(k = B, idx = 8, min = - 2.0 * std::f64::consts::PI, max = 2.0 * std::f64::consts::PI,step = 2.0 * std::f64::consts::PI/400.0, tween = true, setter = set_feedback_rotation, command_simple = (self.main_mix(), "feedback_rotation", Float))]
    feedback_rotation: f64,

    #[adjustable(k = B, idx = 9, min = 0.0, max = (AllSettings::distort_names().len() - 1), step = 1.0, do_not_record = true)]
    distort_scan: f64,
    #[adjustable(kind = assign, k = CB, idx = 9, from = self.distort_scan)]
    distort_selected: f64,
    #[adjustable(k = L, idx = 9, min = 0.0, max = (AllSettings::distort_edge_types().len() - 1), step = 1.0, do_not_record = true)]
    distort_edge_scan: f64,
    #[adjustable(kind = assign, k = CL, idx = 9, from = self.distort_edge_scan, command_simple = (self.main_mix(), "distort_edge", Unsigned))]
    distort_edge_selected: f64,

    // WARP
    #[adjustable(k = R, idx = 9, min = 0.0, max = (AllSettings::distort_names().len() - 1), step = 1.0, do_not_record = true)]
    warp_scan: f64,
    #[adjustable(kind = assign, k = CR, idx = 9, from = self.warp_scan)]
    warp_selected: f64,

    //LUT
    #[adjustable(k = L, idx = 10, min = 0.0, max = (AllSettings::lut_names().len() - 1), step = 1.0, do_not_record = true)]
    lut_scan: f64,
    #[adjustable(kind = assign, k = CL, idx = 10, from = self.lut_scan)]
    lut_selected: f64,

    // FOREGROUND
    #[adjustable(k = R, idx = 11, min = 0.0, max = (AllSettings::blend_modes().len() - 1), step = 1.0, do_not_record = true)]
    blend_scan: f64,
    #[adjustable(kind = assign, k = CR, idx = 11, from = self.blend_scan, command_simple = (self.overlay_mix(), "overlay_kind", Unsigned))]
    blend_selected: f64,

    #[adjustable(k = L, idx = 11, min = 0.0, max = (AllSettings::blend_modes().len() - 1), step = 1.0, do_not_record = true)]
    scanlines_scan: f64,
    #[adjustable(kind = assign, k = CL, idx = 11, from = self.scanlines_scan, command_simple = (self.overlay_mix(), "scanline_kind", Unsigned))]
    scanlines_selected: f64,

    #[adjustable(k = B, idx = 11, min = 0.0, max = (AllSettings::overlay_vids().len() - 1), step = 1.0, do_not_record = true)]
    overlay_scan: f64,
    #[adjustable(kind = assign, k = CB, idx = 11, from = self.overlay_scan)]
    overlay_selected: f64,

    // DELTA FRAMES
    #[adjustable(command_fn = delta_sec_update)]
    delta_sec: f64,
    #[adjustable(k = B, idx = 12, step = 0.05, command_fn = scrub_update)]
    scrub: f64,
    mark: Option<f64>,
    #[adjustable(command_fn = exact_sec_update)]
    exact_sec: f64,
    #[adjustable(kind = toggle)]
    pause: u8,

    // FEEDBACK
    #[adjustable(k = R, idx = 13, min = 0.0, max = 1.0, step = 1.0, do_not_record = true)]
    feedback_style_scan: f64,
    #[adjustable(kind = assign, k = CR, idx = 13,  from = self.feedback_style_scan)]
    feedback_style_selected: f64,
}

impl StreamSettings {
    pub fn new<S: AsRef<str>>(
        name: S,
        first_video: S,
        input_mix: S,
        main_mix: S,
        feedback_mix: S,
        overlay_mix: S,
    ) -> Self {
        Self {
            ident: StreamIdent {
                name: name.as_ref().to_string(),
                first_video: first_video.as_ref().to_string(),
                input_mix: input_mix.as_ref().to_string(),
                main_mix: main_mix.as_ref().to_string(),
                feedback_mix: feedback_mix.as_ref().to_string(),
                overlay_mix: overlay_mix.as_ref().to_string(),
            },
            real_ts: (0, 1),
            continuous_ts: (0, 1),
            flash_enable: 0,
            tween: 0,
            usr_var: 0.0,
            usr_toggle: 0,
            color_mix: (),
            color_mix_selected: 0.0,
            rr: 1.0,
            rg: 0.0,
            rb: 0.0,
            gr: 0.0,
            ra: 0.0,
            gg: 1.0,
            gb: 0.0,
            ga: 0.0,
            br: 0.0,
            bg: 0.0,
            bb: 1.0,
            ba: 0.0,
            ar: 0.0,
            ag: 0.0,
            ab: 0.0,
            aa: 1.0,
            skew_selected: 0.0,
            skew_dx: (),
            skew_dy: (),
            skew_all: (),
            skew_x0: 0.0,
            skew_y0: 0.0,
            skew_x1: 1.0,
            skew_y1: 0.0,
            skew_x2: 0.0,
            skew_y2: 1.0,
            skew_x3: 1.0,
            skew_y3: 1.0,
            sim: 0.001,
            blend: 0.0,
            video_key_enable: 0,
            video_key_color_scan: 0.0,
            video_key_color_selected: 0.0,
            scroll_h: 0.0,
            scroll_v: 0.0,
            scrolled_h: 0.0,
            scrolled_v: 0.0,
            shift_select: 0.0,
            color_shift: (),
            rh: 0.0,
            rv: 0.0,
            gh: 0.0,
            gv: 0.0,
            bh: 0.0,
            bv: 0.0,
            ah: 0.0,
            av: 0.0,
            shift_enable: 0,
            composite_freq: 1.0,
            negate: 0,
            boost: 0.0,
            distort_level: 0.2,
            warp_level: 0.2,
            distort_warp_select: 0,
            distort_or_warp_level: (),
            dx: 0.0,
            dy: 0.0,
            threshold: 0.0,
            blend_scan: 0.0,
            blend_selected: 0.0,
            scanlines_scan: 0.0,
            scanlines_selected: 0.0,
            overlay_selected: 0.0,
            overlay_scan: 0.0,
            distort_scan: 0.0,
            distort_selected: 0.0,
            distort_edge_scan: 0.0,
            distort_edge_selected: 0.0,
            warp_scan: 0.0,
            warp_selected: 0.0,
            lut_selected: 0.0,
            lut_scan: 0.0,
            feedback_rotation: 0.0,
            feedback_style_selected: 0.0,
            feedback_style_scan: 0.0,
            delta_sec: 0.0,
            scrub: 0.0,
            mark: None,
            exact_sec: 0.0,
            pause: 0,
            reserved1: 0.0,
            reserved2: 0.0,
            reserved3: 0.0,
            reserved4: 0.0,
            reserved5: 0.0,
        }
    }

    // Override accessors
    fn set_color_shift(&mut self, v: f64) {
        match self.shift_select as u8 {
            0 => {
                self.rh = v;
            }
            1 => {
                self.rv = v;
            }
            2 => {
                self.gh = v;
            }
            3 => {
                self.gv = v;
            }
            4 => {
                self.bh = v;
            }
            5 => {
                self.bv = v;
            }
            6 => {
                self.ah = v;
            }
            7 => {
                self.av = v;
            }
            _ => (),
        }
    }

    fn color_shift(&self) -> f64 {
        match self.shift_select as u8 {
            0 => self.rh,
            1 => self.rv,
            2 => self.gh,
            3 => self.gv,
            4 => self.bh,
            5 => self.bv,
            6 => self.ah,
            7 => self.av,
            _ => panic!("Color shift out of bounds"),
        }
    }

    pub fn set_color_mix(&mut self, v: f64) {
        match self.color_mix_selected as u8 {
            0 => self.rr = v,
            1 => self.rg = v,
            2 => self.rb = v,
            3 => self.ra = v,
            4 => self.gr = v,
            5 => self.gg = v,
            6 => self.gb = v,
            7 => self.ga = v,
            8 => self.br = v,
            9 => self.bg = v,
            10 => self.bb = v,
            11 => self.ba = v,
            12 => self.ar = v,
            13 => self.ag = v,
            14 => self.ab = v,
            15 => self.aa = v,
            _ => panic!("Color mix out of bounds"),
        }
    }

    pub fn color_mix(&self) -> f64 {
        match self.color_mix_selected as u8 {
            0 => self.rr,
            1 => self.rg,
            2 => self.rb,
            3 => self.ra,
            4 => self.gr,
            5 => self.gg,
            6 => self.gb,
            7 => self.ga,
            8 => self.br,
            9 => self.bg,
            10 => self.bb,
            11 => self.ba,
            12 => self.ar,
            13 => self.ag,
            14 => self.ab,
            15 => self.aa,
            _ => panic!("Color mix out of bounds"),
        }
    }

    pub fn set_skew_selected(&mut self, v: f64) {
        self.skew_selected = (v as u8).rem_euclid(4) as f64;
    }

    pub fn skew_dx(&self) -> f64 {
        match self.skew_selected as u8 {
            0 => self.skew_x0,
            1 => self.skew_x1,
            2 => self.skew_x2,
            3 => self.skew_x3,
            _ => panic!("skew dx selected out of bounds"),
        }
    }

    pub fn set_skew_dx(&mut self, v: f64) {
        match self.skew_selected as u8 {
            0 => self.skew_x0 = v,
            1 => self.skew_x1 = v,
            2 => self.skew_x2 = v,
            3 => self.skew_x3 = v,
            _ => panic!("skew dx selected out of bounds"),
        }
    }

    pub fn skew_dy(&self) -> f64 {
        match self.skew_selected as u8 {
            0 => self.skew_y0,
            1 => self.skew_y1,
            2 => self.skew_y2,
            3 => self.skew_y3,
            _ => panic!("skew dy selected out of bounds"),
        }
    }

    pub fn set_skew_dy(&mut self, v: f64) {
        match self.skew_selected as u8 {
            0 => self.skew_y0 = v,
            1 => self.skew_y1 = v,
            2 => self.skew_y2 = v,
            3 => self.skew_y3 = v,
            _ => panic!("skew dx selected out of bounds"),
        }
    }

    pub fn set_skew_all(&mut self, v: ((f64, f64), (f64, f64), (f64, f64), (f64, f64))) {
        self.skew_x0 = v.0 .0;
        self.skew_y0 = v.0 .1;
        self.skew_x1 = v.1 .0;
        self.skew_y1 = v.1 .1;
        self.skew_x2 = v.2 .0;
        self.skew_y2 = v.2 .1;
        self.skew_x3 = v.3 .0;
        self.skew_y3 = v.3 .1;
    }

    pub fn skew_all(&self) -> ((f64, f64), (f64, f64), (f64, f64), (f64, f64)) {
        (
            (self.skew_x0, self.skew_y0),
            (self.skew_x1, self.skew_y1),
            (self.skew_x2, self.skew_y2),
            (self.skew_x3, self.skew_y3),
        )
    }

    pub fn adjust_skew_all(&mut self, inc: f64) {
        let step = inc * 0.001;
        self.set_skew_all((
            (
                (self.skew_x0 + step).clamp(-2.5, 2.5),
                (self.skew_y0 + step).clamp(-2.5, 2.5),
            ),
            (
                (self.skew_x1 - step).clamp(-2.5, 2.5),
                (self.skew_y1 + step).clamp(-2.5, 2.5),
            ),
            (
                (self.skew_x2 + step).clamp(-2.5, 2.5),
                (self.skew_y2 - step).clamp(-2.5, 2.5),
            ),
            (
                (self.skew_x3 - step).clamp(-2.5, 2.5),
                (self.skew_y3 - step).clamp(-2.5, 2.5),
            ),
        ));
    }

    pub fn set_usr_var(&mut self, v: f64) {
        self.usr_var = (v + 100.0).rem_euclid(201.0) - 100.0;
    }

    pub fn set_feedback_rotation(&mut self, v: f64) {
        self.feedback_rotation = (v + 2.0 * PI).rem_euclid(4.0 * PI) - 2.0 * PI;
    }

    fn distort_or_warp_level(&self) -> f64 {
        if self.distort_warp_select == 0 {
            self.distort_level
        } else {
            self.warp_level
        }
    }

    fn set_distort_or_warp_level(&mut self, v: f64) {
        if self.distort_warp_select == 0 {
            self.distort_level = v
        } else {
            self.warp_level = v
        }
    }

    fn video_key_color_update(&self) -> Vec<RenderSpec> {
        eprintln!("Unimplemented video key color update");
        vec![]
    }

    fn skew_update(&self) -> Vec<RenderSpec> {
        vec![SendCmd::builder()
            .mix(self.main_mix())
            .name("new_corners")
            .value(SendValue::Vector(vec![
                self.skew_x0 as f32,
                self.skew_y0 as f32,
                self.skew_x1 as f32,
                self.skew_y1 as f32,
                self.skew_x2 as f32,
                self.skew_y2 as f32,
                self.skew_x3 as f32,
                self.skew_y3 as f32,
            ]))
            .build()
            .into()]
    }

    fn delta_sec_update(&self) -> Vec<RenderSpec> {
        vec![seek!(self.first_video() => self.delta_sec, false)]
    }

    fn scrub_update(&self) -> Vec<RenderSpec> {
        if self.scrub >= 0.0 {
            vec![seek!(self.first_video() => self.scrub, false)]
        } else {
            vec![seek!(self.first_video() => self.scrub - 0.1, false)]
        }
    }

    fn exact_sec_update(&self) -> Vec<RenderSpec> {
        vec![seek!(self.first_video() => self.exact_sec, true)]
    }

    pub fn first_video(&self) -> String {
        self.ident.first_video.clone()
    }
    pub fn input_mix(&self) -> String {
        self.ident.input_mix.clone()
    }
    pub fn main_mix(&self) -> String {
        self.ident.main_mix.clone()
    }
    pub fn feedback_mix(&self) -> String {
        self.ident.feedback_mix.clone()
    }
    pub fn overlay_mix(&self) -> String {
        self.ident.overlay_mix.clone()
    }
    pub fn reset(&mut self) {
        *self = Self::new(
            self.ident.name.clone(),
            self.first_video(),
            self.input_mix(),
            self.main_mix(),
            self.feedback_mix(),
            self.overlay_mix(),
        );
    }
}
