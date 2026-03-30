use sdlrig::gfxinfo::MidiEvent;
#[allow(unused_imports)]
use sdlrig::gfxinfo::{MIDI_CONTROL_CHANGE, MIDI_NOTE_OFF, MIDI_NOTE_ON};
use sdlrig::renderspec::{CopyEx, Mix, MixInput, Reset, SendCmd, SendMidi, SendValue};
use sdlrig::{
    gfxinfo::{Asset, GfxEvent, KeyCode, KeyEvent, Knob, Vid, VidMixer},
    renderspec::RenderSpec,
};

use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};
use std::{error::Error, i64, io::Write};

use crate::shaderlookup::include_files;
use crate::streamsettings::{self, StreamIdent, StreamSettings, StreamSettingsField};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct LoopEvent {
    pub frame: i64,
    pub diffs: Vec<(StreamSettingsField, f64)>,
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
    pub saved: [Vec<(StreamSettingsField, f64)>; 10],
    pub original: Vec<(StreamSettingsField, f64)>,
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

impl MixConfig {
    pub fn add_shader<T: AsRef<str>>(&mut self, shader: T) {
        let h = shader.as_ref();
        let o = self.def.shader.as_deref().unwrap_or_default();
        self.def.shader.replace([o, h].join("\n"));
    }
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
                    .shader(include_files(include_str!("glsl/main_pipeline.glsl")))
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
                    .shader(include_files(include_str!("glsl/overlay.glsl")))
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
    pub warp_names: Vec<(String, String)>,
    pub distort_edge_types: Vec<String>,
    pub overlay_names: Vec<String>,
    pub lut_names: Vec<String>,
    pub blend_modes: Vec<String>,
    pub feedback_modes: Vec<String>,
    pub colors: Vec<(String, String)>,
    pub asset_path: String,
    pub active_idx: usize,
    pub display_idx: usize,
    pub scan_idx: usize,
    pub midi_target_locked: Option<usize>,
    pub clipboard: StreamSettings,
    pub selected_knobs: usize,
    pub playback: Vec<PlaybackSettings>,
    pub initial_reset_complete: Vec<bool>,
    pub logs: Vec<String>,
    pub last_line: String,
}

static LAST_FRAME: Mutex<i64> = Mutex::new(0);
pub static TARGET_SIZE_W: u32 = 640;
pub static TARGET_SIZE_H: u32 = 480;

impl AllSettings {
    pub fn colors() -> &'static [(&'static str, &'static str)] {
        &[("black", "0x000000")]
    }

    pub fn distort_warp_base_names() -> &'static [(&'static str, &'static str)] {
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
                    .resolution((640, 480))
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

    pub fn feedback_modes() -> &'static [&'static str] {
        &["basic", "jam", "math", "xor", "sea", "alien"]
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
        let asset_path = asset_path.as_ref().to_string();
        let playback_names = playback_names
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<_>>();
        let mut stream_defs = stream_defs.into_iter().collect::<Vec<_>>();
        let mut mix_configs = mix_configs
            .into_iter()
            .map(|mc| (mc.def.name.clone(), mc))
            .collect::<HashMap<_, _>>();

        let distort_warp_base_names = Self::distort_warp_base_names()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<Vec<_>>();

        let mut distort_names = vec![];
        let mut warp_names = vec![];

        for name in distort_warp_base_names {
            let (distort_dx, distort_dy, warp_dx, warp_dy) = (
                format!("distort_{}_dx", name.0),
                format!("distort_{}_dy", name.1),
                format!("warp_{}_dx", name.0),
                format!("warp_{}_dy", name.1),
            );

            distort_names.push((distort_dx.clone(), distort_dy.clone()));
            warp_names.push((warp_dx.clone(), warp_dy.clone()));

            for (name, mapped_name) in [
                (&name.0, &distort_dx),
                (&name.1, &distort_dy),
                (&name.0, &warp_dx),
                (&name.1, &warp_dy),
            ] {
                stream_defs.push(
                    Vid::builder()
                        .name(mapped_name.clone())
                        .path(&format!("{asset_path}/distorts/{}.mp4", name))
                        .resolution((640, 480))
                        .tbq((1, 12800))
                        .pix_fmt("yuv420p")
                        .repeat(true)
                        .realtime(false)
                        .hardware_decode(false)
                        .build(),
                );
                mix_configs.insert(
                    format!("{}_mix", mapped_name),
                    MixConfig {
                        def: VidMixer::builder()
                            .name(format!("{}_mix", mapped_name))
                            .width(640)
                            .height(480)
                            .build(),
                        mix: Mix::builder()
                            .name(format!("{}_mix", mapped_name))
                            .video(mapped_name)
                            .no_display(true)
                            .build(),
                    },
                );
            }
        }

        let wireframe = Vid::builder()
            .name("wireframe")
            .path(format!("{asset_path}/wireframe.png"))
            .resolution((640, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build();
        stream_defs.push(wireframe);
        mix_configs.insert(
            "wireframe_mix".to_string(),
            MixConfig {
                def: VidMixer::builder()
                    .name("wireframe_mix")
                    .width(640)
                    .height(480)
                    .build(),
                mix: Mix::builder()
                    .name("wireframe_mix")
                    .video("wireframe")
                    .no_display(true)
                    .build(),
            },
        );

        // config for wireframe
        mix_configs.insert(
            "wireframe_data_mix".to_string(),
            MixConfig {
                def: VidMixer::builder()
                    .name("wireframe_data_mix")
                    .width(640)
                    .height(480)
                    .shader(include_files(include_str!("glsl/wireframe.glsl")))
                    .build(),
                mix: Mix::builder()
                    .name("wireframe_data_mix")
                    .mixed("wireframe_mix")
                    .no_display(true)
                    .build(),
            },
        );

        // config for logs
        mix_configs.insert(
            "logs_mix".to_string(),
            MixConfig {
                def: VidMixer::builder()
                    .name("logs_mix")
                    .width(640)
                    .height(480)
                    .shader(include_files(include_str!("glsl/logs.glsl")))
                    .build(),
                mix: Mix::builder()
                    .name("logs_mix")
                    .mixed("blank_mix")
                    .no_display(true)
                    .build(),
            },
        );

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
                        .width(640)
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

        let feedback_modes = Self::feedback_modes()
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

            let seek_target = AllSettings::find_seek_target(&mix_configs, format!("{name}_mix"));
            let pb = PlaybackSettings {
                was_reset: true,
                stream: StreamSettings::new(StreamIdent {
                    name: name.clone(),
                    first_video: first_video.clone(),
                    input_mix: input_mix.def.name.clone(),
                    main_mix: mixer_graph.main_mix.def.name.clone(),
                    feedback_mix: mixer_graph.feedback.def.name.clone(),
                    overlay_mix: mixer_graph.overlay.def.name.clone(),
                    seek_target: seek_target.clone(),
                }),
                presets: PresetSettings {
                    baseline: StreamSettings::new(StreamIdent {
                        name: name.clone(),
                        first_video: first_video.clone(),
                        input_mix: input_mix.def.name.clone(),
                        main_mix: mixer_graph.main_mix.def.name.clone(),
                        feedback_mix: mixer_graph.feedback.def.name.clone(),
                        overlay_mix: mixer_graph.overlay.def.name.clone(),
                        seek_target: seek_target.clone(),
                    }),
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
            feedback_modes,
            overlay_names,
            distort_names,
            warp_names,
            distort_edge_types,
            lut_names,
            colors,
            asset_path,
            clipboard: StreamSettings::new(StreamIdent::default()),
            selected_knobs: 0,
            playback,
            initial_reset_complete: vec![false; playback_len],
            active_idx: 0,
            scan_idx: 0,
            midi_target_locked: None,
            display_idx: 0,
            logs: vec![String::new(); 100],
            last_line: String::new(),
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
        // Add any events to send midi to shader uniforms
        for ge in reg_events {
            if let GfxEvent::MidiEvent(me) = ge {
                specs.append(&mut self.midi_to_glsl(me));
            }
        }

        // Always capture live events even while recording is playing

        let current_selected_knob = self.selected_knobs as u8;
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

        if current_selected_knob != self.selected_knobs as u8 {
            specs.push(RenderSpec::SendMidi(SendMidi {
                event: MidiEvent {
                    device: MFT.to_string(),
                    channel: 4,
                    kind: MIDI_CONTROL_CHANGE,
                    key: (self.selected_knobs as u8) / 16,
                    velocity: 127,
                    timestamp: 0,
                },
            }));
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
                    .get_commands(&diffs.iter().map(|d| d.0).collect::<Vec<_>>()),
            );
            if let Some(buf) = self.playback[i].loops.record_buffer.as_mut() {
                let filtered_diffs = diffs
                    .into_iter()
                    .filter(|d| !d.0.properties().unwrap_or_default().do_not_record)
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
                                    prev_state.insert(diff.0, diff.1);
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
                                    prev_state.insert(diff.0, diff.1);
                                }
                            }
                        }

                        if curr == next {
                            self.playback[i].stream.apply_diff(&diffs);
                        } else if lp.tween {
                            let p = ((curr - prev_frame) as f64 / (next - prev_frame) as f64).abs();
                            let mut tween_diffs = vec![];
                            for diff in &diffs {
                                if let Some(prev_value) = prev_state.get(&diff.0).cloned() {
                                    if let Some(tweened) = &self.playback[i]
                                        .stream
                                        .tween_diff(diff.0, prev_value, diff.1, p)
                                    {
                                        tween_diffs.push((diff.0, *tweened));
                                    }
                                }
                            }
                            diffs = tween_diffs;
                            self.playback[i].stream.apply_diff(&diffs);
                        }

                        specs.append(
                            &mut self.playback[i]
                                .stream
                                .get_commands(&diffs.iter().map(|d| d.0).collect::<Vec<_>>()),
                        );
                    }
                }
            }
        }

        // WIRE FRAME DATA
        let mut label_data = String::new();
        let mut label_starts = vec![];
        let mut label_lens = vec![];
        let mut value_data = String::new();
        let mut value_starts = vec![];
        let mut value_lens = vec![];

        let page = self.selected_knobs as u8 / 16;
        for i in 0..16 {
            let cc = i + (16 * page);
            let (label, val, extra) = if let Some(field) = StreamSettings::find_field(0, cc) {
                let mut n = self.playback[self.active_idx].stream.get_field(&field);
                let val = format!("{}{:>5.05}", if n >= 0.0 { " " } else { "" }, n);
                if let Some(props) = field.properties() {
                    let label = props.label.unwrap_or_default().clone();
                    let extra = if field == StreamSettingsField::DistEdgeScan {
                        Some(self.distort_edge_types[n as usize].clone())
                    } else if field == StreamSettingsField::OverlayBlendScan
                        || field == StreamSettingsField::ScanlinesScan
                    {
                        n = n.clamp(0.0, (self.blend_modes.len() - 1) as f64);
                        Some(self.blend_modes[n as usize].clone())
                    } else if field == StreamSettingsField::LutScan {
                        n = n.clamp(0.0, (self.lut_names.len() - 1) as f64);
                        Some(self.lut_names[n as usize].clone())
                    } else if field == StreamSettingsField::FeedbackModeScan {
                        n = n.clamp(0.0, (self.feedback_modes.len() - 1) as f64);
                        Some(self.feedback_modes[n as usize].clone())
                    } else if field == StreamSettingsField::OverlayScan {
                        n = n.clamp(0.0, (self.overlay_names.len() - 1) as f64);
                        Some(self.overlay_names[n as usize].clone())
                    } else if field == StreamSettingsField::DistortScan {
                        n = n.clamp(0.0, (Self::distort_warp_base_names().len() - 1) as f64);
                        Some(Self::distort_warp_base_names()[n as usize].0.to_string())
                    } else if field == StreamSettingsField::WarpScan {
                        n = n.clamp(0.0, (Self::distort_warp_base_names().len() - 1) as f64);
                        Some(Self::distort_warp_base_names()[n as usize].1.to_string())
                    } else {
                        None
                    };
                    (label, val, extra)
                } else {
                    ("none".to_string(), val, None)
                }
            } else {
                ("none".to_string(), "".to_string(), None)
            };

            let start = label_data.len() as u32;
            label_starts.push(start);
            label_data.push_str(&format!(" "));
            label_data.push_str(&label);
            label_data.push_str(&format!(" "));
            label_lens.push(label_data.len() as u32 - start);

            let start = value_data.len() as u32;
            value_starts.push(start);
            if let Some(extra) = extra.clone() {
                value_data.push_str(&format!(" "));
                value_data.push_str(&extra);
            } else {
                value_data.push_str(&val);
            }
            label_data.push_str(&format!(" "));
            value_lens.push(value_data.len() as u32 - start);
        }

        // add current active
        let start = label_data.len() as u32;
        label_starts.push(start);
        label_data.push_str("Active");
        label_lens.push(label_data.len() as u32 - start);

        let start = value_data.len() as u32;
        value_starts.push(start);
        value_data.push_str(&self.playback[self.active_idx].stream.ident.name);
        value_lens.push(value_data.len() as u32 - start);

        // add current display
        let start = label_data.len() as u32;
        label_starts.push(start);
        label_data.push_str("Display");
        label_lens.push(label_data.len() as u32 - start);

        let display_name = &self.playback[self.display_idx].stream.ident.name;
        let start = value_data.len() as u32;
        value_starts.push(start);
        value_data.push_str(display_name);
        value_lens.push(value_data.len() as u32 - start);

        //add scanned video
        let start = label_data.len() as u32;
        label_starts.push(start);
        label_data.push_str("Scanned");
        label_lens.push(label_data.len() as u32 - start);

        let scanned_name = &self.playback[self.scan_idx].stream.ident.name;
        let start = value_data.len() as u32;
        value_starts.push(start);
        value_data.push_str(scanned_name);
        value_lens.push(value_data.len() as u32 - start);

        // Labels text data, starts, and lens
        let txt = label_data.bytes().map(|b| b as u32).collect::<Vec<_>>();
        specs.push(
            SendCmd::builder()
                .mix("wireframe_data_mix")
                .name(format!("label_data"))
                .value(SendValue::UVector(txt))
                .build()
                .into(),
        );
        specs.push(
            SendCmd::builder()
                .mix("wireframe_data_mix")
                .name(format!("label_starts"))
                .value(SendValue::UVector(label_starts.clone()))
                .build()
                .into(),
        );
        specs.push(
            SendCmd::builder()
                .mix("wireframe_data_mix")
                .name(format!("label_lens"))
                .value(SendValue::UVector(label_lens.clone()))
                .build()
                .into(),
        );

        // Values text data, starts, and lens
        let txt = value_data.bytes().map(|b| b as u32).collect::<Vec<_>>();
        specs.push(
            SendCmd::builder()
                .mix("wireframe_data_mix")
                .name(format!("value_data"))
                .value(SendValue::UVector(txt))
                .build()
                .into(),
        );
        specs.push(
            SendCmd::builder()
                .mix("wireframe_data_mix")
                .name(format!("value_starts"))
                .value(SendValue::UVector(value_starts.clone()))
                .build()
                .into(),
        );
        specs.push(
            SendCmd::builder()
                .mix("wireframe_data_mix")
                .name(format!("value_lens"))
                .value(SendValue::UVector(value_lens.clone()))
                .build()
                .into(),
        );

        specs.push(
            SendCmd::builder()
                .mix("wireframe_data_mix")
                .name("selected_button")
                .value(SendValue::Integer(
                    self.selected_knobs as i32 - (16 * page as i32),
                ))
                .build()
                .into(),
        );

        specs.push(
            SendCmd::builder()
                .mix("wireframe_data_mix")
                .name("selected_button_str")
                .value(SendValue::UVector(
                    format!("{:03}", self.selected_knobs as i32 - (16 * page as i32))
                        .into_bytes()
                        .iter()
                        .map(|b| *b as u32)
                        .collect::<Vec<_>>(),
                ))
                .build()
                .into(),
        );

        specs.push(
            SendCmd::builder()
                .mix("wireframe_data_mix")
                .name("midi_target")
                .value(SendValue::UVector(
                    if let Some(idx) = self.midi_target_locked {
                        format!("Midi Tgt:{idx}")
                    } else {
                        "Midi Tgt: Unlocked".to_string()
                    }
                    .as_bytes()
                    .iter()
                    .map(|b| *b as u32)
                    .collect::<Vec<_>>(),
                ))
                .build()
                .into(),
        );

        // LOGS DATA
        let mut txt = vec![0; 8100];
        let mut starts = vec![0; 100];
        let mut ends = vec![0; 100];

        let mut start = 0;
        for i in 0..self.logs.len().min(100) {
            let line = &self.logs[i];
            let mut end = start;
            for b in line.bytes().take(80) {
                txt[end] = b as i32;
                end += 1;
            }
            starts[i] = start as i32;
            ends[i] = end as i32;
            start = end;
        }
        specs.push(
            SendCmd::builder()
                .mix("logs_mix")
                .name("txt")
                .value(SendValue::IVector(txt))
                .build()
                .into(),
        );

        specs.push(
            SendCmd::builder()
                .mix("logs_mix")
                .name("line_start")
                .value(SendValue::IVector(starts))
                .build()
                .into(),
        );
        specs.push(
            SendCmd::builder()
                .mix("logs_mix")
                .name("line_end")
                .value(SendValue::IVector(ends))
                .build()
                .into(),
        );

        let mut seen = HashMap::<String, Mix>::new();

        // TOP
        let mix_name = self.playback[self.active_idx].stream.overlay_mix();
        specs.append(&mut self.do_display(&mut seen, mix_name, (0, 0)));
        // BOTTOM
        let mix_name = self.playback[self.display_idx].stream.overlay_mix();
        specs.append(&mut self.do_display(&mut seen, mix_name, (0, TARGET_SIZE_H as i32)));
        // wireframe_data_mix
        let mix_name = "wireframe_data_mix";
        let mut last_frame_lock = LAST_FRAME.lock().unwrap();
        if *last_frame_lock != 0 {
            let msg = format!("Dropped: {}", (frame - *last_frame_lock) as i32 - 1);
            specs.push(
                SendCmd::builder()
                    .mix(mix_name)
                    .name("dropped")
                    .value(SendValue::UVector(
                        msg.as_bytes().iter().map(|b| *b as u32).collect(),
                    ))
                    .build()
                    .into(),
            );
            specs.push(
                SendCmd::builder()
                    .mix(mix_name)
                    .name("dropped_length")
                    .value(SendValue::Unsigned(msg.len() as u32))
                    .build()
                    .into(),
            );
        }
        *last_frame_lock = frame;

        drop(last_frame_lock);

        specs.append(&mut self.do_display(&mut seen, mix_name, (TARGET_SIZE_W as i32, 0)));
        // logs panel
        let mix_name = "logs_mix";
        specs.append(&mut self.do_display(
            &mut seen,
            mix_name,
            (TARGET_SIZE_W as i32, TARGET_SIZE_H as i32),
        ));

        let to_return = specs.clone();
        self.clean_up_by_specs(&mut specs);

        Ok(to_return)
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
        //Update streams for incoming frame events
        for ge in reg_events {
            match ge {
                GfxEvent::LogEvent(log) => {
                    self.log(log.message.clone());
                }
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
                                    .filter(|d| !d.0.properties().unwrap_or_default().do_not_record)
                                    .collect::<Vec<_>>();
                                self.playback[selected_idx].stream.apply_diff(&diffs);
                            } else {
                                self.playback[selected_idx].presets.baseline =
                                    self.playback[selected_idx].stream.clone();
                            }
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_t,
                            down: true,
                            shift,
                            ..
                        } => {
                            if *shift {
                                self.midi_target_locked.take();
                            } else {
                                self.midi_target_locked.replace(self.active_idx);
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
                                | KeyCode::SDLK_8
                                | KeyCode::SDLK_9
                                | KeyCode::SDLK_0,
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
                                    .filter(|d| !d.0.properties().unwrap_or_default().do_not_record)
                                    .collect::<Vec<_>>();
                            } else if *alt && *down && !*ctl && !*shift {
                                // save time
                                self.playback[selected_idx].presets.saved[selected_preset] =
                                    vec![(
                                        StreamSettingsField::ExactSec,
                                        self.playback[self.active_idx].stream.real_ts.0 as f64
                                            / self.playback[self.active_idx].stream.real_ts.1
                                                as f64,
                                    )];
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
                                    // tween: self.playback[selected_idx]
                                    //     .stream
                                    //     .get_field(&StreamSettingsField::Tween)
                                    //     >= 0.0,
                                    tween: false,
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
                            self.playback[self.active_idx]
                                .stream
                                .set_field(StreamSettingsField::ExactSec, 0.01);
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
                            self.playback[self.active_idx]
                                .stream
                                .adjust_field(&StreamSettingsField::Pause, 1.0);
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_h,
                            down: true,
                            ..
                        } => {
                            self.scan_idx = (self.scan_idx as i64 + 1)
                                .clamp(0, self.playback.len() as i64 - 1)
                                as usize;
                            self.active_idx = self.scan_idx;
                            self.display_idx = self.scan_idx;
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_g,
                            down: true,
                            ..
                        } => {
                            self.scan_idx = (self.scan_idx as i64 - 1)
                                .clamp(0, self.playback.len() as i64 - 1)
                                as usize;
                            self.active_idx = self.scan_idx;
                            self.display_idx = self.scan_idx;
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
                            shift,
                            ..
                        } => {
                            if *shift {
                                self.selected_knobs = (self.selected_knobs + 40).clamp(0, 127);
                            } else {
                                self.selected_knobs = (self.selected_knobs + 4).clamp(0, 127);
                            }
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_UP,
                            down: true,
                            shift,
                            ..
                        } => {
                            if *shift {
                                self.selected_knobs =
                                    (self.selected_knobs as i32 - 40 as i32).clamp(0, 127) as usize;
                            } else {
                                self.selected_knobs =
                                    (self.selected_knobs as i32 - 4 as i32).clamp(0, 127) as usize;
                            }
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_LEFT,
                            down: true,
                            shift,
                            ..
                        } => {
                            if *shift {
                                self.selected_knobs =
                                    (self.selected_knobs as i32 - 10 as i32).clamp(0, 127) as usize;
                            } else {
                                self.selected_knobs =
                                    (self.selected_knobs as i32 - 1 as i32).clamp(0, 127) as usize;
                            }

                            if false {
                                let mix = self.playback[self.active_idx].stream.overlay_mix();
                                let specs =
                                    self.get_playback_specs(mix, (0, 0, 1, 1), (0, 0, 1, 1));
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
                                            self.playback[i].stream.adjust_field(
                                                &StreamSettingsField::DeltaSec,
                                                if *shift { -10.0 } else { -1.0 },
                                            );
                                            break;
                                        }
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
                            if *shift {
                                self.selected_knobs =
                                    (self.selected_knobs as i32 + 10 as i32).clamp(0, 127) as usize;
                            } else {
                                self.selected_knobs =
                                    (self.selected_knobs as i32 + 1 as i32).clamp(0, 127) as usize;
                            }

                            if false {
                                let mix = self.playback[self.active_idx].stream.overlay_mix();
                                let specs =
                                    self.get_playback_specs(mix, (0, 0, 1, 1), (0, 0, 1, 1));
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
                                            self.playback[i].stream.adjust_field(
                                                &StreamSettingsField::DeltaSec,
                                                if *shift { 10.0 } else { 1.0 },
                                            );
                                            break;
                                        }
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
                            key: KeyCode::SDLK_LEFTBRACKET,
                            shift,
                            down: true,
                            ..
                        } => {
                            let scale = if *shift { 10 } else { 1 };
                            self.scan_idx = (self.scan_idx as i64 - scale)
                                .clamp(0, self.playback.len() as i64 - 1)
                                as usize;
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_RIGHTBRACKET,
                            shift,
                            down: true,
                            ..
                        } => {
                            let scale = if *shift { 10 } else { 1 };
                            self.scan_idx = (self.scan_idx as i64 + scale)
                                .clamp(0, self.playback.len() as i64 - 1)
                                as usize;
                        }
                        KeyEvent {
                            key: KeyCode::SDLK_BACKSLASH,
                            shift,
                            down: true,
                            ..
                        } => {
                            if *shift {
                                self.display_idx = self.scan_idx;
                            } else {
                                self.active_idx = self.scan_idx;
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        // scroll all the settings one frame
        for i in 0..self.playback.len() {
            let stream = &mut self.playback[i].stream;
            stream.set_field(
                StreamSettingsField::ScrolledH,
                stream.get_field(&StreamSettingsField::ScrolledH)
                    + stream.get_field(&StreamSettingsField::ScrollH),
            );
            stream.set_field(
                StreamSettingsField::ScrolledV,
                stream.get_field(&StreamSettingsField::ScrolledV)
                    + stream.get_field(&StreamSettingsField::ScrollV),
            );
        }
        Ok(())
    }

    pub fn find_seek_target<S: AsRef<str>>(
        mix_configs: &HashMap<String, MixConfig>,
        root: S,
    ) -> String {
        let mut seen = HashSet::new();
        let mut todo = VecDeque::new();
        if let Some(mix_config) = mix_configs.get(root.as_ref()) {
            todo.push_back(mix_config);
        }
        while let Some(curr) = todo.pop_front() {
            seen.insert(curr.mix.name.clone());
            if let Some(hint) = &curr.mix.seek_target_hint {
                return hint.clone();
            }

            if !curr
                .mix
                .inputs
                .iter()
                .any(|i| matches!(i, MixInput::Mixed(_)))
            {
                if let Some(MixInput::Video(vid)) = curr.mix.inputs.get(0) {
                    return vid.clone();
                } else {
                    return String::new();
                }
            }

            for input in &curr.mix.inputs {
                if let MixInput::Mixed(mix_name) = input {
                    if !seen.contains(mix_name) {
                        if let Some(mix_config) = mix_configs.get(mix_name) {
                            todo.push_back(mix_config);
                        }
                    }
                }
            }
        }
        String::new()
    }

    pub fn adjust(&mut self, kn: Knob, shift: bool, inc: f64) {
        let playback = &mut self.playback[self.active_idx];
        let inc = if shift { inc * 10.0 } else { inc };
        match (kn, self.selected_knobs) {
            (Knob::B, kn) => {
                if let Some(field) = StreamSettingsField::find(0, kn as u8) {
                    playback.stream.adjust_field(&field, inc);
                }
            }
            (Knob::CB, kn) => {
                if let Some(field) = StreamSettingsField::find(1, kn as u8) {
                    playback.stream.adjust_field(&field, inc);
                }
            }
            _ => (),
        }
    }

    pub fn clean_up_by_specs(&mut self, specs: &mut Vec<RenderSpec>) {
        // RESET SEEK
        for i in 0..self.playback.len() {
            self.playback[i]
                .stream
                .set_field(StreamSettingsField::DeltaSec, 0.0);
            self.playback[i]
                .stream
                .set_field(StreamSettingsField::Scrub, 0.0);
            self.playback[i]
                .stream
                .set_field(StreamSettingsField::ExactSec, 0.0);
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
                    if self.playback[fidx]
                        .stream
                        .get_field(&StreamSettingsField::Pause)
                        != 0.0
                        && self.playback[fidx]
                            .stream
                            .get_field(&StreamSettingsField::DeltaSec)
                            == 0.0
                        && self.playback[fidx]
                            .stream
                            .get_field(&StreamSettingsField::ExactSec)
                            == 0.0
                        && self.playback[fidx]
                            .stream
                            .get_field(&StreamSettingsField::Scrub)
                            == 0.0
                    {
                        paused.push(fidx);
                        continue;
                    }

                    let lut = &self.lut_names[self.playback[fidx]
                        .stream
                        .get_field(&StreamSettingsField::LutSelected)
                        as usize];
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
                    let (distort_x, distort_y) = self.distort_names[self.playback[fidx]
                        .stream
                        .get_field(&StreamSettingsField::DistortSelected)
                        as usize]
                        .clone();
                    let (warp_x, warp_y) = self.warp_names[self.playback[fidx]
                        .stream
                        .get_field(&StreamSettingsField::WarpSelected)
                        as usize]
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
                    let overly = &self.overlay_names[self.playback[fidx]
                        .stream
                        .get_field(&StreamSettingsField::OverlaySelected)
                        as usize];
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
                    streamsettings::ALL_FIELDS
                        .iter()
                        .cloned()
                        .map(|f| self.playback[*i].stream.command(&f))
                        .flatten(),
                );
            }
        }
        specs
    }

    pub fn is_playback_reset<S: AsRef<str>>(&self, name: S) -> bool {
        self.playback
            .iter()
            .enumerate()
            .find(|(_, p)| p.stream.ident.name == name.as_ref())
            .map(|(i, _)| self.initial_reset_complete[i])
            .unwrap_or(false)
    }
}

