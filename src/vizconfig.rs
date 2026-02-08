use sdlrig::gfxinfo::MidiEvent;
#[allow(unused_imports)]
use sdlrig::gfxinfo::{MIDI_CONTROL_CHANGE, MIDI_NOTE_OFF, MIDI_NOTE_ON};
use sdlrig::renderspec::{CopyEx, HudText, Mix, MixInput, Reset, SendCmd, SendMidi, SendValue};
use sdlrig::{
    gfxinfo::{Asset, GfxEvent, KeyCode, KeyEvent, Knob, Vid, VidInfo, VidMixer},
    renderspec::RenderSpec,
};

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::iter::repeat;
use std::sync::LazyLock;
use std::{error::Error, i64, io::Write};

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
    pub fn add_header<T: AsRef<str>>(&mut self, header: T) {
        let h = header.as_ref();
        let o = self.def.header.as_deref().unwrap_or_default();
        self.def.header.replace([o, h].join("\n"));
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
                    .header(concat!(
                        include_str!("glsl/utils.glsl"),
                        "\n",
                        include_str!("glsl/patch_rototrans.glsl"),
                        "\n",
                        include_str!("glsl/patch_feedback.glsl"),
                        "\n",
                    ))
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
    pub feedback_modes: Vec<String>,
    pub colors: Vec<(String, String)>,
    pub asset_path: String,
    pub active_idx: usize,
    pub display_idx: usize,
    pub scan_idx: usize,
    pub clipboard: StreamSettings,
    pub selected_knobs: usize,
    pub playback: Vec<PlaybackSettings>,
    pub initial_reset_complete: Vec<bool>,
    pub logs: Vec<String>,
    pub last_line: String,
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
        &["basic", "jam", "math", "xor"]
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
                        .resolution((640, 480))
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
                        .width(640)
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
                    .header(concat!(
                        include_str!("glsl/utils.glsl"),
                        "\n",
                        include_str!("glsl/strings.glsl"),
                        "\n",
                    ))
                    .body(include_str!("glsl/wireframe.glsl"))
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
                    .header(concat!(
                        include_str!("glsl/utils.glsl"),
                        "\n",
                        include_str!("glsl/strings.glsl"),
                        "\n",
                    ))
                    .body(include_str!("glsl/logs.glsl"))
                    .build(),
                mix: Mix::builder()
                    .name("logs_mix")
                    .mixed("blank_mix")
                    .no_display(true)
                    .build(),
            },
        );

        //config for status
        mix_configs.insert(
            "status_mix".to_string(),
            MixConfig {
                def: VidMixer::builder()
                    .name("status_mix")
                    .width(640)
                    .height(480)
                    .header(concat!(
                        include_str!("glsl/utils.glsl"),
                        "\n",
                        include_str!("glsl/strings.glsl"),
                        "\n",
                    ))
                    .body(include_str!("glsl/status.glsl"))
                    .build(),
                mix: Mix::builder()
                    .name("status_mix")
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

            let pb = PlaybackSettings {
                was_reset: true,
                stream: StreamSettings::new(StreamIdent {
                    name: name.clone(),
                    first_video: first_video.clone(),
                    input_mix: input_mix.def.name.clone(),
                    main_mix: mixer_graph.main_mix.def.name.clone(),
                    feedback_mix: mixer_graph.feedback.def.name.clone(),
                    overlay_mix: mixer_graph.overlay.def.name.clone(),
                }),
                presets: PresetSettings {
                    baseline: StreamSettings::new(StreamIdent {
                        name: name.clone(),
                        first_video: first_video.clone(),
                        input_mix: input_mix.def.name.clone(),
                        main_mix: mixer_graph.main_mix.def.name.clone(),
                        feedback_mix: mixer_graph.feedback.def.name.clone(),
                        overlay_mix: mixer_graph.overlay.def.name.clone(),
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

        specs.push(
            HudText {
                text: self.hud(&VidInfo::default()),
            }
            .into(),
        );

        // WIRE FRAME DATA
        let page = self.selected_knobs as u8 / 16;
        for i in 0..16 {
            let cc = i + (16 * page);
            let button = i + 1;
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
                        n = n.clamp(0.0, (self.distort_names.len() - 1) as f64);
                        Some(self.distort_names[n as usize].0.clone())
                    } else if field == StreamSettingsField::WarpScan {
                        n = n.clamp(0.0, (self.distort_names.len() - 1) as f64);
                        Some(self.distort_names[n as usize].1.clone())
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

            let txt = label
                .bytes()
                .map(|b| b as i32)
                .chain(repeat(0).take((128 - label.len()).clamp(0, 128)))
                .collect::<Vec<_>>();

            specs.push(
                SendCmd::builder()
                    .mix("wireframe_data_mix")
                    .name(format!("button{button}_len"))
                    .value(SendValue::Integer(label.len() as i32))
                    .build()
                    .into(),
            );
            specs.push(
                SendCmd::builder()
                    .mix("wireframe_data_mix")
                    .name(format!("button{button}_txt"))
                    .value(SendValue::IVector(txt))
                    .build()
                    .into(),
            );

            let txt = if let Some(mut extra) = extra.clone() {
                extra.truncate(11);
                extra
                    .bytes()
                    .map(|b| b as i32)
                    .chain(repeat(0).take((128 - extra.len()).clamp(0, 128)))
                    .collect::<Vec<_>>()
            } else {
                val.bytes()
                    .map(|b| b as i32)
                    .chain(repeat(0).take((128 - val.len()).clamp(0, 128)))
                    .collect::<Vec<_>>()
            };

            specs.push(
                SendCmd::builder()
                    .mix("wireframe_data_mix")
                    .name(format!("button{button}_val_len"))
                    .value(SendValue::Integer(val.len() as i32))
                    .build()
                    .into(),
            );
            specs.push(
                SendCmd::builder()
                    .mix("wireframe_data_mix")
                    .name(format!("button{button}_val"))
                    .value(SendValue::IVector(txt))
                    .build()
                    .into(),
            );
        }

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
                                    tween: self.playback[selected_idx]
                                        .stream
                                        .get_field(&StreamSettingsField::Tween)
                                        >= 0.0,
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
                                        self.playback[i]
                                            .stream
                                            .set_field(StreamSettingsField::ExactSec, 0.01);
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
                        // KeyEvent {
                        //     key: KeyCode::SDLK_9,
                        //     shift,
                        //     down: true,
                        //     ..
                        // } => {
                        //     self.adjust(Knob::L, *shift, -1.0);
                        // }
                        // KeyEvent {
                        //     key: KeyCode::SDLK_0,
                        //     shift,
                        //     down: true,
                        //     ..
                        // } => {
                        //     self.adjust(Knob::L, *shift, 1.0);
                        // }
                        // KeyEvent {
                        //     key: KeyCode::SDLK_MINUS,
                        //     shift,
                        //     down: true,
                        //     ..
                        // } => {
                        //     self.adjust(Knob::CL, *shift, 1.0);
                        // }
                        // KeyEvent {
                        //     key: KeyCode::SDLK_LEFTBRACKET,
                        //     shift,
                        //     down: true,
                        //     ..
                        // } => {
                        //     self.adjust(Knob::R, *shift, -1.0);
                        // }
                        // KeyEvent {
                        //     key: KeyCode::SDLK_RIGHTBRACKET,
                        //     shift,
                        //     down: true,
                        //     ..
                        // } => {
                        //     self.adjust(Knob::R, *shift, 1.0);
                        // }
                        // KeyEvent {
                        //     key: KeyCode::SDLK_BACKSLASH,
                        //     shift,
                        //     down: true,
                        //     ..
                        // } => {
                        //     self.adjust(Knob::CR, *shift, 1.0);
                        // }
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

    pub fn hud(&self, _vid_info: &VidInfo) -> String {
        return String::new();
        // let mut hud_txt = vec![];
        // let mut fields = vec![];
        // for f in streamsettings::ALL_FIELDS {
        //     if let Some(props) = f.properties() {
        //         if props.cc.is_some() {
        //             fields.push((
        //                 props.cc.unwrap_or_default(),
        //                 props.channel.unwrap_or_default(),
        //                 props.label.unwrap_or_default().to_string(),
        //                 self.playback[self.active_idx].stream.get_field(f),
        //             ));
        //         }
        //     }
        // }

        // fields.sort_by(|a, b| {
        //     a.0.cmp(&b.0)
        //         .then(a.1.cmp(&b.1))
        //         .then(a.2.cmp(&b.2))
        //         .then(a.3.total_cmp(&b.3))
        // });
        // for (cc, ch, label, value) in fields {
        //     hud_txt.push(format!(
        //         "{} [{:02}:{:03}] {:20}: {:10.4}",
        //         if self.selected_knobs as u8 == cc {
        //             ">"
        //         } else {
        //             " "
        //         },
        //         ch,
        //         cc,
        //         label,
        //         value
        //     ));
        // }
        // hud_txt.push(format!(
        //     "Active: {} | Displayed: {} | Scan: {}",
        //     self.playback_names[self.active_idx],
        //     self.playback_names[self.display_idx],
        //     self.playback_names[self.scan_idx],
        // ));
        // hud_txt.push(format!("Selected {}", self.selected_knobs));
        // hud_txt.push(format!("Duration: {}", vid_info.duration()));
        // hud_txt.push(format!(
        //     "Timecode: {:02}:{:02}:{:02}:{:02}",
        //     {
        //         let tc = self.playback[self.active_idx].stream.real_ts.0 as f64
        //             / self.playback[self.active_idx].stream.real_ts.1 as f64;
        //         (tc / 3600.0).floor() as u32
        //     },
        //     {
        //         let tc = self.playback[self.active_idx].stream.real_ts.0 as f64
        //             / self.playback[self.active_idx].stream.real_ts.1 as f64;
        //         ((tc % 3600.0) / 60.0).floor() as u32
        //     },
        //     {
        //         let tc = self.playback[self.active_idx].stream.real_ts.0 as f64
        //             / self.playback[self.active_idx].stream.real_ts.1 as f64;
        //         (tc % 60.0).floor() as u32
        //     },
        //     {
        //         let tc = self.playback[self.active_idx].stream.real_ts.0 as f64
        //             / self.playback[self.active_idx].stream.real_ts.1 as f64;
        //         ((tc - tc.floor()) * 24.0 as f64).round() as u32
        //     },
        // ));
        // hud_txt.push(format!(
        //     "ts: {:-6}/{:-6} = {:.3})",
        //     self.playback[self.active_idx].stream.real_ts.0,
        //     self.playback[self.active_idx].stream.real_ts.1,
        //     {
        //         let tc = self.playback[self.active_idx].stream.real_ts.0 as f64
        //             / self.playback[self.active_idx].stream.real_ts.1 as f64;
        //         tc
        //     },
        // ));
        // hud_txt.push(format!(
        //     "Pb {:?}\nLoop {}",
        //     self.playback[self.active_idx].loops.playing,
        //     self.playback[self.active_idx].loops.selected_loop
        // ));
        // hud_txt.join("\n")
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
                    let (warp_x, warp_y) = self.distort_names[self.playback[fidx]
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
    ( $all_settings:expr, $midi_event:expr, $bg_name:expr, $combo_name:expr, $time_codes:expr) => {
        static BG_IDX: std::sync::LazyLock<Option<usize>> = std::sync::LazyLock::new(|| {
            let mut idx = None;
            for i in 0..PLAYBACK_NAMES.len() {
                if PLAYBACK_NAMES[i] == $bg_name {
                    idx.replace(i);
                    break;
                }
            }
            idx
        });

        static COMBO_IDX: std::sync::LazyLock<Option<usize>> = std::sync::LazyLock::new(|| {
            let mut idx = None;
            for i in 0..PLAYBACK_NAMES.len() {
                if PLAYBACK_NAMES[i] == $combo_name {
                    idx.replace(i);
                    break;
                }
            }
            idx
        });

        if BG_IDX.is_none() || COMBO_IDX.is_none() {
            return;
        }

        static TIME_IDX: std::sync::LazyLock<Mutex<std::cell::RefCell<usize>>> =
            std::sync::LazyLock::new(|| Mutex::new(std::cell::RefCell::new(0)));
        if let (Some(bg_idx), Some(combo_idx)) = (*BG_IDX, *COMBO_IDX) {
            if $all_settings.active_idx != bg_idx
                && $all_settings.display_idx != bg_idx
                && $all_settings.active_idx != combo_idx
                && $all_settings.display_idx != combo_idx
            {
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
                (IAC, 0, MIDI_CONTROL_CHANGE, 0, v) => {
                    if v > 10 {
                        let lock = TIME_IDX.lock().unwrap();
                        let mut idx = lock.borrow_mut();

                        let mut next_idx = rand::random::<u32>() % $time_codes.len() as u32;
                        if next_idx == *idx as u32 {
                            next_idx = (next_idx + 1) % $time_codes.len() as u32;
                        }
                        *idx = next_idx as usize;
                        $all_settings.playback[bg_idx].stream.set_field(
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

const IAC: &str = "IAC Driver Bus 1";
const IAC_GLSL: &str = "iac_driver_bus_1";
const MFT: &str = "Midi Fighter Twister";
const MFT_GLSL: &str = "midi_fighter_twister";
const MPK: &str = "MPK mini 3";
const MPK_GLSL: &str = "mpk_mini_3";

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
        if event.device != MFT || event.kind != MIDI_CONTROL_CHANGE {
            return;
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
}
