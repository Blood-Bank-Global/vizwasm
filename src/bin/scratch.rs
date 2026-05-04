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
    beat_time_boilerplate,
    shaderlookup::include_files,
    streamsettings::StreamSettingsField,
    vizconfig::{self, time_code_2_float, AllSettings, DisplayText, MixConfig, TextFileLoader},
    watch_text_for_display,
};

fn main() {}

static STREAM_PATH: &'static str = "/Users/ttie/Desktop/scratch/streams";
static TECH_PATH: &'static str = "/Users/ttie/Desktop/tech_streams";
static ASSET_PATH: &'static str = "/Users/ttie/Desktop/common_data";

static STREAM_DEFS: LazyLock<Vec<Vid>> = LazyLock::new(|| {
    let mut vids = vec![];

    let vid640x480: &[&str] = &[
        "spring",
        "dino",
        "park_screen",
        "arch",
        "bats_cut",
        "bats_full",
        "castle_cut",
        "castle_full",
        "doorway",
        "60s_reel",
        "60s_title",
        "60s_glitch",
        "70s",
        "70s_title",
        "80s_reel",
        "90s_reel",
        "test",
        "future",
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

    let pngs640x480: &[&str] = &["80s_achtung"];
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

    let cyber_tiles: &[&str] = &[
        "pier8x8",
        "street8x8",
        "prison8x8",
        "testing_room8x8",
        "warehouse8x8",
    ];
    for png_name in cyber_tiles.iter() {
        vids.push(
            Vid::builder()
                .name(png_name)
                .path(format!(
                    "/Users/ttie/Desktop/common_data/cyberpunk/{}.png",
                    png_name
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

    //Cameras
    // vids.push(
    //     Vid::builder()
    //         .name("front cam")
    //         .path("MacBook Pro Camera")
    //         // .path("Logitech BRIO")
    //         .format("avfoundation")
    //         .opts(&vec![
    //             ("pixel_format", "bgr0"),
    //             ("framerate", "30.0"),
    //             ("video_size", "1280x720"),
    //             // ("video_size", "640x360"),
    //             // ("video_size", "1920x1080"),
    //             ("fflags", "+nobuffer+flush_packets"),
    //             ("probesize", "32"),
    //             ("flags", "low_delay"),
    //             ("analyzeduration", "0"),
    //             ("rtbufsize", "5000000"),
    //         ])
    //         .resolution((1280, 720))
    //         // .resolution((640, 360))
    //         // .resolution((1920, 1080))
    //         .tbq((1, 1000000))
    //         .pix_fmt("bgr0")
    //         .repeat(false)
    //         .realtime(true)
    //         .hardware_decode(false)
    //         .build()
    //         .into(),
    // );
    vids
});

static PLAYBACK_NAMES: LazyLock<Vec<String>> = LazyLock::new(|| {
    let names = [
        "blank",
        "demo_fonts",
        "spring",
        "cp437",
        "boxel",
        "dino",
        "dino_glitch",
        "virtual",
        "cyberpunk",
        "vampire",
        "sixties",
        "seventies",
        "eighties",
        "nineties",
        "future",
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
            .name("dino_glitch_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/dino_glitch.glsl")))
            .build(),
        mix: Mix::builder()
            .name("dino_glitch_mix")
            .mixed("dino_overlay")
            .mixed("park_screen_mix")
            .mixed("distort_digital_white_dx_mix")
            .mixed("distort_digital_white_dy_mix")
            .seek_target_hint("dino")
            .no_display(true)
            .build(),
    });

    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("virtual_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/virtual.glsl")))
            .build(),
        mix: Mix::builder()
            .name("virtual_mix")
            .video("brush_maze1")
            .video("brush_maze2")
            .video("brush_maze3")
            .video("brush_maze4")
            .video("brush_cell1")
            .video("brush_cell2")
            .video("brush_cell3")
            .video("brush_text1")
            .video("brush_text2")
            .video("brush_text3")
            .video("brush_text4")
            .no_display(true)
            .build(),
    });

    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("cyberpunk_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/cyberpunk.glsl")))
            .build(),
        mix: Mix::builder()
            .name("cyberpunk_mix")
            .video("warehouse8x8")
            .no_display(true)
            .build(),
    });

    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("vampire_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/vampire.glsl")))
            .build(),
        mix: Mix::builder()
            .name("vampire_mix")
            .video("arch")
            .video("bats_cut")
            .video("bats_full")
            .video("castle_cut")
            .video("castle_full")
            .video("doorway")
            .no_display(true)
            .build(),
    });

    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("sixties_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/sixties.glsl")))
            .build(),
        mix: Mix::builder()
            .name("sixties_mix")
            .video("60s_reel")
            .video("60s_title")
            .video("60s_glitch")
            .no_display(true)
            .build(),
    });

    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("seventies_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/seventies.glsl")))
            .build(),
        mix: Mix::builder()
            .name("seventies_mix")
            .video("70s")
            .video("70s_title")
            .no_display(true)
            .build(),
    });

    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("eighties_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/eighties.glsl")))
            .build(),
        mix: Mix::builder()
            .name("eighties_mix")
            .video("80s_reel")
            .video("80s_achtung")
            .no_display(true)
            .build(),
    });

    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("nineties_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/nineties.glsl")))
            .build(),
        mix: Mix::builder()
            .name("nineties_mix")
            .video("90s_reel")
            .video("brush_maze1")
            .video("brush_pattern12")
            .video("brush_cell4")
            .no_display(true)
            .build(),
    });

    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("future_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/future.glsl")))
            .build(),
        mix: Mix::builder()
            .name("future_mix")
            .video("future")
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
    let mut specs = vec![];

    specs.extend(watch_text_for_display!(
        settings,
        "/tmp/viz/dino_frame.txt",
        "dino",
        "dino_glitch_mix",
        "dino_frame",
        "dino_frame_starts",
        "dino_frame_lens"
    ));

    specs.extend(watch_text_for_display!(
        settings,
        "/tmp/viz/maze.txt",
        "virtual",
        "virtual_mix",
        "maze_txt",
        "maze_starts",
        "maze_lens"
    ));

    specs.extend(watch_text_for_display!(
        settings,
        "/tmp/viz/cyberpunk.txt",
        "cyberpunk",
        "cyberpunk_mix",
        "cyberpunk_txt",
        "cyberpunk_starts",
        "cyberpunk_lens"
    ));

    specs.extend(watch_text_for_display!(
        settings,
        "/tmp/viz/future_dialog.txt",
        "future",
        "future_mix",
        "dialog",
        "dialog_starts",
        "dialog_lens"
    ));

    specs.extend(watch_text_for_display!(
        settings,
        "/tmp/viz/future_msg.txt",
        "future",
        "future_mix",
        "msg",
        "msg_starts",
        "msg_lens"
    ));

    specs.append(&mut settings.update_record_and_get_specs(reg_events, frame, Some(mega_cb))?);
    Ok(specs)
}

