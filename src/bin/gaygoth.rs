use std::{
    collections::HashMap,
    error::Error,
    sync::{LazyLock, Mutex},
};

#[allow(unused_imports)]
use sdlrig::gfxinfo::{MIDI_CONTROL_CHANGE, MIDI_NOTE_OFF, MIDI_NOTE_ON};

use sdlrig::{
    gfxinfo::{Asset, GfxEvent, GfxInfo, MidiEvent, Vid, VidMixer},
    renderspec::{Mix, MixInput, RenderSpec},
};

use vizwasm::{
    shaderlookup::include_files,
    streamsettings::StreamSettingsField,
    vizconfig::{AllSettings, MixConfig, IAC},
};

fn main() {}

static STREAM_PATH: &'static str = "/Users/ttie/Desktop/GayGoth/streams";
static TECH_PATH: &'static str = "/Users/ttie/Desktop/tech_streams";
static ASSET_PATH: &'static str = "/Users/ttie/Desktop/common_data";

static STREAM_DEFS: LazyLock<Vec<Vid>> = LazyLock::new(|| {
    let mut vids = vec![];

    let vid640x480: &[&str] = &[
        "bit_blood",
        "bit_dance",
        "bit_disco_ball",
        "hunger_club",
        "hunger_sizzle",
        "hunger_texture",
        "interview_armond",
        "interview_fire",
        "interview_lestat",
        "nadja_kiss",
        "nadja_texture",
        "nadja_walking",
        "own_blood",
        "own_kissing",
        "own_party",
        "queen_bite",
        "queen_city",
        "queen_crypt",
        "rocky_hot",
        "rocky_mouth",
        "rocky_warp",
        "shadows_parade",
        "so_vam_bite",
        "so_vam_dance",
        "so_vam_texture",
        "stoker_shave",
        "stoker_rain",
        "stoker_texture",
        "hellraiser_texture",
        "hellraiser_new",
        "hellraiser_old",
        "logo",
        "ahs_coven",
        "woodcut",
        "drac1",
        "drac2",
        "drac3",
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
        "arc",
        "another_kind_of_knight",
        "seb_eyes",
        "seb_eyes_sharp",
        "seb_eyes_sharp_only",
        "seb_eyes_sharp_only_invert",
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
                .name("front cam")
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
    let mut names = [
        "blank",
        "demo_fonts",
        "cp437",
        "freak",
        "fluffy_clouds",
        "coven",
        "shadows_parade",
        "fbtest",
        "gaygoth_all",
        "drac",
        "oils",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<_>>();

    if false {
        names.push("cam_freak".to_string());
    }

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
            .name("freak_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/frog.glsl")))
            .build(),
        mix: Mix::builder()
            .name("freak_mix")
            .video("fluffy_clouds")
            .video("brush_dither5")
            .no_display(true)
            .build(),
    });

    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("gaygoth_all_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/gaygoth.glsl")))
            .build(),
        mix: Mix::builder()
            .name("gaygoth_all_mix")
            .video("bit_blood")
            .video("bit_dance")
            .video("bit_disco_ball")
            .video("logo")
            .video("brush_pattern79") //18 26 79
            .video("brush_text2")
            .no_display(true)
            .build(),
    });

    if false {
        configs.push(MixConfig {
            def: VidMixer::builder()
                .name("cam_freak_mix")
                .width(640)
                .height(480)
                .shader(include_files(include_str!("../glsl/frog.glsl")))
                .build(),
            mix: Mix::builder()
                .name("cam_freak_mix")
                .video("front cam")
                .video("brush_dither5")
                .no_display(true)
                .build(),
        });
    }
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("coven_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/coven.glsl")))
            .build(),
        mix: Mix::builder()
            .name("coven_mix")
            .video("ahs_coven")
            .video("woodcut")
            .no_display(true)
            .build(),
    });
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("fbtest_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/fbtest.glsl")))
            .build(),
        mix: Mix::builder()
            .name("fbtest_mix")
            .video("arc")
            .video("another_kind_of_knight")
            .video("brush_pattern1")
            .video("brush_dither2")
            .no_display(true)
            .build(),
    });

    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("drac_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/drac.glsl")))
            .build(),
        mix: Mix::builder()
            .name("drac_mix")
            .video("drac1")
            .video("drac2")
            .video("drac3")
            .video("stoker_texture")
            .no_display(true)
            .build(),
    });

    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("oils_mix")
            .width(640)
            .height(480)
            .shader(include_files(include_str!("../glsl/oils.glsl")))
            .build(),
        mix: Mix::builder()
            .name("oils_mix")
            .video("brush_text2")
            .video("brush_pattern79")
            .video("seb_eyes_sharp_only_invert")
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

    // select the current set of input videos to the mix
    let names = vec![
        vec!["fluffy_clouds", "brush_text1", "brush_maze1"],
        vec!["bit_dance", "bit_blood", "bit_disco_ball"],
        vec!["hunger_club", "hunger_sizzle", "hunger_texture"],
        vec!["interview_fire", "interview_armond", "interview_lestat"],
        vec!["nadja_walking", "nadja_kiss", "nadja_texture"],
        vec!["own_party", "own_kissing", "own_blood"],
        vec!["queen_crypt", "queen_bite", "queen_city"],
        vec!["rocky_warp", "rocky_hot", "rocky_mouth"],
        vec!["stoker_rain", "stoker_shave", "stoker_texture"],
        vec!["so_vam_dance", "so_vam_bite", "so_vam_texture"],
        vec!["shadows_parade", "brush_pattern1", "brush_dither2"],
        vec!["hellraiser_old", "hellraiser_new", "hellraiser_texture"],
    ];

    if let Some(gay_goth_mix) = settings
        .playback
        .iter_mut()
        .find(|p| p.stream.ident.name == "gaygoth_all")
    {
        let idx = (100.0 * gay_goth_mix.stream.get_field(&StreamSettingsField::User6))
            .clamp(0.0, (names.len() - 1) as f64) as usize as usize;
        gay_goth_mix.stream.ident.seek_target = names[idx][0].to_string();
        let mix_name = gay_goth_mix.stream.ident.input_mix.clone();
        if let Some(cfg) = settings.mix_configs.get_mut(&mix_name) {
            cfg.mix.inputs[0] = MixInput::Video(names[idx][0].to_string());
            cfg.mix.inputs[1] = MixInput::Video(names[idx][1].to_string());
            cfg.mix.inputs[2] = MixInput::Video(names[idx][2].to_string());
        }
    }

    specs.append(&mut settings.update_record_and_get_specs(reg_events, frame, Some(mega_cb))?);
    Ok(specs)
}

pub fn mega_cb(
    #[allow(unused)] all_settings: &mut AllSettings,
    #[allow(unused)] event: &MidiEvent,
) {
    all_gay(all_settings, event);
}

pub fn all_gay(
    #[allow(unused)] all_settings: &mut AllSettings,
    #[allow(unused)] event: &MidiEvent,
) {
    if event.channel == 0 && event.device == IAC && event.key == 0 && event.velocity >= 10 {
        if let Some((idx, _)) = all_settings
            .playback_names
            .iter()
            .enumerate()
            .find(|(_, name)| *name == "all_gay")
        {
            if idx == all_settings.active_idx || idx == all_settings.display_idx {
                let t = rand::random::<f64>() * 2720.0;
                all_settings.playback[idx]
                    .stream
                    .set_field(vizwasm::streamsettings::StreamSettingsField::ExactSec, t);
            }
        }
    }
}
