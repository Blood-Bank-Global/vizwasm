use std::{
    collections::HashMap,
    error::Error,
    sync::{
        mpsc::{channel, Receiver, Sender},
        LazyLock, Mutex,
    },
};

use rand;
use std::cell::RefCell;

#[allow(unused_imports)]
use sdlrig::gfxinfo::{MIDI_CONTROL_CHANGE, MIDI_NOTE_OFF, MIDI_NOTE_ON};

use sdlrig::{
    gfxinfo::{Asset, GfxEvent, GfxInfo, MidiEvent, Vid, VidMixer},
    renderspec::{Mix, RenderSpec, SendCmd, SendValue},
};

use vizwasm::vizconfig::{AllSettings, MixConfig};
fn main() {}

static STREAM_PATH: &'static str = "/Users/ttie/Desktop/labyrinth/streams";
static ASSET_PATH: &'static str = "/Users/ttie/Desktop/common_data";

static STREAM_DEFS: LazyLock<Vec<Vid>> = LazyLock::new(|| {
    let mut vids = vec![];

    let vid640x320 = ["blank"];
    for vid_name in vid640x320.iter() {
        vids.push(
            Vid::builder()
                .name(vid_name)
                .path(format!("{STREAM_PATH}/{}.mp4", vid_name))
                .resolution((640, 320))
                .tbq((1, 12800))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }

    let vid640x480 = [
        "castles_final",
        "towers",
        "clouds",
        "inside",
        "angels",
        "library",
        "text",
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
    let pngs640x480: &[&str] = &["upperdragon", "lowerdragon"];
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

    vids
});

static PLAYBACK_NAMES: LazyLock<Vec<String>> = LazyLock::new(|| {
    let names = [
        "blank",
        "text",
        "text_combo",
        "castles_final",
        "towers",
        "castle_combo",
        "library",
        "library_combo",
        "inside",
        "angels",
        "inside_combo",
        "clouds",
        "upperdragon",
        "lowerdragon",
        "clouds_combo",
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
                    .header(concat!(include_str!("../glsl/utils.glsl")))
                    .body(include_str!(concat!("../glsl/", $name, ".glsl")))
                    .width(640)
                    .height(480)
                    .build(),
                mix
            });
        }
    }

    generate_combo_mix!("text_combo", "text_overlay");

    generate_combo_mix!("castle_combo", "castles_final_overlay", "towers_overlay");

    generate_combo_mix!(
        "clouds_combo",
        "clouds_overlay",
        "upperdragon_overlay",
        "lowerdragon_overlay"
    );

    generate_combo_mix!("inside_combo", "inside_overlay", "angels_overlay");

    generate_combo_mix!("library_combo", "library_overlay");
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
    if let Some(mix_config) = settings.mix_configs.get_mut(&mix_name) {
        let iw = mix_config.def.width as i32;
        let ih = mix_config.def.height as i32;
        let mut ow = iw;
        let mut oh = ih;
        let mut ix = 0;
        let mut iy = 0;

        let iaspect = iw as f32 / ih as f32;
        let oaspect = canvas_w as f32 / (canvas_h as f32 / 2.0);

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
        let dst = (0, 0, canvas_w as u32, canvas_h as u32 / 2);

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

    // BOTTOM
    let mix_name = settings.playback[settings.display_idx].stream.overlay_mix();
    if let Some(mix_config) = settings.mix_configs.get_mut(&mix_name) {
        let iw = mix_config.def.width as i32;
        let ih = mix_config.def.height as i32;
        let mut ow = iw;
        let mut oh = ih;
        let mut ix = 0;
        let mut iy = 0;

        let iaspect = iw as f32 / ih as f32;
        let oaspect = canvas_w as f32 / (canvas_h as f32 / 2.0);

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
        let dst = (0, canvas_h as i32 / 2, canvas_w as u32, canvas_h as u32 / 2);

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
    let to_return = specs.clone();
    settings.clean_up_by_specs(&mut specs);
    Ok(to_return)
}

const IAC: &str = "IAC Driver Bus 1";
const IAC_GLSL: &str = "iac_driver_bus_1";
const MFT: &str = "Midi Fighter Twister";
const MFT_GLSL: &str = "midi_fighter_twister";

const MIDI_DEVICE_VARS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(IAC.to_string(), IAC_GLSL.to_string());
    m.insert(MFT.to_string(), MFT_GLSL.to_string());
    m
});

