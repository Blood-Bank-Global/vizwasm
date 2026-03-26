use std::{
    collections::HashMap,
    error::Error,
    sync::{LazyLock, Mutex},
};

#[allow(unused_imports)]
use sdlrig::gfxinfo::{MIDI_CONTROL_CHANGE, MIDI_NOTE_OFF, MIDI_NOTE_ON};

use sdlrig::{
    gfxinfo::{Asset, GfxEvent, GfxInfo, MidiEvent, Vid, VidMixer},
    renderspec::{Mix, RenderSpec},
};

use vizwasm::{
    shaderlookup::include_files,
    vizconfig::{AllSettings, MixConfig},
};

fn main() {}

static STREAM_PATH: &'static str = "/Users/ttie/Desktop/presentation/streams";
static TECH_PATH: &'static str = "/Users/ttie/Desktop/tech_streams";
static ASSET_PATH: &'static str = "/Users/ttie/Desktop/common_data";

static STREAM_DEFS: LazyLock<Vec<Vid>> = LazyLock::new(|| {
    let mut vids = vec![];

    let vid640x480: &[&str] = &[
        "01_blood",
        "02_tools",
        "03_first_algorave",
        "05_no_really",
        "06_cats_all",
        "08_demo_wasm",
        "09_video_inspiration",
        "11_bionic_six",
        "13_walking",
        "15_macro",
        "17_synthbiosis",
        "17_5_memory",
        "19_dissodance",
        "nyc",
        "morning",
        "coffee",
    ];
    for vid_name in vid640x480.iter() {
        vids.push(
            Vid::builder()
                .name(vid_name)
                .path(format!("{STREAM_PATH}/{}.mp4", vid_name))
                .resolution((640, 480))
                .tbq((1, 12800))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }

    let tech_vids640x480: &[&str] = &["blank", "fluffy_clouds"];
    for vid_name in tech_vids640x480.iter() {
        vids.push(
            Vid::builder()
                .name(vid_name)
                .path(format!("{TECH_PATH}/{}.mp4", vid_name))
                .resolution((640, 480))
                .tbq((1, 12800))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }

    let pngs640x480: &[&str] = &[
        "04_first_process",
        "07_architecture_with_wasm",
        "10_arch_ffmpeg",
        "12_ffmpeg_api",
        "14_keyboard",
        "16_realtime_diagram",
        "18_libplacebo",
        "20_final_form",
    ];
    for png_name in pngs640x480.iter() {
        vids.push(
            Vid::builder()
                .name(png_name)
                .path(format!("{STREAM_PATH}/{}.png", png_name))
                .resolution((640, 480))
                .tbq((1, 12800))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }

    //Cameras
    vids.push(
        Vid::builder()
            .name("front cam")
            .path("MacBook Pro Camera")
            // .path("Logitech BRIO")
            .format("avfoundation")
            .opts(&vec![
                ("pixel_format", "bgr0"),
                ("framerate", "30.0"),
                ("video_size", "1280x720"),
                // ("video_size", "640x360"),
                // ("video_size", "1920x1080"),
                ("fflags", "+nobuffer+flush_packets"),
                ("probesize", "32"),
                ("flags", "low_delay"),
                ("analyzeduration", "0"),
                ("rtbufsize", "5000000"),
            ])
            .resolution((1280, 720))
            // .resolution((640, 360))
            // .resolution((1920, 1080))
            .tbq((1, 1000000))
            .pix_fmt("bgr0")
            .repeat(false)
            .realtime(true)
            .hardware_decode(false)
            .build()
            .into(),
    );
    vids
});

static PLAYBACK_NAMES: LazyLock<Vec<String>> = LazyLock::new(|| {
    let names = [
        "blank",
        "demo_fonts",
        "title", //"01_blood",
        "02_tools",
        "03_first_algorave",
        "04_first_process",
        "05_no_really",
        "06_cats_all",
        "07_architecture_with_wasm",
        "08_demo_wasm",
        "09_video_inspiration",
        "10_arch_ffmpeg",
        "11_bionic_six",
        "12_ffmpeg_api",
        "13_walking",
        "14_keyboard",
        "15_macro",
        "16_realtime_diagram",
        "17_synthbiosis",
        "17_5_memory",
        "18_libplacebo",
        "19_dissodance",
        "20_final_form",
        "flex",
        "cp437",
        "boxel",
        "decompile",
        "glitch_morning",
        "fluffy_clouds",
        "front cam",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<_>>();
    names
});

static MIX_CONFIGS: LazyLock<Vec<MixConfig>> = LazyLock::new(|| {
    let mut configs = vec![];

    for vid in STREAM_DEFS.iter() {
        let mix_name = format!("{}_mix", vid.name);
        configs.push(MixConfig {
            def: VidMixer::builder()
                .name(mix_name.clone())
                .width(vid.resolution.0 as u32)
                .height(vid.resolution.1 as u32)
                .build(),
            mix: Mix::builder()
                .name(mix_name.clone())
                .video(&vid.name)
                .no_display(true)
                .build(),
        });
    }

    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("title_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!(
                "../glsl/presentation/title.glsl"
            )))
            .build(),
        mix: Mix::builder()
            .name("title_mix")
            .mixed("01_blood_mix")
            .no_display(true)
            .build(),
    });

    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("flex_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!(
                "../glsl/presentation/flex.glsl"
            )))
            .build(),
        mix: Mix::builder()
            .name("flex_mix")
            .mixed("blank_mix")
            .no_display(true)
            .build(),
    });
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("cp437_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/cp437.glsl")))
            .build(),
        mix: Mix::builder()
            .name("cp437_mix")
            .mixed("blank_mix")
            .no_display(true)
            .build(),
    });
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("boxel_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/boxel.glsl")))
            .build(),
        mix: Mix::builder()
            .name("boxel_mix")
            .mixed("fluffy_clouds_mix")
            .no_display(true)
            .build(),
    });
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("demo_fonts_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/demo_fonts.glsl")))
            .build(),
        mix: Mix::builder()
            .name("demo_fonts_mix")
            .mixed("blank_mix")
            .no_display(true)
            .build(),
    });

    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("decompile_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/decompile.glsl")))
            .build(),
        mix: Mix::builder()
            .name("decompile_mix")
            .mixed("nyc_mix")
            .no_display(true)
            .build(),
    });

    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("glitch_morning_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/morning.glsl")))
            .build(),
        mix: Mix::builder()
            .name("glitch_morning_mix")
            .mixed("morning_mix")
            .mixed("coffee_mix")
            .no_display(true)
            .build(),
    });
    configs
});