pub fn mega_cb(all_settings: &mut AllSettings, event: &MidiEvent) {
    dino_cb(all_settings, event);
    sixties_cb(all_settings, event);
    seventies_cb(all_settings, event);
    eighties_cb(all_settings, event);
    nineties_cb(all_settings, event);
    future_cb(all_settings, event);
}

pub fn dino_cb(all_settings: &mut AllSettings, event: &MidiEvent) {
    static TIMING: LazyLock<Vec<f64>> = LazyLock::new(|| {
        vec![
            time_code_2_float("00:00:00:01"),
            time_code_2_float("00:00:05:16"),
            time_code_2_float("00:00:12:13"),
            time_code_2_float("00:00:16:04"),
            time_code_2_float("00:00:22:00"),
            time_code_2_float("00:00:25:13"),
            time_code_2_float("00:00:38:01"),
            time_code_2_float("00:00:47:00"),
            time_code_2_float("00:00:51:18"),
            time_code_2_float("00:00:56:16"),
            time_code_2_float("00:01:04:02"),
            time_code_2_float("00:01:07:00"),
            time_code_2_float("00:01:13:24"),
            time_code_2_float("00:01:16:00"),
            time_code_2_float("00:01:20:24"),
            time_code_2_float("00:01:23:04"),
            time_code_2_float("00:01:26:21"),
            time_code_2_float("00:01:32:22"),
            time_code_2_float("00:01:36:09"),
            time_code_2_float("00:01:38:18"),
            time_code_2_float("00:01:41:02"),
            time_code_2_float("00:01:43:04"),
            time_code_2_float("00:01:44:29"),
            time_code_2_float("00:01:51:06"),
            time_code_2_float("00:01:54:26"),
            time_code_2_float("00:01:57:22"),
            time_code_2_float("00:02:02:26"),
            time_code_2_float("00:02:03:28"),
            time_code_2_float("00:02:07:16"),
            time_code_2_float("00:02:09:05"),
            time_code_2_float("00:02:12:05"),
            time_code_2_float("00:02:18:11"),
            time_code_2_float("00:02:35:14"),
        ]
    });

    beat_time_boilerplate!(all_settings, event, "dino_glitch", TIMING);
}