pub fn mega_cb(all_settings: &mut AllSettings, event: &MidiEvent) {
    glsl_midi_cb(all_settings, event);
    //no castle cb
    inside_cb(all_settings, event);
    library_cb(all_settings, event);
    clouds_cb(all_settings, event);
}

macro_rules! cb_boilerplate {
    ( $all_settings:expr, $midi_event:expr, $bg_name:expr, $combo_name:expr, $time_codes:expr) => {
        static _CB_TX: LazyLock<Sender<SendCmd>> = LazyLock::new(|| {
            let midi_channels = MIDI_CALLBACK_CHANNELS.lock().unwrap();
            midi_channels.0.clone()
        });

        static BG_IDX: LazyLock<Option<usize>> = LazyLock::new(|| {
            let mut idx = None;
            for i in 0..PLAYBACK_NAMES.len() {
                if PLAYBACK_NAMES[i] == $bg_name {
                    idx.replace(i);
                    break;
                }
            }
            idx
        });

        static COMBO_IDX: LazyLock<Option<usize>> = LazyLock::new(|| {
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

        static TIME_IDX: LazyLock<Mutex<RefCell<usize>>> =
            LazyLock::new(|| Mutex::new(RefCell::new(0)));
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
                        $all_settings.playback[bg_idx]
                            .stream
                            .set_exact_sec(*$time_codes.get(*idx as usize).unwrap_or(&1.0));
                    }
                }
                _ => (),
            }
        }
    };
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

// Generic send for all midi devices to GLSL vars
pub fn glsl_midi_cb(_all_settings: &mut AllSettings, event: &MidiEvent) {
    static CB_TX: LazyLock<Sender<SendCmd>> = LazyLock::new(|| {
        let midi_channels = MIDI_CALLBACK_CHANNELS.lock().unwrap();
        midi_channels.0.clone()
    });

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
        .or_else(|| Some("???".to_string()))
        .unwrap();

    eprintln!(
        "{debug_kind}_{debug_device}_{}_{}{} = {}",
        event.channel, event.key, on_off, event.velocity
    );

    if let Some(glsl_device) = MIDI_DEVICE_VARS.get(&event.device) {
        for mix in PLAYBACK_NAMES.iter() {
            match event.kind {
                MIDI_NOTE_ON => {
                    let cmd = SendCmd {
                        mix: format!("{mix}_mix").to_string(),
                        name: format!("note_{}_{}_{}", glsl_device, event.channel, event.key)
                            .to_string(),
                        value: SendValue::Float(event.velocity as f32),
                    };
                    // eprintln!("Sending MIDI Note ON command to glsl: {:?}", cmd);
                    CB_TX.send(cmd).ok();
                    CB_TX
                        .send(SendCmd {
                            mix: format!("{mix}_mix").to_string(),
                            name: format!(
                                "note_{}_{}_{}_on",
                                glsl_device, event.channel, event.key
                            )
                            .to_string(),
                            value: SendValue::Unsigned(1),
                        })
                        .ok();
                }
                MIDI_NOTE_OFF => {
                    let cmd = SendCmd {
                        mix: format!("{mix}_mix").to_string(),
                        name: format!("note_{}_{}_{}", glsl_device, event.channel, event.key)
                            .to_string(),
                        value: SendValue::Float(0.0),
                    };
                    // eprintln!("Sending MIDI Note OFF command to glsl: {:?}", cmd);
                    CB_TX.send(cmd).ok();
                    CB_TX
                        .send(SendCmd {
                            mix: format!("{mix}_mix").to_string(),
                            name: format!(
                                "note_{}_{}_{}_on",
                                glsl_device, event.channel, event.key
                            )
                            .to_string(),
                            value: SendValue::Unsigned(0),
                        })
                        .ok();
                }
                MIDI_CONTROL_CHANGE => {
                    let cmd = SendCmd {
                        mix: format!("{mix}_mix").to_string(),
                        name: format!("cc_{}_{}_{}", glsl_device, event.channel, event.key)
                            .to_string(),
                        value: SendValue::Float(event.velocity as f32),
                    };
                    // eprintln!("Sending MIDI CC command to glsl: {:?}", cmd);
                    CB_TX.send(cmd).ok();
                }
                _ => (),
            }
        }
    }
}

