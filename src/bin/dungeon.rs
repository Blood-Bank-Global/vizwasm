use std::{
    collections::HashMap,
    error::Error,
    sync::{
        mpsc::{channel, Receiver, Sender},
        LazyLock, Mutex,
    },
};

#[allow(unused_imports)]
use sdlrig::gfxinfo::{MIDI_CONTROL_CHANGE, MIDI_NOTE_OFF, MIDI_NOTE_ON};

use sdlrig::{
    gfxinfo::{Asset, GfxEvent, GfxInfo, MidiEvent, Vid, VidMixer},
    renderspec::{Mix, RenderSpec, SendCmd},
};

use vizwasm::{beat_time_boilerplate, streamsettings::StreamSettingsField};
use vizwasm::{
    shaderhelper::include_files,
    vizconfig::{time_code_2_float, AllSettings, MixConfig},
};
fn main() {}

static STREAM_PATH: &'static str = "/Users/ttie/Desktop/dungeon/streams";
static TECH_PATH: &'static str = "/Users/ttie/Desktop/tech_streams";
static ASSET_PATH: &'static str = "/Users/ttie/Desktop/common_data";

static STREAM_DEFS: LazyLock<Vec<Vid>> = LazyLock::new(|| {
    let mut vids = vec![];

    let vid640x480 = [
        "CityHunter",
        "a_sword_in_the_stone",
        "arthur",
        "columns",
        "facade",
        "statue",
        "the_moon",
        "the_snow_queen",
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

    let rip_vids640x480 = ["sopranos"];
    for vid_name in rip_vids640x480.iter() {
        vids.push(
            Vid::builder()
                .name(vid_name.trim_end_matches(".mp4"))
                .path(format!("/Users/ttie/Desktop/rip/{}.mp4", vid_name))
                .resolution((640, 480))
                .tbq((1, 12800))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }

    let tech_vids640x480 = [
        "blank",
        "blur_lights",
        "burns",
        "circles",
        "clock",
        "corp",
        "cube",
        "epic",
        "fluffy_clouds",
        "holo_city",
        "horizon",
        "night_sky",
        "old_clouds",
        "silver_lining",
        "smoke",
        "stars",
        "target",
        "tube",
        "vestial1",
        "vestial2",
    ];
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
        "pixel_cam",
        "CityHunter",
        "demo_fonts",
        "a_sword_in_the_stone",
        "the_moon",
        "front cam",
        "text cam",
        "blur_lights",
        "burns",
        "circles",
        "clock",
        "corp",
        "cube",
        "epic",
        "fluffy_clouds",
        "holo_city",
        "horizon",
        "night_sky",
        "old_clouds",
        "silver_lining",
        "smoke",
        "stars",
        "target",
        "tube",
        "vestial1",
        "vestial2",
        "quest_message",
        "arthur",
        "a_sword_in_the_stone_combo",
        "jam",
        "statue",
        "the_snow_queen",
        "sunrise_scene",
        "sunrise_combo",
        "columns",
        "facade",
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

    macro_rules! generate_combo_mix {
        ($name:expr, $($to_combo:expr),* $(,)?) => {
            let mut mix = Mix::builder()
                .name(concat!($name, "_mix"))
                .no_display(true);

            $(mix = mix.mixed($to_combo);)*
            let mix = mix.build();
            configs.push(MixConfig {
                def: VidMixer::builder()
                    .name(concat!($name, "_mix"))
                    .shader(include_files(include_str!(concat!("../glsl/", $name, ".glsl"))))
                    .width(640)
                    .height(480)
                    .build(),
                mix
            });
        }
    }

    generate_combo_mix!(
        "a_sword_in_the_stone_combo",
        "a_sword_in_the_stone_overlay",
        "arthur_overlay",
    );

    generate_combo_mix!("sunrise_scene", "blank_overlay");
    generate_combo_mix!("sunrise_combo", "sunrise_scene_overlay");

    // And config for the jam
    let blank_vid = STREAM_DEFS.iter().find(|v| v.name == "blank");
    if let Some(vid) = blank_vid {
        configs.push(MixConfig {
            def: VidMixer::builder()
                .name("jam_mix")
                .width(vid.resolution.0 as u32)
                .height(vid.resolution.1 as u32)
                .shader(include_files(include_str!("../glsl/jam.glsl")))
                .build(),
            mix: Mix::builder()
                .name("jam_mix")
                .mixed("blank_overlay")
                .mixed("statue_overlay")
                .mixed("epic_overlay")
                .mixed("columns_overlay")
                .mixed("facade_overlay")
                .mixed("night_sky_overlay")
                .mixed("quest_message_overlay")
                .mixed("horizon_overlay")
                .mixed("the_snow_queen_overlay")
                .no_display(true)
                .build(),
        });
    }

    // And config for the fragment shaders on blank
    let blank_vid = STREAM_DEFS.iter().find(|v| v.name == "blank");
    if let Some(vid) = blank_vid {
        configs.push(MixConfig {
            def: VidMixer::builder()
                .name("quest_message_mix")
                .width(vid.resolution.0 as u32)
                .height(vid.resolution.1 as u32)
                .shader(include_files(include_str!("../glsl/quest_message.glsl")))
                .build(),
            mix: Mix::builder()
                .name("quest_message_mix")
                .mixed("blank_overlay")
                .no_display(true)
                .build(),
        });

        configs.push(MixConfig {
            def: VidMixer::builder()
                .name("demo_fonts_mix")
                .width(vid.resolution.0 as u32)
                .height(vid.resolution.1 as u32)
                .shader(include_files(include_str!("../glsl/demo_fonts.glsl")))
                .build(),
            mix: Mix::builder()
                .name("demo_fonts_mix")
                .mixed("blank_overlay")
                .no_display(true)
                .build(),
        });

        configs.push(MixConfig {
            def: VidMixer::builder()
                .name("text cam_mix")
                .width(vid.resolution.0 as u32)
                .height(vid.resolution.1 as u32)
                .shader(include_files(include_str!("../glsl/text_cam.glsl")))
                .build(),
            mix: Mix::builder()
                .name("text cam_mix")
                .mixed("front cam_overlay")
                .no_display(true)
                .build(),
        });

        configs.push(MixConfig {
            def: VidMixer::builder()
                .name("pixel_cam_mix")
                .width(vid.resolution.0 as u32)
                .height(vid.resolution.1 as u32)
                .shader(include_files(include_str!("../glsl/pixel_cam.glsl")))
                .build(),
            mix: Mix::builder()
                .name("pixel_cam_mix")
                .mixed("front cam_overlay")
                .no_display(true)
                .build(),
        });
    }

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

// setup midi callback
static MIDI_CALLBACK_CHANNELS: LazyLock<Mutex<(Sender<SendCmd>, Receiver<SendCmd>)>> =
    LazyLock::new(|| Mutex::new(channel::<SendCmd>()));

static TARGET_SIZE_W: u32 = 640;
static TARGET_SIZE_H: u32 = 480;

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

    let mut specs = settings.update_record_and_get_specs(reg_events, frame, Some(mega_cb))?;

    // Drain out any midi commands for glsl
    {
        let midi_channels = MIDI_CALLBACK_CHANNELS.lock().unwrap();
        for cmd in midi_channels.1.try_iter() {
            specs.push(RenderSpec::SendCmd(cmd));
        }
    }

    let mut seen = HashMap::<String, Mix>::new();

    // TOP
    let mix_name = settings.playback[settings.active_idx].stream.overlay_mix();
    specs.append(&mut do_display(settings, &mut seen, mix_name, (0, 0)));
    // BOTTOM
    let mix_name = settings.playback[settings.display_idx].stream.overlay_mix();
    specs.append(&mut do_display(
        settings,
        &mut seen,
        mix_name,
        (0, TARGET_SIZE_H as i32),
    ));
    // wireframe_data_mix
    let mix_name = "wireframe_data_mix";
    specs.append(&mut do_display(
        settings,
        &mut seen,
        mix_name,
        (TARGET_SIZE_W as i32, 0),
    ));
    // logs panel
    let mix_name = "logs_mix";
    specs.append(&mut do_display(
        settings,
        &mut seen,
        mix_name,
        (TARGET_SIZE_W as i32, TARGET_SIZE_H as i32),
    ));

    let to_return = specs.clone();
    settings.clean_up_by_specs(&mut specs);
    Ok(to_return)
}

pub fn do_display<T: AsRef<str>>(
    settings: &mut AllSettings,
    seen: &mut HashMap<String, Mix>,
    mix_name: T,
    offset: (i32, i32),
) -> Vec<RenderSpec> {
    let mut specs = vec![];
    if let Some(mix_config) = settings.mix_configs.get_mut(mix_name.as_ref()) {
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

        let playback_specs = settings.get_playback_specs(&mix_name, src, dst);
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

const IAC: &str = "IAC Driver Bus 1";
pub fn mega_cb(all_settings: &mut AllSettings, event: &MidiEvent) {
    a_sword_in_the_stone_cb(all_settings, event);
    alien_cb(all_settings, event);
    city_hunter_cb(all_settings, event);
}

pub fn a_sword_in_the_stone_cb(_all_settings: &mut AllSettings, event: &MidiEvent) {
    let time_codes = [
        "00:00:35:21",
        "00:00:43:20",
        "00:00:47:15",
        "00:00:54:15",
        "00:01:00:18",
        "00:01:07:00",
        "00:01:11:03",
        "00:01:31:18",
        "00:01:36:50",
        "00:01:39:37",
        "00:01:48:14",
        "00:01:55:39",
        "00:02:00:22",
        "00:02:05:36",
        "00:02:15:05",
        "00:02:20:32",
    ]
    .iter()
    .map(|s| time_code_2_float(s))
    .collect::<Vec<_>>();

    beat_time_boilerplate!(
        _all_settings,
        event,
        "a_sword_in_the_stone",
        "a_sword_in_the_stone_combo",
        time_codes
    );
}

pub fn alien_cb(all_settings: &mut AllSettings, event: &MidiEvent) {
    let text_cam = all_settings
        .playback
        .iter_mut()
        .find(|p| p.stream.ident.name == "text cam");
    if let Some(pb) = text_cam {
        match (event.kind, event.device.as_str(), event.key, event.velocity) {
            (MIDI_CONTROL_CHANGE, IAC, 0, value) => {
                if value > 10 {
                    pb.stream.set_field(StreamSettingsField::WarpLevel, 0.05);
                } else {
                    pb.stream.set_field(StreamSettingsField::WarpLevel, 0.0);
                }
            }
            _ => {}
        }
    }
}

pub fn city_hunter_cb(all_settings: &mut AllSettings, event: &MidiEvent) {
    beat_time_boilerplate!(
        all_settings,
        event,
        "CityHunter",
        "CityHunter",
        vec![
            time_code_2_float("00:00:01:23"),
            time_code_2_float("00:00:05:02"),
            time_code_2_float("00:00:08:13"),
            time_code_2_float("00:00:16:16"),
            time_code_2_float("00:00:22:01"),
            time_code_2_float("00:00:23:19"),
            time_code_2_float("00:00:35:17"),
            time_code_2_float("00:00:54:21"),
            time_code_2_float("00:00:56:14"),
            time_code_2_float("00:01:05:12"),
            time_code_2_float("00:01:10:06"),
            time_code_2_float("00:01:13:15"),
            time_code_2_float("00:01:16:22"),
            time_code_2_float("00:01:18:13"),
            time_code_2_float("00:01:21:04"),
            time_code_2_float("00:01:23:04"),
            time_code_2_float("00:01:25:07"),
            time_code_2_float("00:01:27:06"),
            time_code_2_float("00:01:43:15"),
            time_code_2_float("00:01:49:12"),
            time_code_2_float("00:01:55:03"),
            time_code_2_float("00:01:59:12"),
        ]
    );
}