pub fn time_code_2_float<T>(tc: T) -> f64
where
    T: AsRef<str>,
{
    let parts: Vec<&str> = tc.as_ref().split(':').collect();
    let hours: f64 = parts[0].parse().unwrap_or(0.0);
    let minutes: f64 = parts[1].parse().unwrap_or(0.0);
    let seconds: f64 = parts[2].parse().unwrap_or(0.0);
    let frames: f64 = parts[3].parse().unwrap_or(0.0) * 1.0 / 24.0;
    hours * 3600.0 + minutes * 60.0 + seconds + frames
}

#[macro_export]
macro_rules! beat_time_boilerplate {
    ( $all_settings:expr, $midi_event:expr, $pb_name:expr, $time_codes:expr) => {
        static PB_IDX: std::sync::LazyLock<Option<usize>> = std::sync::LazyLock::new(|| {
            let mut idx = None;
            for i in 0..PLAYBACK_NAMES.len() {
                if PLAYBACK_NAMES[i] == $pb_name {
                    idx.replace(i);
                    break;
                }
            }
            idx
        });

        if PB_IDX.is_none() {
            return;
        }

        static TIME_IDX: std::sync::LazyLock<Mutex<std::cell::RefCell<usize>>> =
            std::sync::LazyLock::new(|| Mutex::new(std::cell::RefCell::new(0)));
        if let Some(pb_idx) = *PB_IDX {
            if $all_settings.active_idx != pb_idx && $all_settings.display_idx != pb_idx {
                return;
            }

            // INTERNAL MATCHING FOR SETTING MODIFICATION
            match (
                $midi_event.device.as_str(),
                $midi_event.channel,
                $midi_event.kind,
                $midi_event.key,
                $midi_event.velocity,
            ) {
                (vizwasm::vizconfig::IAC, 0, MIDI_CONTROL_CHANGE, 0, v) => {
                    if v > 10 {
                        let lock = TIME_IDX.lock().unwrap();
                        let mut idx = lock.borrow_mut();

                        let mut next_idx = rand::random::<u32>() % $time_codes.len() as u32;
                        if next_idx == *idx as u32 {
                            next_idx = (next_idx + 1) % $time_codes.len() as u32;
                        }
                        *idx = next_idx as usize;
                        $all_settings.playback[pb_idx].stream.set_field(
                            vizwasm::streamsettings::StreamSettingsField::ExactSec,
                            *$time_codes.get(*idx as usize).unwrap_or(&1.0),
                        );
                    }
                }
                _ => (),
            }
        }
    };
}

