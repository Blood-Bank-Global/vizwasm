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

static STREAM_PATH: &'static str = "/Users/ttie/Desktop/abstracts/streams";
static TECH_PATH: &'static str = "/Users/ttie/Desktop/tech_streams";
static ASSET_PATH: &'static str = "/Users/ttie/Desktop/common_data";

static STREAM_DEFS: LazyLock<Vec<Vid>> = LazyLock::new(|| {
    let mut vids = vec![];

    let vid640x480: &[&str] = &[];
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

    let tech_vids640x480: &[&str] = &["blank", "fluffy_clouds", "masque", "statue"];
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

    let pngs640x480: &[&str] = &[];
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

    let brushes: &[&str] = &[
        "1",
        "2",
        "3",
        "4",
        "cell1",
        "cell2",
        "cell3",
        "cell4",
        "cell5",
        "dither1",
        "dither10",
        "dither2",
        "dither3",
        "dither4",
        "dither5",
        "dither6",
        "dither7",
        "dither8",
        "dither9",
        "maze1",
        "maze2",
        "maze3",
        "maze4",
        "maze5",
        "maze6",
        "maze7",
        "maze8",
        "pattern1",
        "pattern10",
        "pattern11",
        "pattern12",
        "pattern13",
        "pattern14",
        "pattern15",
        "pattern16",
        "pattern17",
        "pattern18",
        "pattern19",
        "pattern2",
        "pattern20",
        "pattern21",
        "pattern22",
        "pattern23",
        "pattern24",
        "pattern25",
        "pattern26",
        "pattern27",
        "pattern28",
        "pattern29",
        "pattern3",
        "pattern30",
        "pattern31",
        "pattern32",
        "pattern33",
        "pattern34",
        "pattern35",
        "pattern36",
        "pattern37",
        "pattern38",
        "pattern39",
        "pattern4",
        "pattern40",
        "pattern41",
        "pattern42",
        "pattern43",
        "pattern44",
        "pattern45",
        "pattern46",
        "pattern47",
        "pattern48",
        "pattern49",
        "pattern5",
        "pattern50",
        "pattern51",
        "pattern52",
        "pattern53",
        "pattern54",
        "pattern55",
        "pattern56",
        "pattern57",
        "pattern58",
        "pattern59",
        "pattern6",
        "pattern60",
        "pattern61",
        "pattern62",
        "pattern63",
        "pattern64",
        "pattern65",
        "pattern66",
        "pattern67",
        "pattern68",
        "pattern69",
        "pattern7",
        "pattern70",
        "pattern71",
        "pattern72",
        "pattern73",
        "pattern74",
        "pattern75",
        "pattern76",
        "pattern77",
        "pattern78",
        "pattern79",
        "pattern8",
        "pattern80",
        "pattern81",
        "pattern82",
        "pattern83",
        "pattern84",
        "pattern85",
        "pattern86",
        "pattern87",
        "pattern88",
        "pattern89",
        "pattern9",
        "text1",
        "text2",
        "text3",
        "text4",
        "texture1",
        "texture2",
        "texture3",
    ];

    for brush_name in brushes.iter() {
        vids.push(
            Vid::builder()
                .name(format!("brush_{}", brush_name))
                .path(format!(
                    "/Users/ttie/Desktop/common_data/brushes/pngs/{}.png",
                    brush_name
                ))
                .resolution((640, 480))
                .tbq((1, 12800))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }

    if false {
        // Cameras
        vids.push(
            Vid::builder()
                .name("front_cam")
                .path("MacBook Pro Camera")
                // .path("Logitech BRIO")
                // .path("Logitech StreamCam")
                .format("avfoundation")
                .opts(&vec![
                    ("pixel_format", "bgr0"),
                    ("framerate", "30.0"),
                    // ("video_size", "640x480"),
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
    }
    vids
});

static PLAYBACK_NAMES: LazyLock<Vec<String>> = LazyLock::new(|| {
    let names = ["blank", "demo_fonts", "cp437", "fluffy_clouds", "abstracts"]
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
            .name("abstracts_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/abstracts.glsl")))
            .build(),
        mix: Mix::builder()
            .name("abstracts_mix")
            .video("brush_text2")
            .video("brush_pattern79")
            .video("brush_dither2")
            .video("brush_maze2")
            .video("statue")
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
        if settings.playback[i].stream.ident.name == decoded.playback[i].stream.ident.name {
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
    let mut specs = vec![];

    specs.append(&mut settings.update_record_and_get_specs(reg_events, frame, Some(mega_cb))?);
    Ok(specs)
}

pub fn mega_cb(
    #[allow(unused)] all_settings: &mut AllSettings,
    #[allow(unused)] event: &MidiEvent,
) {
}