pub fn sixties_cb(all_settings: &mut AllSettings, event: &MidiEvent) {
    static TIMING: LazyLock<Vec<f64>> = LazyLock::new(|| {
        vec![
            time_code_2_float("00:00:11:26"),
            time_code_2_float("00:00:16:11"),
            time_code_2_float("00:00:23:16"),
            time_code_2_float("00:00:34:03"),
            time_code_2_float("00:00:36:02"),
            time_code_2_float("00:00:38:07"),
            time_code_2_float("00:00:42:19"),
            time_code_2_float("00:00:44:05"),
            time_code_2_float("00:00:45:08"),
            time_code_2_float("00:00:49:10"),
            time_code_2_float("00:00:55:08"),
            time_code_2_float("00:00:57:02"),
            time_code_2_float("00:00:59:02"),
            time_code_2_float("00:01:00:06"),
            time_code_2_float("00:01:03:20"),
            time_code_2_float("00:01:08:00"),
            time_code_2_float("00:02:07:13"),
            time_code_2_float("00:02:10:08"),
            time_code_2_float("00:02:12:02"),
            time_code_2_float("00:02:19:24"),
            time_code_2_float("00:02:24:01"),
            time_code_2_float("00:02:30:20"),
            time_code_2_float("00:02:35:24"),
            time_code_2_float("00:02:39:12"),
            time_code_2_float("00:02:42:05"),
            time_code_2_float("00:02:44:25"),
            time_code_2_float("00:02:49:06"),
            time_code_2_float("00:02:52:24"),
            time_code_2_float("00:02:57:01"),
            time_code_2_float("00:03:05:24"),
            time_code_2_float("00:03:13:24"),
            time_code_2_float("00:03:28:09"),
            time_code_2_float("00:04:21:25"),
            time_code_2_float("00:04:34:00"),
            time_code_2_float("00:04:53:29"),
            time_code_2_float("00:04:57:08"),
            time_code_2_float("00:05:10:06"),
            time_code_2_float("00:05:15:14"),
            time_code_2_float("00:05:46:15"),
            time_code_2_float("00:06:08:01"),
            time_code_2_float("00:06:13:26"),
            time_code_2_float("00:06:19:10"),
            time_code_2_float("00:06:25:16"),
            time_code_2_float("00:06:38:25"),
            time_code_2_float("00:06:46:05"),
            time_code_2_float("00:06:48:11"),
            time_code_2_float("00:06:50:22"),
            time_code_2_float("00:06:52:16"),
            time_code_2_float("00:06:58:09"),
            time_code_2_float("00:07:04:09"),
            time_code_2_float("00:07:10:13"),
            time_code_2_float("00:07:14:06"),
            time_code_2_float("00:07:28:07"),
            time_code_2_float("00:07:34:19"),
            time_code_2_float("00:07:40:05"),
            time_code_2_float("00:07:44:21"),
            time_code_2_float("00:07:53:06"),
            time_code_2_float("00:07:59:24"),
            time_code_2_float("00:08:03:28"),
            time_code_2_float("00:08:10:21"),
            time_code_2_float("00:08:15:24"),
        ]
    });

    beat_time_boilerplate!(all_settings, event, "sixties", TIMING);
}