pub const IAC: &str = "IAC Driver Bus 1";
pub const IAC_GLSL: &str = "iac_driver_bus_1";
pub const MFT: &str = "Midi Fighter Twister";
pub const MFT_GLSL: &str = "midi_fighter_twister";
pub const MPK: &str = "MPK mini 3";
pub const MPK_GLSL: &str = "mpk_mini_3";

const MIDI_DEVICE_VARS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(IAC.to_string(), IAC_GLSL.to_string());
    m.insert(MFT.to_string(), MFT_GLSL.to_string());
    m.insert(MPK.to_string(), MPK_GLSL.to_string());
    m
});

impl AllSettings {
    pub fn reload_encoders_for_active_idx(&self) -> Vec<RenderSpec> {
        streamsettings::ALL_FIELDS
            .iter()
            .cloned()
            .map(|f| {
                let value = self.playback[self.active_idx].stream.get_field(&f);
                if let Some(prop) = f.properties() {
                    if prop.channel == Some(0) {
                        if let (Some(min), Some(max)) = (prop.min, prop.max) {
                            let midi_value =
                                (((value - min) / (max - min)) * 127.0).clamp(0.0, 127.0) as u8;
                            return Some(RenderSpec::SendMidi(SendMidi {
                                event: MidiEvent {
                                    device: MFT.to_string(),
                                    channel: 0,
                                    kind: MIDI_CONTROL_CHANGE,
                                    key: prop.cc.unwrap_or_default(),
                                    velocity: midi_value,
                                    timestamp: 0,
                                },
                            }));
                        }
                    }
                }
                None
            })
            .flatten()
            .collect()
    }