// MIDI callback function for castle_combo
pub fn castle_combo_cb(_all_settings: &mut AllSettings, event: &MidiEvent) {
    static TIME_CODES: LazyLock<Vec<f64>> = LazyLock::new(|| {
        ["00:00:00:01"]
            .iter()
            .map(|s| time_code_2_float(s))
            .collect::<Vec<_>>()
    });

    cb_boilerplate!(
        _all_settings,
        event,
        "castles_final",
        "castle_combo",
        *TIME_CODES
    );

    // INTERNAL MATCHING FOR SETTING MODIFICATION
    match (
        event.device.as_str(),
        event.channel,
        event.kind,
        event.key,
        event.velocity,
    ) {
        // (IAC, 0, MIDI_NOTE_ON, 36, v) => settings.set_rr(v as f64 / 127.0 + 1.0),
        // (IAC, 0, MIDI_NOTE_OFF, 36, _) => settings.set_rr(1.0),
        // (IAC, 0, MIDI_NOTE_ON, 37, v) => settings.set_warp_level(v as f64 / 127.0 * 0.3),
        // (IAC, 0, MIDI_NOTE_OFF, 37, _) => settings.set_warp_level(0.0),
        // (IAC, 0, MIDI_NOTE_ON, 38, v) => settings.set_distort_level(v as f64 / 127.0 * 0.3),
        // (IAC, 0, MIDI_NOTE_OFF, 38, _) => settings.set_distort_level(0.1),
        // (IAC, 0, MIDI_CONTROL_CHANGE, 0, v) => settings.set_rh(v as f64 / 127.0 * 0.05),
        _ => (),
    }
}

pub fn clouds_cb(_all_settings: &mut AllSettings, event: &MidiEvent) {
    static TIME_CODES: LazyLock<Vec<f64>> = LazyLock::new(|| {
        [
            "00:00:02:12",
            "00:00:45:23",
            "00:01:32:24",
            "00:02:01:21",
            "00:02:38:04",
            "00:03:26:00",
        ]
        .iter()
        .map(|s| time_code_2_float(s))
        .collect::<Vec<_>>()
    });

    cb_boilerplate!(_all_settings, event, "clouds", "clouds_combo", *TIME_CODES);
}

pub fn inside_cb(_all_settings: &mut AllSettings, event: &MidiEvent) {
    static TIME_CODES: LazyLock<Vec<f64>> = LazyLock::new(|| {
        [
            "00:00:00:01",
            "00:00:20:04",
            "00:00:41:15",
            "00:00:58:04",
            "00:01:05:19",
            "00:01:18:23",
            "00:01:29:08",
            "00:01:31:23",
            "00:01:48:11",
            "00:02:05:16",
            "00:02:40:12",
            "00:03:02:14",
            "00:03:25:06",
            "00:03:36:22",
            "00:03:54:02",
            "00:04:05:04",
            "00:04:12:10",
            "00:04:31:10",
            "00:04:49:05",
            "00:05:06:08",
        ]
        .iter()
        .map(|s| time_code_2_float(s))
        .collect::<Vec<_>>()
    });

    cb_boilerplate!(_all_settings, event, "inside", "inside_combo", *TIME_CODES);
}

pub fn library_cb(_all_settings: &mut AllSettings, event: &MidiEvent) {
    static TIME_CODES: LazyLock<Vec<f64>> = LazyLock::new(|| {
        [
            "00:00:00:05",
            "00:00:03:27",
            "00:00:05:21",
            "00:00:07:18",
            "00:00:15:01",
            "00:00:23:19",
            "00:00:25:05",
            "00:00:37:00",
            "00:00:42:27",
            "00:00:50:10",
            "00:00:52:26",
            "00:00:57:18",
            "00:01:13:21",
            "00:01:20:13",
            "00:01:29:06",
            "00:01:30:16",
            "00:01:31:21",
            "00:01:34:10",
            "00:01:37:17",
            "00:01:39:28",
            "00:01:41:15",
            "00:01:43:27",
            "00:01:46:24",
            "00:01:51:03",
            "00:01:53:26",
            "00:01:59:26",
            "00:02:01:20",
            "00:02:06:16",
            "00:02:10:12",
            "00:02:15:08",
        ]
        .iter()
        .map(|s| time_code_2_float(s))
        .collect::<Vec<_>>()
    });

    cb_boilerplate!(
        _all_settings,
        event,
        "library",
        "library_combo",
        *TIME_CODES
    );
}