static SETTINGS: LazyLock<Mutex<Box<AllSettings>>> = LazyLock::new(|| {
    let settings = AllSettings::new(
        STREAM_DEFS.clone(),
        MIX_CONFIGS.clone(),
        PLAYBACK_NAMES.clone(),
        ASSET_PATH,
    );
    Mutex::new(Box::new(settings))
});

#[no_mangle]
#[allow(unused)]
pub fn asset_list(fps: i64) -> Vec<Asset> {
    let mut lock = if let Ok(lock) = SETTINGS.lock() {
        lock
    } else {
        panic!("Settings object has been corrupted.");
    };

    let settings = lock.as_mut();

    if PLAYBACK_NAMES
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        != settings.playback_names
    {
        let orig = settings.clone();
        *settings = AllSettings::new(
            STREAM_DEFS.clone(),
            MIX_CONFIGS.clone(),
            PLAYBACK_NAMES.clone(),
            STREAM_PATH,
        );
        eprintln!("settings playback names: {:?}", settings.playback_names);
        for i in 0..orig.playback.len() {
            for j in 0..settings.playback.len() {
                if settings.playback[j].stream.ident.name == orig.playback[i].stream.ident.name {
                    // If the playback name matches, copy the stream.
                    let ident = settings.playback[j].stream.ident.clone();
                    settings.playback[j].stream = orig.playback[i].stream.clone();
                    settings.playback[j].stream.ident = ident;
                }
            }
        }
    }

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
    let decoded =
        serde_json::from_slice::<AllSettings>(bytes).expect("Failed to decode settings from bytes");
    settings.scan_idx = decoded.scan_idx;
    settings.active_idx = decoded.active_idx;
    settings.display_idx = decoded.display_idx;
    for i in 0..decoded.playback.len() {
        if i >= settings.playback.len() {
            break;
        }
        if settings.playback[i].stream.ident == decoded.playback[i].stream.ident {
            settings.playback[i] = decoded.playback[i].clone();
        }
    }
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
    Ok(settings.update_record_and_get_specs(reg_events, frame, Some(mega_cb))?)
}

const _IAC: &str = "IAC Driver Bus 1";
pub fn mega_cb(_all_settings: &mut AllSettings, _event: &MidiEvent) {}