    pub fn video_fight_cb(&mut self, event: &MidiEvent) {
        if event.kind != MIDI_CONTROL_CHANGE {
            return;
        }

        if event.device == MFT {
            if event.channel == 1 || event.channel == 0 {
                self.selected_knobs = event.key as usize;
            } else if event.channel == 3 && event.velocity > 0 {
                self.selected_knobs = (event.key * 16) as usize;
            }

            if let Some(field) = StreamSettingsField::find(event.channel, event.key) {
                let idx = self.active_idx;
                match event.velocity {
                    63 => {
                        self.playback[idx].stream.adjust_field(&field, -1.0);
                    }
                    65 => {
                        self.playback[idx].stream.adjust_field(&field, 1.0);
                    }
                    127 => {
                        // this is an "assign" command, adjust is overriden and the 0 is ignored
                        self.playback[idx].stream.adjust_field(&field, 0.0);
                    }
                    _ => {}
                }
            } else if let Some(field) = StreamSettingsField::find(0, event.key) {
                if event.channel == 1 {
                    //special case - channel was NOT zero but there is a setting at this key
                    // reset to default
                    let idx = self.active_idx;

                    self.playback[idx].stream.set_field(
                        field,
                        field
                            .properties()
                            .unwrap_or_default()
                            .default
                            .unwrap_or_default(),
                    );
                }
            }
        } else if event.device == IAC {
            let channel = if event.channel == 4 {
                0
            } else if event.channel == 5 {
                1
            } else {
                return;
            };
            if let Some(field) = StreamSettingsField::find(channel, event.key) {
                let idx = if let Some(idx) = self.midi_target_locked {
                    idx
                } else {
                    self.active_idx
                };
                let min = field
                    .properties()
                    .unwrap_or_default()
                    .min
                    .unwrap_or_default();
                let max = field
                    .properties()
                    .unwrap_or_default()
                    .max
                    .unwrap_or_default();
                let value = event.velocity as f64 / 127.0 * (max - min) + min;
                self.playback[idx].stream.set_field(field, value);
            }
        }
    }