pub fn seventies_cb(all_settings: &mut AllSettings, event: &MidiEvent) {
    static TIMING: LazyLock<Vec<f64>> = LazyLock::new(|| {
        vec![
            time_code_2_float("00:00:04:08"),
            time_code_2_float("00:04:20:08"),
            time_code_2_float("00:04:45:26"),
            time_code_2_float("00:05:28:16"),
            time_code_2_float("00:09:14:20"),
            time_code_2_float("00:10:01:18"),
            time_code_2_float("00:10:27:06"),
            time_code_2_float("00:12:13:26"),
            time_code_2_float("00:14:04:24"),
            time_code_2_float("00:17:40:22"),
            time_code_2_float("00:18:46:12"),
            time_code_2_float("00:19:41:26"),
            time_code_2_float("00:20:37:10"),
            time_code_2_float("00:24:32:00"),
            time_code_2_float("00:31:08:24"),
            time_code_2_float("00:31:38:20"),
            time_code_2_float("00:33:16:24"),
            time_code_2_float("00:34:50:20"),
            time_code_2_float("00:36:07:14"),
            time_code_2_float("00:39:44:15"),
            time_code_2_float("00:42:26:28"),
            time_code_2_float("00:42:38:01"),
            time_code_2_float("00:43:09:26"),
            time_code_2_float("00:44:01:02"),
            time_code_2_float("00:44:25:07"),
            time_code_2_float("00:47:04:16"),
            time_code_2_float("00:50:42:18"),
            time_code_2_float("00:50:46:02"),
            time_code_2_float("00:50:50:20"),
            time_code_2_float("00:55:26:20"),
            time_code_2_float("00:55:33:21"),
            time_code_2_float("00:56:00:04"),
            time_code_2_float("00:03:00:08"),
            time_code_2_float("00:03:52:26"),
            time_code_2_float("00:04:05:27"),
            time_code_2_float("00:04:48:09"),
            time_code_2_float("00:05:05:18"),
            time_code_2_float("00:05:08:12"),
            time_code_2_float("00:06:19:21"),
            time_code_2_float("00:06:31:21"),
            time_code_2_float("00:06:34:16"),
            time_code_2_float("00:07:11:04"),
            time_code_2_float("00:07:45:26"),
            time_code_2_float("00:08:17:06"),
        ]
    });

    beat_time_boilerplate!(all_settings, event, "seventies", TIMING);
}

pub fn eighties_cb(all_settings: &mut AllSettings, event: &MidiEvent) {
    static TIMING: LazyLock<Vec<f64>> = LazyLock::new(|| {
        vec![
            time_code_2_float("00:00:00:05"),
            time_code_2_float("00:00:09:12"),
            time_code_2_float("00:00:15:02"),
            time_code_2_float("00:00:18:20"),
            time_code_2_float("00:00:23:21"),
            time_code_2_float("00:00:30:16"),
            time_code_2_float("00:00:41:13"),
            time_code_2_float("00:01:02:04"),
            time_code_2_float("00:01:10:14"),
            time_code_2_float("00:01:19:03"),
            time_code_2_float("00:01:24:13"),
            time_code_2_float("00:01:31:23"),
            time_code_2_float("00:01:38:11"),
            time_code_2_float("00:01:44:06"),
            time_code_2_float("00:01:48:12"),
            time_code_2_float("00:01:50:15"),
            time_code_2_float("00:01:52:19"),
            time_code_2_float("00:01:55:03"),
            time_code_2_float("00:02:00:01"),
            time_code_2_float("00:02:10:20"),
            time_code_2_float("00:02:17:04"),
            time_code_2_float("00:02:21:08"),
            time_code_2_float("00:02:23:22"),
            time_code_2_float("00:02:26:23"),
            time_code_2_float("00:02:31:23"),
            time_code_2_float("00:02:37:23"),
            time_code_2_float("00:02:55:23"),
            time_code_2_float("00:03:03:03"),
            time_code_2_float("00:03:09:04"),
            time_code_2_float("00:03:33:11"),
            time_code_2_float("00:03:45:13"),
            time_code_2_float("00:04:02:08"),
            time_code_2_float("00:04:10:09"),
            time_code_2_float("00:04:21:21"),
            time_code_2_float("00:04:36:20"),
            time_code_2_float("00:04:51:01"),
            time_code_2_float("00:04:53:06"),
            time_code_2_float("00:05:03:22"),
            time_code_2_float("00:05:15:12"),
            time_code_2_float("00:05:21:21"),
            time_code_2_float("00:05:28:01"),
            time_code_2_float("00:05:46:13"),
            time_code_2_float("00:05:54:05"),
            time_code_2_float("00:06:01:06"),
            time_code_2_float("00:06:14:06"),
            time_code_2_float("00:06:22:06"),
            time_code_2_float("00:06:26:06"),
            time_code_2_float("00:06:28:09"),
            time_code_2_float("00:06:34:11"),
            time_code_2_float("00:06:52:10"),
            time_code_2_float("00:07:01:20"),
            time_code_2_float("00:07:07:11"),
            time_code_2_float("00:07:15:05"),
            time_code_2_float("00:07:17:20"),
            time_code_2_float("00:07:24:20"),
            time_code_2_float("00:07:27:12"),
            time_code_2_float("00:07:32:16"),
            time_code_2_float("00:07:35:09"),
            time_code_2_float("00:07:40:16"),
            time_code_2_float("00:07:41:21"),
            time_code_2_float("00:07:43:10"),
            time_code_2_float("00:07:44:19"),
            time_code_2_float("00:07:45:23"),
            time_code_2_float("00:07:50:16"),
            time_code_2_float("00:07:52:05"),
            time_code_2_float("00:07:57:22"),
            time_code_2_float("00:08:01:22"),
            time_code_2_float("00:08:08:15"),
            time_code_2_float("00:08:11:18"),
            time_code_2_float("00:08:42:16"),
            time_code_2_float("00:08:44:04"),
            time_code_2_float("00:08:48:15"),
        ]
    });

    beat_time_boilerplate!(all_settings, event, "eighties", TIMING);
}

