use std::{
    collections::HashMap,
    error::Error,
    sync::{LazyLock, Mutex},
};

use sdlrig::{
    gfxinfo::{Asset, GfxEvent, GfxInfo, Vid, VidInfo},
    hud_text,
    renderspec::{CopyEx, Mix, RenderSpec},
    reset,
};
use serde::{Deserialize, Serialize};
use vizwasm::vizconfig::{AllSettings, GlobalNameAccessors, LoopEvent, StreamSettings};
fn main() {}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TechNameAccessors {}

impl GlobalNameAccessors for TechNameAccessors {
    fn stream_defs() -> &'static [Vid] {
        static STREAM_DEFS: LazyLock<Vec<Vid>> = LazyLock::new(|| {
            vec![
                Vid::builder()
                    .name("blank")
                    .path(&format!(
                        "{}/streams/blank.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build(),
                Vid::builder()
                    .name("generate")
                    .path(&format!(
                        "{}/streams/blank.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build(),
                Vid::builder()
                    .name("blood")
                    .path(&format!(
                        "{}/streams/blank.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build(),
                Vid::builder()
                    .name("blob")
                    .path(&format!(
                        "{}/streams/blob.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build(),
                Vid::builder()
                    .name("blur_lights")
                    .path(&format!(
                        "{}/streams/blur_lights.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("burns")
                    .path(&format!(
                        "{}/streams/burns.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("circles")
                    .path(&format!(
                        "{}/streams/circles.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("circuit1")
                    .path(&format!(
                        "{}/streams/circuit1.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("circuit2")
                    .path(&format!(
                        "{}/streams/circuit2.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("clock")
                    .path(&format!(
                        "{}/streams/clock.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("corp")
                    .path(&format!(
                        "{}/streams/corp.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("cube")
                    .path(&format!(
                        "{}/streams/cube.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("dna")
                    .path(&format!(
                        "{}/streams/dna.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("epic")
                    .path(&format!(
                        "{}/streams/epic.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("ether")
                    .path(&format!(
                        "{}/streams/ether.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("volume")
                    .path(&format!(
                        "{}/streams/volume.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("fluffy_clouds")
                    .path(&format!(
                        "{}/streams/fluffy_clouds.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("holo_city")
                    .path(&format!(
                        "{}/streams/holo_city.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("masque")
                    .path(&format!(
                        "{}/streams/masque.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("night_sky")
                    .path(&format!(
                        "{}/streams/night_sky.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("nyc")
                    .path(&format!(
                        "{}/streams/nyc.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("old_clouds")
                    .path(&format!(
                        "{}/streams/old_clouds.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("oldny")
                    .path(&format!(
                        "{}/streams/oldny.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("phone")
                    .path(&format!(
                        "{}/streams/phone.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("silver_lining")
                    .path(&format!(
                        "{}/streams/silver_lining.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("stars")
                    .path(&format!(
                        "{}/streams/stars.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("target")
                    .path(&format!(
                        "{}/streams/target.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("tv")
                    .path(&format!(
                        "{}/streams/tv.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("vestial1")
                    .path(&format!(
                        "{}/streams/vestial1.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("vestial2")
                    .path(&format!(
                        "{}/streams/vestial2.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("world_grid")
                    .path(&format!(
                        "{}/streams/world_grid.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("the_moon")
                    .path(&format!(
                        "{}/streams/the_moon.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("tube")
                    .path(&format!(
                        "{}/streams/tube.mp4",
                        TechNameAccessors::asset_path()
                    ))
                    .resolution((720, 480))
                    .tbq((1, 12800))
                    .pix_fmt("yuv420p")
                    .repeat(true)
                    .realtime(false)
                    .hardware_decode(true)
                    .build()
                    .into(),
                Vid::builder()
                    .name("front cam")
                    .path("MacBook Pro Camera")
                    .format("avfoundation")
                    .opts(&vec![
                        ("pixel_format", "bgr0"),
                        ("framerate", "30.0"),
                        ("video_size", "1280x720"),
                    ])
                    .resolution((1280, 720))
                    .tbq((1, 1000000))
                    .pix_fmt("bgr0")
                    .repeat(false)
                    .realtime(true)
                    .hardware_decode(false)
                    .build()
                    .into(),
                // Vid::builder()
                //     .name("liquid_purple")
                //     .path(&format!(
                //         "{}/streams/liquid_purple.mp4",
                //         TechNameAccessors::asset_path()
                //     ))
                //     .resolution((720, 480))
                //     .tbq((1, 12800))
                //     .pix_fmt("yuv420p")
                //     .repeat(true)
                //     .realtime(false)
                //     .hardware_decode(true)
                //     .build()
                //     .into(),
                // Vid::builder()
                //     .name("liquid_gold")
                //     .path(&format!(
                //         "{}/streams/liquid_gold.mp4",
                //         TechNameAccessors::asset_path()
                //     ))
                //     .resolution((720, 480))
                //     .tbq((1, 12800))
                //     .pix_fmt("yuv420p")
                //     .repeat(true)
                //     .realtime(false)
                //     .hardware_decode(true)
                //     .build()
                //     .into(),
                // Vid::builder()
                //     .name("liquid_lava")
                //     .path(&format!(
                //         "{}/streams/liquid_lava.mp4",
                //         TechNameAccessors::asset_path()
                //     ))
                //     .resolution((720, 480))
                //     .tbq((1, 12800))
                //     .pix_fmt("yuv420p")
                //     .repeat(true)
                //     .realtime(false)
                //     .hardware_decode(true)
                //     .build()
                //     .into(),
                // Vid::builder()
                //     .name("bootleg_liberty")
                //     .path(&format!(
                //         "{}/streams/bootleg_liberty.mp4",
                //         TechNameAccessors::asset_path()
                //     ))
                //     .resolution((720, 480))
                //     .tbq((1, 12800))
                //     .pix_fmt("yuv420p")
                //     .repeat(true)
                //     .realtime(false)
                //     .hardware_decode(true)
                //     .build()
                //     .into(),
                // Vid::builder()
                //     .name("smoke")
                //     .path(&format!(
                //         "{}/streams/smoke.mp4",
                //         TechNameAccessors::asset_path()
                //     ))
                //     .resolution((720, 480))
                //     .tbq((1, 12800))
                //     .pix_fmt("yuv420p")
                //     .repeat(true)
                //     .realtime(false)
                //     .hardware_decode(true)
                //     .build()
                //     .into(),
                // Vid::builder()
                //     .name("ground_ctl")
                //     .path(&format!(
                //         "{}/streams/ground_ctl.mp4",
                //         TechNameAccessors::asset_path()
                //     ))
                //     .resolution((720, 480))
                //     .tbq((1, 12800))
                //     .pix_fmt("yuv420p")
                //     .repeat(true)
                //     .realtime(false)
                //     .hardware_decode(true)
                //     .build()
                //     .into(),
                // Vid::builder()
                //     .name("fall")
                //     .path(&format!(
                //         "{}/streams/fall.mp4",
                //         TechNameAccessors::asset_path()
                //     ))
                //     .resolution((720, 480))
                //     .tbq((1, 12800))
                //     .pix_fmt("yuv420p")
                //     .repeat(true)
                //     .realtime(false)
                //     .hardware_decode(true)
                //     .build()
                //     .into(),
                // Vid::builder()
                //     .name("horizon")
                //     .path(&format!(
                //         "{}/streams/horizon.mp4",
                //         TechNameAccessors::asset_path()
                //     ))
                //     .resolution((720, 480))
                //     .tbq((1, 12800))
                //     .pix_fmt("yuv420p")
                //     .repeat(true)
                //     .realtime(false)
                //     .hardware_decode(true)
                //     .build()
                //     .into(),
                // Vid::builder()
                //     .name("capture")
                //     .path("USB3 Video")
                //     .format("avfoundation")
                //     .resolution((1280, 720))
                //     .tbq((1, 1000000))
                //     .pix_fmt("bgr0")
                //     .opts(&vec![
                //         ("pixel_format", "bgr0"),
                //         ("framerate", "30.0"),
                //         ("video_size", "1280x720"),
                //     ])
                //     .realtime(true)
                //     .repeat(false)
                //     .build(),
                // Vid::builder()
                //     .name("StreamCam")
                //     .path("Logitech StreamCam")
                //     .format("avfoundation")
                //     .resolution((848, 480))
                //     .tbq((1, 1000000))
                //     .pix_fmt("bgr0")
                //     .opts(&vec![
                //         ("pixel_format", "bgr0"),
                //         ("framerate", "30.0"),
                //         ("video_size", "848,480"),
                //     ])
                //     .realtime(true)
                //     .repeat(false)
                //     .build(),
            ]
        });
        &STREAM_DEFS
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

    let vid_info = if let Some(GfxInfo::VidInfo(vid_info)) = gfx_info.get(
        &settings.playback[settings.display_idx]
            .stream
            .base_stream()
            .to_string(),
    ) {
        vid_info
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
    let vid_info = if let Some(GfxInfo::VidInfo(vid_info)) = gfx_info.get(
        &settings.playback[settings.active_idx]
            .stream
            .base_stream()
            .to_string(),
    ) {
        vid_info
    } else {
        &VidInfo::default()
    };

    let iw = vid_info.size.0 as i32;
    let ih = vid_info.size.1 as i32;
    let mut ow = iw;
    let mut oh = ih;
    let mut ix = 0;
    let mut iy = 0;

    let iaspect = iw as f32 / ih as f32;
    let oaspect = (canvas_w / 2) as f32 / canvas_h as f32;

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
        .dst((0, 0, canvas_w / 2 as u32, canvas_h as u32))
        .build();
    specs.extend(show_stream(settings.active_idx, settings, target));

    // RHS
    let vid_info = if let Some(GfxInfo::VidInfo(vid_info)) = gfx_info.get(
        &settings.playback[settings.display_idx]
            .stream
            .base_stream()
            .to_string(),
    ) {
        vid_info
    } else {
        &VidInfo::default()
    };
    let iw = vid_info.size.0 as i32;
    let ih = vid_info.size.1 as i32;
    let mut ow = iw;
    let mut oh = ih;
    let mut ix = 0;
    let mut iy = 0;

    let iaspect = iw as f32 / ih as f32;
    let oaspect = (canvas_w / 2) as f32 / canvas_h as f32;

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
        .dst((canvas_w as i32 / 2, 0, canvas_w / 2 as u32, canvas_h as u32))
        .build();

    specs.extend(show_stream(settings.display_idx, settings, target));

    // RESET SEEK
    for i in 0..settings.playback.len() {
        settings.playback[i].stream.set_delta_sec(0.0);
        settings.playback[i].stream.set_scrub(0.0);
        settings.playback[i].stream.set_exact_sec(0.0);
    }
    Ok(specs)
}

pub fn videos_to_cache<T>(stream_settings: &StreamSettings<T>) -> Vec<(String, Mix)>
where
    T: GlobalNameAccessors,
{
    let mut videos = vec![];

    if stream_settings.pause() == 0
        || stream_settings.exact_sec() != 0.0
        || stream_settings.scrub() != 0.0
        || stream_settings.delta_sec() != 0.0
    {
        let mut builder = Mix::builder()
            .name(stream_settings.base_cache())
            .video(stream_settings.base_stream())
            .no_display(true);

        let lut = TechNameAccessors::lut_names()[stream_settings.lut_selected() as usize];
        if lut != "none" {
            builder = builder.lut(format!(
                "{}/luts/{}.cube",
                TechNameAccessors::asset_path(),
                lut
            ));
        }

        videos.push((stream_settings.base_cache(), builder.build()));
    }

    let dx = TechNameAccessors::distort_names()[stream_settings.distort_selected() as usize].0;
    if dx != "none" {
        videos.push((
            format!("distort_cache_{}", dx),
            Mix::builder()
                .name(format!("distort_cache_{}", dx))
                .video(format!("distort_{}", dx))
                .no_display(true)
                .build(),
        ));
    }
    let dy = TechNameAccessors::distort_names()[stream_settings.distort_selected() as usize].1;
    if dy != "none" {
        videos.push((
            format!("distort_cache_{}", dy),
            Mix::builder()
                .name(format!("distort_cache_{}", dy))
                .video(format!("distort_{}", dy))
                .no_display(true)
                .build(),
        ));
    }
    let wx = TechNameAccessors::distort_names()[stream_settings.warp_selected() as usize].0;
    if wx != "none" {
        videos.push((
            format!("distort_cache_{}", wx),
            Mix::builder()
                .name(format!("distort_cache_{}", wx))
                .video(format!("distort_{}", wx))
                .no_display(true)
                .build(),
        ));
    }
    let wy = TechNameAccessors::distort_names()[stream_settings.warp_selected() as usize].1;
    if wy != "none" {
        videos.push((
            format!("distort_cache_{}", wy),
            Mix::builder()
                .name(format!("distort_cache_{}", wy))
                .video(format!("distort_{}", wy))
                .no_display(true)
                .build(),
        ));
    }
    let overlay = TechNameAccessors::overlay_names()[stream_settings.overlay_selected() as usize];
    videos.push((
        format!("overlay_{overlay}"),
        Mix::builder()
            .name(format!("overlay_cache_{overlay}"))
            .video(format!("overlay_{overlay}"))
            .no_display(true)
            .build(),
    ));
    videos
}

pub fn show_stream<T>(idx: usize, settings: &mut AllSettings<T>, target: CopyEx) -> Vec<RenderSpec>
where
    T: GlobalNameAccessors,
{
    let mut specs = vec![];
    if false && settings.initial_reset_complete[idx] == false {
        settings.initial_reset_complete[idx] = true;
        specs.extend(
            (&StreamSettings::<T>::ALL_STREAMSETTINGS_UPDATERS)
                .iter()
                .map(|f| f(&settings.playback[idx].stream))
                .flatten(),
        );
    }

    let stream_settings = &settings.playback[idx].stream;

    let _lut = TechNameAccessors::lut_names()[stream_settings.lut_selected() as usize];
    let (distort_x, distort_y) =
        TechNameAccessors::distort_names()[stream_settings.distort_selected() as usize];
    let (warp_x, warp_y) =
        TechNameAccessors::distort_names()[stream_settings.warp_selected() as usize];

    let mut builder = Mix::builder()
        .name(stream_settings.main_mix())
        .mixed(stream_settings.feedback_cache())
        .mixed(stream_settings.base_cache())
        .no_display(true);

    if distort_x != "none" && distort_y != "none" {
        builder = builder.mixed(format!("distort_cache_{}", distort_x));
        builder = builder.mixed(format!("distort_cache_{}", distort_y));
    }
    if warp_x != "none" && warp_y != "none" {
        builder = builder.mixed(format!("distort_cache_{}", warp_x));
        builder = builder.mixed(format!("distort_cache_{}", warp_y));
    }
    specs.push(builder.build().into());

    specs.push(
        Mix::builder()
            .name(stream_settings.feedback_cache())
            .mixed(stream_settings.main_mix())
            .no_display(true)
            .build()
            .into(),
    );

    let overlay_name =
        TechNameAccessors::overlay_names()[stream_settings.overlay_selected() as usize];
    let overlay_cache = format!("overlay_cache_{}", overlay_name);

    specs.push(
        Mix::builder()
            .name(stream_settings.overlay_layer())
            .mixed(stream_settings.feedback_cache())
            .mixed(overlay_cache)
            .target(target)
            .no_display(false)
            .build()
            .into(),
    );

    specs
}