    // Generic send for all midi devices to GLSL vars
    pub fn midi_to_glsl(&mut self, event: &MidiEvent) -> Vec<RenderSpec> {
        let debug_kind = match event.kind {
            MIDI_NOTE_ON => "note",
            MIDI_NOTE_OFF => "note",
            MIDI_CONTROL_CHANGE => "cc",
            _ => "???",
        };

        let on_off = match event.kind {
            MIDI_NOTE_ON => "_on",
            MIDI_NOTE_OFF => "_off",
            _ => "",
        };
        let debug_device = MIDI_DEVICE_VARS
            .get(&event.device)
            .cloned()
            .or_else(|| Some(format!("???({})", event.device).to_string()))
            .unwrap();

        eprintln!(
            "{debug_kind}_{debug_device}_{}_{}{} = {}",
            event.channel, event.key, on_off, event.velocity
        );

        let mut cmds = vec![];
        if let Some(glsl_device) = MIDI_DEVICE_VARS.get(&event.device) {
            for mix in self.playback_names.iter() {
                match event.kind {
                    MIDI_NOTE_ON => {
                        cmds.push(RenderSpec::SendCmd(SendCmd {
                            mix: format!("{mix}_mix").to_string(),
                            name: format!("note_{}_{}_{}", glsl_device, event.channel, event.key)
                                .to_string(),
                            value: SendValue::Float(event.velocity as f32),
                        }));
                        cmds.push(RenderSpec::SendCmd(SendCmd {
                            mix: format!("{mix}_mix").to_string(),
                            name: format!(
                                "note_{}_{}_{}_on",
                                glsl_device, event.channel, event.key
                            )
                            .to_string(),
                            value: SendValue::Unsigned(1),
                        }));
                    }
                    MIDI_NOTE_OFF => {
                        cmds.push(RenderSpec::SendCmd(SendCmd {
                            mix: format!("{mix}_mix").to_string(),
                            name: format!("note_{}_{}_{}", glsl_device, event.channel, event.key)
                                .to_string(),
                            value: SendValue::Float(0.0),
                        }));
                        cmds.push(RenderSpec::SendCmd(SendCmd {
                            mix: format!("{mix}_mix").to_string(),
                            name: format!(
                                "note_{}_{}_{}_on",
                                glsl_device, event.channel, event.key
                            )
                            .to_string(),
                            value: SendValue::Unsigned(0),
                        }));
                    }
                    MIDI_CONTROL_CHANGE => {
                        cmds.push(RenderSpec::SendCmd(SendCmd {
                            mix: format!("{mix}_mix").to_string(),
                            name: format!("cc_{}_{}_{}", glsl_device, event.channel, event.key)
                                .to_string(),
                            value: SendValue::Float(event.velocity as f32),
                        }));
                    }
                    _ => (),
                }
            }
        }

        return cmds;
    }

    pub fn log<T: AsRef<str>>(&mut self, line: T) {
        let mut s = line.as_ref().to_string();

        // replace all unicode characters with a placeholder to avoid rendering issues in the log
        s = s
            .chars()
            .map(|c| if c.is_ascii() { c } else { '?' })
            .collect();

        if s.starts_with("gfx_ll>") {
            //filter some of the content
            return;
        }

        if self.last_line == s {
            let mut rep = self.logs[self.logs.len() - 1].clone();
            rep.truncate(80 - " (rep.)".len());
            rep.push_str(" (rep.)");
        } else {
            self.last_line = s.clone();

            while s.len() > 80 {
                let remainder = s.split_off(80);
                self.logs.push(s);
                s = remainder;
            }
            self.logs.push(s);
        }

        if self.logs.len() > 100 {
            self.logs.remove(0);
        }
    }