pub fn nineties_cb(all_settings: &mut AllSettings, event: &MidiEvent) {
    static TIMING: LazyLock<Vec<f64>> = LazyLock::new(|| {
        vec![
            time_code_2_float("00:00:02:02"),
            time_code_2_float("00:00:58:25"),
            time_code_2_float("00:01:54:02"),
            time_code_2_float("00:02:13:05"),
            time_code_2_float("00:03:22:12"),
            time_code_2_float("00:04:36:00"),
            time_code_2_float("00:04:54:09"),
            time_code_2_float("00:05:16:20"),
            time_code_2_float("00:05:53:04"),
            time_code_2_float("00:06:54:21"),
            time_code_2_float("00:08:34:28"),
            time_code_2_float("00:08:39:24"),
            time_code_2_float("00:08:59:19"),
            time_code_2_float("00:10:37:21"),
            time_code_2_float("00:11:09:00"),
            time_code_2_float("00:11:34:26"),
            time_code_2_float("00:12:25:14"),
            time_code_2_float("00:14:09:08"),
            time_code_2_float("00:15:32:13"),
            time_code_2_float("00:18:38:27"),
            time_code_2_float("00:19:38:14"),
            time_code_2_float("00:20:23:18"),
            time_code_2_float("00:20:46:14"),
            time_code_2_float("00:23:50:12"),
            time_code_2_float("00:25:56:16"),
            time_code_2_float("00:28:36:06"),
            time_code_2_float("00:31:18:08"),
            time_code_2_float("00:33:16:08"),
            time_code_2_float("00:08:59:06"),
            time_code_2_float("00:10:03:16"),
            time_code_2_float("00:11:13:14"),
            time_code_2_float("00:11:53:22"),
            time_code_2_float("00:14:12:29"),
            time_code_2_float("00:26:02:21"),
            time_code_2_float("00:26:56:13"),
        ]
    });

    beat_time_boilerplate!(all_settings, event, "nineties", TIMING);
}

pub fn future_cb(all_settings: &mut AllSettings, event: &MidiEvent) {
    let pb_idx = all_settings
        .playback_names
        .iter()
        .position(|s| s == "future");

    if pb_idx.is_none() {
        return;
    }
    let pb_idx = pb_idx.unwrap();

    if all_settings.display_idx != pb_idx && all_settings.active_idx != pb_idx {
        return;
    }

    let duration = 11.0 * 60.0; // 11 minutes
    if matches!(
        (event.device.as_str(), event.channel, event.kind, event.key),
        (vizconfig::IAC, 0, MIDI_CONTROL_CHANGE, 0)
    ) && event.velocity > 10
    {
        let seek_target = rand::random::<f64>() * duration;
        all_settings.playback[pb_idx]
            .stream
            .set_field(StreamSettingsField::ExactSec, seek_target);
    }
}
