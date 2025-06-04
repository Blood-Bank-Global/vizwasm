use std::{
    collections::HashMap,
    error::Error,
    sync::{LazyLock, Mutex},
};

use sdlrig::{
    gfxinfo::{Asset, GfxEvent, GfxInfo, Vid, VidInfo, VidMixer},
    hud_text,
    renderspec::{CopyEx, Mix, RenderSpec},
    reset,
};
use serde::{Deserialize, Serialize};
use vizwasm::vizconfig::{AllSettings, GlobalNameAccessors, LoopEvent, MixConfig, StreamSettings};
fn main() {}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TechNameAccessors {}

impl GlobalNameAccessors for TechNameAccessors {
    fn stream_defs() -> &'static [Vid] {
        static STREAM_DEFS: LazyLock<Vec<Vid>> = LazyLock::new(|| {
            vec![Vid::builder()
                .name("blank")
                .path(&format!(
                    "{}/streams/blank.mp4",
                    TechNameAccessors::asset_path()
                ))
                .resolution((720, 480))
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build()
                .into()]
        });
        &STREAM_DEFS
    }

    fn mix_configs() -> &'static [vizwasm::vizconfig::MixConfig] {
        static MIX_DEFS: LazyLock<Vec<MixConfig>> = LazyLock::new(|| {
            vec![MixConfig {
                def: VidMixer::builder()
                    .name("blood")
                    .header(concat!(
                        include_str!("../glsl/utils.glsl"),
                        "\n",
                        include_str!("../glsl/blood_funcs.glsl")
                    ))
                    .body(include_str!("../glsl/blood.glsl"))
                    .width(720)
                    .height(480)
                    .build(),
                mix: Mix::builder()
                    .name("blood")
                    .video("blank")
                    .no_display(true)
                    .build(),
            }]
        });
        &MIX_DEFS
    }

    fn playback_mixes() -> &'static [&'static str] {
        static PLAYBACK_MIXES: LazyLock<Vec<&'static str>> = LazyLock::new(|| vec!["blood"]);
        &PLAYBACK_MIXES
    }

    fn overlay_names() -> &'static [&'static str] {
        &[
            "blank",
            "vhs_overlay",
            "film_dust",
            "tracking",
            "colorful",
            "bottom",
        ]
    }

    fn distort_names() -> &'static [(&'static str, &'static str)] {
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

    fn distort_edge_types() -> &'static [&'static str] {
        &["smear", "wrap", "mirror", "blank"]
    }
    fn lut_names() -> &'static [&'static str] {
        &[
            "none",
            "rad",
            "midas",
            "blackwhite",
            "blue",
            "redzone",
            "riso",
            "plague",
            "hyper",
            "sepia",
        ]
    }

    fn blend_modes() -> &'static [&'static str] {
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

    fn colors() -> &'static [(&'static str, &'static str)] {
        &[
            ("black", "0x000000"),
            ("defined", "6e9ea2"),
            ("white", "0xffffff"),
            ("lightgray", "0x808080"),
            ("darkgray", "0x0f0f0f"),
            ("red", "0xff0000"),
            ("green", "0x00ff00"),
            ("blue", "0x00007f"),
            ("darkred", "0x7f0000"),
            ("darkgreen", "0x007f00"),
            ("darkblue", "0x00007f"),
            ("orange", "0x7f3f00"),
        ]
    }

    fn asset_path() -> &'static str {
        "/Users/ttie/Desktop/tech"
    }
}

lazy_static::lazy_static! {
    static ref SETTINGS: Mutex<Box<AllSettings<TechNameAccessors>>> = Mutex::new(Box::new(AllSettings::new()));
}

#[no_mangle]
#[allow(unused)]
pub fn asset_list(fps: i64) -> Vec<Asset> {
    let mut lock = if let Ok(lock) = SETTINGS.lock() {
        lock
    } else {
        panic!("Settings object has been corrupted.");
    };

    let settings = lock.as_mut();
    settings.asset_list(fps)
}

#[no_mangle]
pub fn encode_settings() -> Vec<u8> {
    let mut lock = SETTINGS.lock().expect("Could not get settings lock.");
    let settings = lock.as_mut();
    serde_json::to_vec(settings).unwrap()
}

#[no_mangle]
pub fn decode_settings(bytes: &[u8]) {
    let mut lock = SETTINGS.lock().expect("Could not get settings lock.");
    let settings = lock.as_mut();
    *settings = serde_json::from_slice(bytes).unwrap();
}