    pub fn do_display<T: AsRef<str>>(
        &mut self,
        seen: &mut HashMap<String, Mix>,
        mix_name: T,
        offset: (i32, i32),
    ) -> Vec<RenderSpec> {
        let mut specs = vec![];
        if let Some(mix_config) = self.mix_configs.get_mut(mix_name.as_ref()) {
            let iw = mix_config.def.width as i32;
            let ih = mix_config.def.height as i32;
            let mut ow = iw;
            let mut oh = ih;
            let mut ix = 0;
            let mut iy = 0;

            let iaspect = iw as f32 / ih as f32;
            let oaspect = TARGET_SIZE_W as f32 / TARGET_SIZE_H as f32;

            // correct aspect ratio
            if iaspect > oaspect {
                let effective_ow = (ih as f32 * oaspect) as i32;
                ix = (ow - effective_ow) / 2;
                ow = effective_ow;
            } else if iaspect < oaspect {
                let effective_oh = (iw as f32 / oaspect) as i32;
                iy = (oh - effective_oh) / 2;
                oh = effective_oh;
            }
            let src = (ix, iy, ow as u32, oh as u32);
            let dst = (offset.0, offset.1, TARGET_SIZE_W, TARGET_SIZE_H);

            let playback_specs = self.get_playback_specs(&mix_name, src, dst);
            for spec in playback_specs {
                if let RenderSpec::Mix(mix) = &spec {
                    let other = seen.get(&mix.name);
                    if let Some(other) = other {
                        if other.target == mix.target {
                            // If the mix already exists, skip adding it again.
                            continue;
                        }
                    }
                    seen.insert(mix.name.clone(), mix.clone());
                }
                specs.push(spec);
            }
        }
        specs
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct DisplayText {
    pub chars: Vec<u32>,
    pub lens: Vec<u32>,
    pub starts: Vec<u32>,
}

impl<S: AsRef<str>> From<S> for DisplayText {
    fn from(s: S) -> Self {
        let mut text = DisplayText::default();
        for line in s.as_ref().lines() {
            let as_ints = unicode_to_cp437(line);
            text.starts.push(text.chars.len() as u32);
            text.lens.push(as_ints.len() as u32);
            text.chars.extend(as_ints);
        }
        text
    }
}

impl DisplayText {
    pub fn get_specs<S: AsRef<str>>(
        &self,
        target: S,
        data_var: S,
        starts_var: S,
        lens_var: S,
    ) -> Vec<RenderSpec> {
        vec![
            SendCmd::builder()
                .mix(target.as_ref())
                .name(data_var.as_ref())
                .value(SendValue::UVector(self.chars.clone()))
                .build()
                .into(),
            SendCmd::builder()
                .mix(target.as_ref())
                .name(lens_var.as_ref())
                .value(SendValue::UVector(self.lens.clone()))
                .build()
                .into(),
            SendCmd::builder()
                .mix(target.as_ref())
                .name(starts_var.as_ref())
                .value(SendValue::UVector(self.starts.clone()))
                .build()
                .into(),
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextFileLoader {
    path: PathBuf,
    last_loaded: u64,
    data: String,
}

impl TextFileLoader {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().into(),
            last_loaded: 0,
            data: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.last_loaded = 0;
        self.data.clear();
    }

    pub fn refresh(&mut self) -> bool {
        if let Some(mtime) = std::fs::metadata(&self.path)
            .and_then(|meta| meta.modified())
            .map(|mtime| {
                mtime
                    .duration_since(std::time::UNIX_EPOCH)
                    .ok()
                    .map(|d| d.as_secs())
            })
            .ok()
            .flatten()
        {
            if mtime <= self.last_loaded {
                return false;
            }

            if let Some(data) = std::fs::read_to_string(&self.path).ok() {
                self.data = data;
                self.last_loaded = mtime;
                return true;
            }
        }
        false
    }

    pub fn data(&self) -> &str {
        &self.data
    }
}

pub static CP437: LazyLock<HashMap<char, u32>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert('\u{000000}', 0x00); // NULL
    m.insert('☺', 0x01); // WHITE SMILING FACE
    m.insert('\u{00263B}', 0x02); // BLACK SMILING FACE
    m.insert('\u{002665}', 0x03); // BLACK HEART SUIT
    m.insert('\u{002666}', 0x04); // BLACK DIAMOND SUIT
    m.insert('\u{002663}', 0x05); // BLACK CLUB SUIT
    m.insert('\u{002660}', 0x06); // BLACK SPADE SUIT
    m.insert('\u{002022}', 0x07); // BULLET
    m.insert('\u{0025D8}', 0x08); // INVERSE BULLET
    m.insert('\u{0025CB}', 0x09); // WHITE CIRCLE
    m.insert('\u{0025D9}', 0x0A); // INVERSE WHITE CIRCLE
    m.insert('\u{002642}', 0x0B); // MALE SIGN
    m.insert('\u{002640}', 0x0C); // FEMALE SIGN
    m.insert('\u{0026A5}', 0x0D); // MALE AND FEMALE SIGN, VARIANT
    m.insert('\u{00266B}', 0x0E); // BEAMED EIGHTH NOTES
    m.insert('\u{00263C}', 0x0F); // WHITE SUN WITH RAYS
    m.insert('\u{0025BA}', 0x10); // BLACK RIGHT-POINTING POINTER
    m.insert('\u{0025C4}', 0x11); // BLACK LEFT-POINTING POINTER
    m.insert('\u{002195}', 0x12); // UP DOWN ARROW
    m.insert('\u{00203C}', 0x13); // DOUBLE EXCLAMATION MARK
    m.insert('\u{0000B6}', 0x14); // PILCROW SIGN
    m.insert('\u{0000A7}', 0x15); // SECTION SIGN
    m.insert('\u{0025AC}', 0x16); // BLACK RECTANGLE
    m.insert('\u{0021A8}', 0x17); // UP DOWN ARROW WITH BASE
    m.insert('\u{002191}', 0x18); // UPWARDS ARROW
    m.insert('\u{002193}', 0x19); // DOWNWARDS ARROW
    m.insert('\u{002192}', 0x1A); // RIGHTWARDS ARROW
    m.insert('\u{002190}', 0x1B); // LEFTWARDS ARROW
    m.insert('\u{00221F}', 0x1C); // RIGHT ANGLE
    m.insert('\u{002194}', 0x1D); // LEFT RIGHT ARROW
    m.insert('\u{0025B2}', 0x1E); // BLACK UP-POINTING TRIANGLE
    m.insert('\u{0025BC}', 0x1F); // BLACK DOWN-POINTING TRIANGLE
                                  // BEGIN ASCII SECTION
    m.insert(' ', 0x20);
    m.insert('!', 0x21);
    m.insert('"', 0x22);
    m.insert('#', 0x23);
    m.insert('$', 0x24);
    m.insert('%', 0x25);
    m.insert('&', 0x26);
    m.insert('\'', 0x27);
    m.insert('(', 0x28);
    m.insert(')', 0x29);
    m.insert('*', 0x2A);
    m.insert('+', 0x2B);
    m.insert(',', 0x2C);
    m.insert('-', 0x2D);
    m.insert('.', 0x2E);
    m.insert('/', 0x2F);
    m.insert('0', 0x30);
    m.insert('1', 0x31);
    m.insert('2', 0x32);
    m.insert('3', 0x33);
    m.insert('4', 0x34);
    m.insert('5', 0x35);
    m.insert('6', 0x36);
    m.insert('7', 0x37);
    m.insert('8', 0x38);
    m.insert('9', 0x39);
    m.insert(':', 0x3A);
    m.insert(';', 0x3B);
    m.insert('<', 0x3C);
    m.insert('=', 0x3D);
    m.insert('>', 0x3E);
    m.insert('?', 0x3F);
    m.insert('@', 0x40);
    m.insert('A', 0x41);
    m.insert('B', 0x42);
    m.insert('C', 0x43);
    m.insert('D', 0x44);
    m.insert('E', 0x45);
    m.insert('F', 0x46);
    m.insert('G', 0x47);
    m.insert('H', 0x48);
    m.insert('I', 0x49);
    m.insert('J', 0x4A);
    m.insert('K', 0x4B);
    m.insert('L', 0x4C);
    m.insert('M', 0x4D);
    m.insert('N', 0x4E);
    m.insert('O', 0x4F);
    m.insert('P', 0x50);
    m.insert('Q', 0x51);
    m.insert('R', 0x52);
    m.insert('S', 0x53);
    m.insert('T', 0x54);
    m.insert('U', 0x55);
    m.insert('V', 0x56);
    m.insert('W', 0x57);
    m.insert('X', 0x58);
    m.insert('Y', 0x59);
    m.insert('Z', 0x5A);
    m.insert('[', 0x5B);
    m.insert('\\', 0x5C);
    m.insert(']', 0x5D);
    m.insert('^', 0x5E);
    m.insert('_', 0x5F);
    m.insert('`', 0x60);
    m.insert('a', 0x61);
    m.insert('b', 0x62);
    m.insert('c', 0x63);
    m.insert('d', 0x64);
    m.insert('e', 0x65);
    m.insert('f', 0x66);
    m.insert('g', 0x67);
    m.insert('h', 0x68);
    m.insert('i', 0x69);
    m.insert('j', 0x6A);
    m.insert('k', 0x6B);
    m.insert('l', 0x6C);
    m.insert('m', 0x6D);
    m.insert('n', 0x6E);
    m.insert('o', 0x6F);
    m.insert('p', 0x70);
    m.insert('q', 0x71);
    m.insert('r', 0x72);
    m.insert('s', 0x73);
    m.insert('t', 0x74);
    m.insert('u', 0x75);
    m.insert('v', 0x76);
    m.insert('w', 0x77);
    m.insert('x', 0x78);
    m.insert('y', 0x79);
    m.insert('z', 0x7A);
    m.insert('{', 0x7B);
    m.insert('|', 0x7C);
    m.insert('}', 0x7D);
    m.insert('~', 0x7E);
    // END ASCII SECTION
    m.insert('\u{002302}', 0x7F); // HOUSE
    m.insert('\u{0000C7}', 0x80); // LATIN CAPITAL LETTER C WITH CEDILLA
    m.insert('\u{0000FC}', 0x81); // LATIN SMALL LETTER U WITH DIAERESIS
    m.insert('\u{0000E9}', 0x82); // LATIN SMALL LETTER E WITH ACUTE
    m.insert('\u{0000E2}', 0x83); // LATIN SMALL LETTER A WITH CIRCUMFLEX
    m.insert('\u{0000E4}', 0x84); // LATIN SMALL LETTER A WITH DIAERESIS
    m.insert('\u{0000E0}', 0x85); // LATIN SMALL LETTER A WITH GRAVE
    m.insert('\u{0000E5}', 0x86); // LATIN SMALL LETTER A WITH RING ABOVE
    m.insert('\u{0000E7}', 0x87); // LATIN SMALL LETTER C WITH CEDILLA
    m.insert('\u{0000EA}', 0x88); // LATIN SMALL LETTER E WITH CIRCUMFLEX
    m.insert('\u{0000EB}', 0x89); // LATIN SMALL LETTER E WITH DIAERESIS
    m.insert('\u{0000E8}', 0x8A); // LATIN SMALL LETTER E WITH GRAVE
    m.insert('\u{0000EF}', 0x8B); // LATIN SMALL LETTER I WITH DIAERESIS
    m.insert('\u{0000EE}', 0x8C); // LATIN SMALL LETTER I WITH CIRCUMFLEX
    m.insert('\u{0000EC}', 0x8D); // LATIN SMALL LETTER I WITH GRAVE
    m.insert('\u{0000C4}', 0x8E); // LATIN CAPITAL LETTER A WITH DIAERESIS
    m.insert('\u{0000C5}', 0x8F); // LATIN CAPITAL LETTER A WITH RING ABOVE
    m.insert('\u{0000C9}', 0x90); // LATIN CAPITAL LETTER E WITH ACUTE
    m.insert('\u{0000E6}', 0x91); // LATIN SMALL LETTER AE
    m.insert('\u{0000C6}', 0x92); // LATIN CAPITAL LETTER AE
    m.insert('\u{0000F4}', 0x93); // LATIN SMALL LETTER O WITH CIRCUMFLEX
    m.insert('\u{0000F6}', 0x94); // LATIN SMALL LETTER O WITH DIAERESIS
    m.insert('\u{0000F2}', 0x95); // LATIN SMALL LETTER O WITH GRAVE
    m.insert('\u{0000FB}', 0x96); // LATIN SMALL LETTER U WITH CIRCUMFLEX
    m.insert('\u{0000F9}', 0x97); // LATIN SMALL LETTER U WITH GRAVE
    m.insert('\u{0000FF}', 0x98); // LATIN SMALL LETTER Y WITH DIAERESIS
    m.insert('\u{0000D6}', 0x99); // LATIN CAPITAL LETTER O WITH DIAERESIS
    m.insert('\u{0000DC}', 0x9A); // LATIN CAPITAL LETTER U WITH DIAERESIS
    m.insert('\u{0000A2}', 0x9B); // CENT SIGN
    m.insert('\u{0000A3}', 0x9C); // POUND SIGN
    m.insert('\u{0000A5}', 0x9D); // YEN SIGN
    m.insert('\u{0020A7}', 0x9E); // PESETA SIGN
    m.insert('\u{000192}', 0x9F); // LATIN SMALL LETTER F WITH HOOK
    m.insert('\u{0000E1}', 0xA0); // LATIN SMALL LETTER A WITH ACUTE
    m.insert('\u{0000ED}', 0xA1); // LATIN SMALL LETTER I WITH ACUTE
    m.insert('\u{0000F3}', 0xA2); // LATIN SMALL LETTER O WITH ACUTE
    m.insert('\u{0000FA}', 0xA3); // LATIN SMALL LETTER U WITH ACUTE
    m.insert('\u{0000F1}', 0xA4); // LATIN SMALL LETTER N WITH TILDE
    m.insert('\u{0000D1}', 0xA5); // LATIN CAPITAL LETTER N WITH TILDE
    m.insert('\u{0000AA}', 0xA6); // FEMININE ORDINAL INDICATOR
    m.insert('\u{0000BA}', 0xA7); // MASCULINE ORDINAL INDICATOR
    m.insert('\u{0000BF}', 0xA8); // INVERTED QUESTION MARK
    m.insert('\u{002310}', 0xA9); // REVERSED NOT SIGN
    m.insert('\u{0000AC}', 0xAA); // NOT SIGN
    m.insert('\u{0000BD}', 0xAB); // VULGAR FRACTION ONE HALF
    m.insert('\u{0000BC}', 0xAC); // VULGAR FRACTION ONE QUARTER
    m.insert('\u{0000A1}', 0xAD); // INVERTED EXCLAMATION MARK
    m.insert('\u{0000AB}', 0xAE); // LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
    m.insert('\u{0000BB}', 0xAF); // RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
    m.insert('\u{002591}', 0xB0); // LIGHT SHADE
    m.insert('\u{002592}', 0xB1); // MEDIUM SHADE
    m.insert('\u{002593}', 0xB2); // DARK SHADE
    m.insert('\u{002502}', 0xB3); // BOX DRAWINGS LIGHT VERTICAL
    m.insert('\u{002524}', 0xB4); // BOX DRAWINGS LIGHT VERTICAL AND LEFT
    m.insert('\u{002561}', 0xB5); // BOX DRAWINGS VERTICAL SINGLE AND LEFT DOUBLE
    m.insert('\u{002562}', 0xB6); // BOX DRAWINGS VERTICAL DOUBLE AND LEFT SINGLE
    m.insert('\u{002556}', 0xB7); // BOX DRAWINGS DOWN DOUBLE AND LEFT SINGLE
    m.insert('\u{002555}', 0xB8); // BOX DRAWINGS DOWN SINGLE AND LEFT DOUBLE
    m.insert('\u{002563}', 0xB9); // BOX DRAWINGS DOUBLE VERTICAL AND LEFT
    m.insert('\u{002551}', 0xBA); // BOX DRAWINGS DOUBLE VERTICAL
    m.insert('\u{002557}', 0xBB); // BOX DRAWINGS DOUBLE DOWN AND LEFT
    m.insert('\u{00255D}', 0xBC); // BOX DRAWINGS DOUBLE UP AND LEFT
    m.insert('\u{00255C}', 0xBD); // BOX DRAWINGS UP DOUBLE AND LEFT SINGLE
    m.insert('\u{00255B}', 0xBE); // BOX DRAWINGS UP SINGLE AND LEFT DOUBLE
    m.insert('\u{002510}', 0xBF); // BOX DRAWINGS LIGHT DOWN AND LEFT
    m.insert('\u{002514}', 0xC0); // BOX DRAWINGS LIGHT UP AND RIGHT
    m.insert('\u{002534}', 0xC1); // BOX DRAWINGS LIGHT UP AND HORIZONTAL
    m.insert('\u{00252C}', 0xC2); // BOX DRAWINGS LIGHT DOWN AND HORIZONTAL
    m.insert('\u{00251C}', 0xC3); // BOX DRAWINGS LIGHT VERTICAL AND RIGHT
    m.insert('\u{002500}', 0xC4); // BOX DRAWINGS LIGHT HORIZONTAL
    m.insert('\u{00253C}', 0xC5); // BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL
    m.insert('\u{00255E}', 0xC6); // BOX DRAWINGS VERTICAL SINGLE AND RIGHT DOUBLE
    m.insert('\u{00255F}', 0xC7); // BOX DRAWINGS VERTICAL DOUBLE AND RIGHT SINGLE
    m.insert('\u{00255A}', 0xC8); // BOX DRAWINGS DOUBLE UP AND RIGHT
    m.insert('\u{002554}', 0xC9); // BOX DRAWINGS DOUBLE DOWN AND RIGHT
    m.insert('\u{002569}', 0xCA); // BOX DRAWINGS DOUBLE UP AND HORIZONTAL
    m.insert('\u{002566}', 0xCB); // BOX DRAWINGS DOUBLE DOWN AND HORIZONTAL
    m.insert('\u{002560}', 0xCC); // BOX DRAWINGS DOUBLE VERTICAL AND RIGHT
    m.insert('\u{002550}', 0xCD); // BOX DRAWINGS DOUBLE HORIZONTAL
    m.insert('\u{00256C}', 0xCE); // BOX DRAWINGS DOUBLE VERTICAL AND HORIZONTAL
    m.insert('\u{002567}', 0xCF); // BOX DRAWINGS UP SINGLE AND HORIZONTAL DOUBLE
    m.insert('\u{002568}', 0xD0); // BOX DRAWINGS UP DOUBLE AND HORIZONTAL SINGLE
    m.insert('\u{002564}', 0xD1); // BOX DRAWINGS DOWN SINGLE AND HORIZONTAL DOUBLE
    m.insert('\u{002565}', 0xD2); // BOX DRAWINGS DOWN DOUBLE AND HORIZONTAL SINGLE
    m.insert('\u{002559}', 0xD3); // BOX DRAWINGS UP DOUBLE AND RIGHT SINGLE
    m.insert('\u{002558}', 0xD4); // BOX DRAWINGS UP SINGLE AND RIGHT DOUBLE
    m.insert('\u{002552}', 0xD5); // BOX DRAWINGS DOWN SINGLE AND RIGHT DOUBLE
    m.insert('\u{002553}', 0xD6); // BOX DRAWINGS DOWN DOUBLE AND RIGHT SINGLE
    m.insert('\u{00256B}', 0xD7); // BOX DRAWINGS VERTICAL DOUBLE AND HORIZONTAL SINGLE
    m.insert('\u{00256A}', 0xD8); // BOX DRAWINGS VERTICAL SINGLE AND HORIZONTAL DOUBLE
    m.insert('\u{002518}', 0xD9); // BOX DRAWINGS LIGHT UP AND LEFT
    m.insert('\u{00250C}', 0xDA); // BOX DRAWINGS LIGHT DOWN AND RIGHT
    m.insert('\u{002588}', 0xDB); // FULL BLOCK
    m.insert('\u{002584}', 0xDC); // LOWER HALF BLOCK
    m.insert('\u{00258C}', 0xDD); // LEFT HALF BLOCK
    m.insert('\u{002590}', 0xDE); // RIGHT HALF BLOCK
    m.insert('\u{002580}', 0xDF); // UPPER HALF BLOCK
    m.insert('\u{0003B1}', 0xE0); // GREEK SMALL LETTER ALPHA
    m.insert('\u{0000DF}', 0xE1); // LATIN SMALL LETTER SHARP S
    m.insert('\u{000393}', 0xE2); // GREEK CAPITAL LETTER GAMMA
    m.insert('\u{0003C0}', 0xE3); // GREEK SMALL LETTER PI
    m.insert('\u{0003A3}', 0xE4); // GREEK CAPITAL LETTER SIGMA
    m.insert('\u{0003C3}', 0xE5); // GREEK SMALL LETTER SIGMA
    m.insert('\u{0000B5}', 0xE6); // MICRO SIGN
    m.insert('\u{0003C4}', 0xE7); // GREEK SMALL LETTER TAU
    m.insert('\u{0003A6}', 0xE8); // GREEK CAPITAL LETTER PHI
    m.insert('\u{000398}', 0xE9); // GREEK CAPITAL LETTER THETA
    m.insert('\u{0003A9}', 0xEA); // GREEK CAPITAL LETTER OMEGA
    m.insert('\u{0003B4}', 0xEB); // GREEK SMALL LETTER DELTA
    m.insert('\u{00221E}', 0xEC); // INFINITY
    m.insert('\u{0003C6}', 0xED); // GREEK SMALL LETTER PHI
    m.insert('\u{0003B5}', 0xEE); // GREEK SMALL LETTER EPSILON
    m.insert('\u{002229}', 0xEF); // INTERSECTION
    m.insert('\u{002261}', 0xF0); // IDENTICAL TO
    m.insert('\u{0000B1}', 0xF1); // PLUS-MINUS SIGN
    m.insert('\u{002265}', 0xF2); // GREATER-THAN OR EQUAL TO
    m.insert('\u{002264}', 0xF3); // LESS-THAN OR EQUAL TO
    m.insert('\u{002320}', 0xF4); // TOP HALF INTEGRAL
    m.insert('\u{002321}', 0xF5); // BOTTOM HALF INTEGRAL
    m.insert('\u{0000F7}', 0xF6); // DIVISION SIGN
    m.insert('\u{002248}', 0xF7); // ALMOST EQUAL TO
    m.insert('\u{0000B0}', 0xF8); // DEGREE SIGN
    m.insert('\u{002219}', 0xF9); // BULLET OPERATOR
    m.insert('\u{0000B7}', 0xFA); // MIDDLE DOT
    m.insert('\u{00221A}', 0xFB); // SQUARE ROOT
    m.insert('\u{00207F}', 0xFC); // SUPERSCRIPT LATIN SMALL LETTER N
    m.insert('\u{0000B2}', 0xFD); // SUPERSCRIPT TWO
    m.insert('\u{0025A0}', 0xFE); // BLACK SQUARE
    m.insert('\u{0000A0}', 0xFF); // NO-BREAK SPACE
    m
});

pub fn unicode_to_cp437<S: AsRef<str>>(input: S) -> Vec<u32> {
    let input = input.as_ref();
    input
        .chars()
        .map(|c| {
            if let Some(&byte) = CP437.get(&c) {
                byte
            } else if c.is_ascii() {
                c as u32
            } else {
                // If the character is not in the CP437 mapping and not an ascii byte,
                // use a placeholder (e.g., '?')
                '?' as u32
            }
        })
        .collect()
}

/*************
* Example
    specs.extend(watch_text_for_display!(
        settings,
        "/tmp/viz/dino_frame.txt",
        "dino",
        "dino_glitch_mix",
        "dino_frame",
        "dino_frame_starts",
        "dino_frame_lens"
    ));
*
*/

pub static WATCHERS: LazyLock<Mutex<RefCell<HashMap<String, TextFileLoader>>>> =
    LazyLock::new(|| Mutex::new(RefCell::new(HashMap::new())));

#[macro_export]
macro_rules! watch_text_for_display {
    ($settings:expr, $path:expr, $playback:expr, $target:expr, $text_var:expr, $starts_var:expr, $lens_var:expr) => {{
        let lock = vizwasm::vizconfig::WATCHERS
            .lock()
            .expect("WATCHERS mutex corrupted");
        let mut map = lock.borrow_mut();
        let entry = map
            .entry($path.to_string())
            .or_insert_with(|| TextFileLoader::new($path));

        if $settings.is_playback_reset($playback) {
            entry.reset();
        }
        eprintln!("watch_text_for_display! refreshing {} {:#?}", $path, entry);

        if entry.refresh() {
            DisplayText::from(entry.data()).get_specs($target, $text_var, $starts_var, $lens_var)
        } else {
            vec![]
        }
    }};
}