#[no_mangle]
pub fn calculate(
    #[allow(unused)] canvas_w: u32,
    #[allow(unused)] canvas_h: u32,
    #[allow(unused)] frame: i64,
    #[allow(unused)] fps: i64,
    #[allow(unused)] gfx_info: &HashMap<String, GfxInfo>,
    #[allow(unused)] reg_events: &[GfxEvent],
) -> Result<Vec<RenderSpec>, Box<dyn Error>> {
    let mut lock = SETTINGS.lock().expect("Settings mutex corrupted");
    let settings = lock.as_mut();

    static ONCE: std::sync::Once = std::sync::Once::new();

    let mut specs = vec![];
    ONCE.call_once(|| {
        settings.initial_reset_complete = vec![false; TechNameAccessors::stream_defs().len()];
    });

    //Update steams for incoming frame events first
    for event in reg_events {
        match event {
            GfxEvent::FrameEvent(fe) => {
                if let Some(eidx) = settings.lookup_stream(&fe.stream) {
                    settings.playback[eidx].stream.real_ts = fe.real_ts;
                    settings.playback[eidx].stream.continuous_ts = fe.continuous_ts;
                }
            }
            _ => (),
        }
    }

    let orig = if reg_events.contains(&GfxEvent::ReloadEvent()) {
        let mut tmp = settings.playback.clone();
        for i in 0..tmp.len() {
            tmp[i].stream.reset();
        }
        tmp
    } else {
        settings.playback.clone()
    };

    // Always capture live events even while recording is playing
    settings.update(reg_events, frame)?;
    for i in 0..settings.playback.len() {
        let diffs = orig[i]
            .stream
            .diff(&settings.playback[i].stream)
            .into_iter()
            .collect::<Vec<_>>();
        specs.append(&mut settings.playback[i].stream.get_commands(&diffs));
        if let Some(buf) = settings.playback[i].loops.record_buffer.as_mut() {
            let filtered_diffs = diffs
                .into_iter()
                .filter(|d| StreamSettings::<TechNameAccessors>::should_record(d))
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

    for i in 0..settings.playback.len() {
        for j in 0..settings.playback[i].loops.saved.len() {
            if settings.playback[i].loops.playing[j] {
                //send events for recorded loop at this frame
                let lp = &settings.playback[i].loops.saved[j];
                if lp.events.len() > 0 {
                    let start = lp.events[0].frame;
                    let lp_len = lp.end - start;
                    let curr = (frame % lp_len) + start;
                    for event in &lp.events {
                        if event.frame == curr {
                            let diffs = event.diffs.clone();
                            if diffs.len() > 0 {
                                settings.playback[i].stream.apply_diff(&diffs);
                                specs.append(&mut settings.playback[i].stream.get_commands(&diffs));
                            }

                            break;
                        }
                    }
                }
            }
        }
    }

    for i in 0..TechNameAccessors::stream_defs().len() {
        if !(i == settings.display_idx || i == settings.active_idx)
            && settings.initial_reset_complete[i] == true
        {
            eprintln!("unloading {i}");
            settings.initial_reset_complete[i] = false;
            specs.push(reset!(settings.playback[i].stream.main_mix()));
        }
    }

    let vid_info = if let Some(vid_name) = settings.playback[settings.display_idx]
        .stream
        .find_first_video()
    {
        if let Some(GfxInfo::VidInfo(vid_info)) = gfx_info.get(&vid_name) {
            vid_info
        } else {
            &VidInfo::default()
        }
    } else {
        &VidInfo::default()
    };

    specs.push(hud_text!(settings.hud(vid_info)));

    let mut videos = HashMap::new();
    videos.extend(videos_to_cache(
        &settings.playback[settings.active_idx].stream,
    ));
    videos.extend(videos_to_cache(
        &settings.playback[settings.display_idx].stream,
    ));

    specs.extend(videos.values().map(|s| s.clone().into()));

    // LHS
    let mix_name = TechNameAccessors::playback_mixes()[settings.display_idx];
    let mix_def = TechNameAccessors::mix_configs()
        .iter()
        .find(|m| m.def.name == mix_name)
        .map(|c| c.def.clone())
        .expect("Mix config not found for display index");

    let iw = mix_def.width as i32;
    let ih = mix_def.height as i32;
    let mut ow = iw;
    let mut oh = ih;
    let mut ix = 0;
    let mut iy = 0;

    let iaspect = iw as f32 / ih as f32;
    let oaspect = canvas_w as f32 / canvas_h as f32;

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
    let target = CopyEx::builder()
        .src((ix, iy, ow as u32, oh as u32))
        .dst((0, 0, canvas_w as u32, canvas_h as u32))
        .build();
    specs.extend(show_stream(settings.active_idx, settings, target));

    // RESET SEEK
    for i in 0..settings.playback.len() {
        settings.playback[i].stream.set_delta_sec(0.0);
        settings.playback[i].stream.set_scrub(0.0);
        settings.playback[i].stream.set_exact_sec(0.0);
    }
    Ok(specs)
}
