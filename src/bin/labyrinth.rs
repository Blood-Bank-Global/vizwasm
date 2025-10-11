use std::{
    cell::RefCell,
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
    renderspec::{Mix, RenderSpec, SendCmd, SendValue},
};

use vizwasm::vizconfig::{AllSettings, MixConfig, StreamSettingsAllFieldsEnum};
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
        "undead",
        "jester",
        "waves",
        "uncanny",
        "fire",
        "vertigo_dict",
        "vertigo_flowers",
        "vertigo_scenes",
        "vertigo_swirl",
        "toxic_bg",
        "toxic_boss",
        "toxic_cans",
        "toxic_door",
        "toxic_dunk",
        "toxic_guy",
        "toxic_mop",
        "toxic_plant",
        "toxic_rat",
        "toxic_static",
        "toxic_truck",
        "toxic_world",
        "flicker_scene",
        "flicker_book",
        "day8_agent",
        "day8_fault",
        "day8_flow",
        "day8_ops_err",
        "day8_trace",
        "swol_smoke",
        "swol_how",
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

    vids
});

static PLAYBACK_NAMES: LazyLock<Vec<String>> = LazyLock::new(|| {
    let names = vec![
        "castles_final".to_string(),
        "blank".to_string(),
        "towers".to_string(),
        "castle_combo".to_string(),
        "undead".to_string(),
        "jester".to_string(),
        "waves".to_string(),
        "jester_combo".to_string(),
        "uncanny".to_string(),
        "fire".to_string(),
        "vertigo_dict".to_string(),
        "vertigo_flowers".to_string(),
        "vertigo_scenes".to_string(),
        "vertigo_swirl".to_string(),
        "vertigo_combo".to_string(),
        "toxic_bg".to_string(),
        "toxic_boss".to_string(),
        "toxic_cans".to_string(),
        "toxic_door".to_string(),
        "toxic_dunk".to_string(),
        "toxic_guy".to_string(),
        "toxic_mop".to_string(),
        "toxic_plant".to_string(),
        "toxic_rat".to_string(),
        "toxic_static".to_string(),
        "toxic_truck".to_string(),
        "toxic_world".to_string(),
        "toxic_combo".to_string(),
        "flicker_scene".to_string(),
        "flicker_book".to_string(),
        "flicker_combo".to_string(),
        "day8_agent".to_string(),
        "day8_fault".to_string(),
        "day8_flow".to_string(),
        "day8_ops_err".to_string(),
        "day8_trace".to_string(),
        "day8_combo".to_string(),
        "swol_smoke".to_string(),
        "swol_how".to_string(),
        "swol_combo".to_string(),
    ];
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

    generate_combo_mix!("castle_combo", "castles_final_overlay", "towers_overlay");
    generate_combo_mix!("jester_combo", "jester_overlay", "waves_overlay");
    generate_combo_mix!(
        "vertigo_combo",
        "vertigo_dict_overlay",
        "vertigo_swirl_overlay",
        "vertigo_flowers_overlay",
        "vertigo_scenes_overlay",
    );

    generate_combo_mix!(
        "toxic_combo",
        "toxic_bg_overlay",
        "toxic_plant_overlay",
        "toxic_boss_overlay",
        "toxic_cans_overlay",
        "toxic_door_overlay",
        "toxic_dunk_overlay",
        "toxic_mop_overlay",
        "toxic_static_overlay",
    );

    generate_combo_mix!(
        "flicker_combo",
        "flicker_scene_overlay",
        "flicker_book_overlay"
    );

    generate_combo_mix!(
        "day8_combo",
        "day8_agent_overlay",
        "day8_fault_overlay",
        "day8_flow_overlay",
        "day8_ops_err_overlay",
        "day8_trace_overlay"
    );

    generate_combo_mix!("swol_combo", "swol_smoke_overlay", "swol_how_overlay");
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

    // Wire up usr_toggle to actually count up usr_var as well every change
    for i in 0..specs.len() {
        match &specs[i] {
            RenderSpec::SendCmd(cmd) => {
                if cmd.name == "usr_toggle" {
                    settings.playback[settings.active_idx]
                        .stream
                        .adjust_usr_var(1.0);
                    if settings.playback[settings.active_idx].stream.usr_var() >= 99.0 {
                        settings.playback[settings.active_idx]
                            .stream
                            .set_usr_var(-99.0);
                    } else if settings.playback[settings.active_idx].stream.usr_var() <= -99.0 {
                        settings.playback[settings.active_idx]
                            .stream
                            .set_usr_var(99.0);
                    }
                }
            }
            _ => (),
        }
    }

    specs.extend(
        settings.playback[settings.active_idx]
            .stream
            .get_commands(&[StreamSettingsAllFieldsEnum::USR_VAR]),
    );

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
const MIDI_DEVICE_VARS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(IAC.to_string(), IAC_GLSL.to_string());
    m
});

pub fn mega_cb(all_settings: &mut AllSettings, event: &MidiEvent) {
    glsl_midi_cb(all_settings, event);
    castle_combo_cb(all_settings, event);
    uncanny_cb(all_settings, event);
    fool_cb(all_settings, event);
    fire_cb(all_settings, event);
    toxic_cb(all_settings, event);
    flicker_cb(all_settings, event);
    day8_cb(all_settings, event);
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
    static _CB_TX: LazyLock<Sender<SendCmd>> = LazyLock::new(|| {
        let midi_channels = MIDI_CALLBACK_CHANNELS.lock().unwrap();
        midi_channels.0.clone()
    });

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

// 02 Jester
pub fn fool_cb(_all_settings: &mut AllSettings, event: &MidiEvent) {
    static _CB_TX: LazyLock<Sender<SendCmd>> = LazyLock::new(|| {
        let midi_channels = MIDI_CALLBACK_CHANNELS.lock().unwrap();
        midi_channels.0.clone()
    });

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

// 03 uncanny
pub fn uncanny_cb(all_settings: &mut AllSettings, event: &MidiEvent) {
    static _CB_TX: LazyLock<Sender<SendCmd>> = LazyLock::new(|| {
        let midi_channels = MIDI_CALLBACK_CHANNELS.lock().unwrap();
        midi_channels.0.clone()
    });

    static IDX: LazyLock<Option<usize>> = LazyLock::new(|| {
        let mut idx = None;
        for i in 0..PLAYBACK_NAMES.len() {
            if PLAYBACK_NAMES[i] == "uncanny" {
                idx.replace(i);
                break;
            }
        }
        idx
    });
    if let Some(idx) = *IDX {
        if all_settings.active_idx != idx && all_settings.display_idx != idx {
            return;
        }

        let stream = &mut all_settings.playback[idx].stream;
        // INTERNAL MATCHING FOR SETTING MODIFICATION
        match (
            event.device.as_str(),
            event.channel,
            event.kind,
            event.key,
            event.velocity,
        ) {
            (IAC, 0, MIDI_NOTE_ON, 60.., _) => {
                stream.set_rr(0.0);
                stream.set_rg(1.0);
                stream.set_gg(0.0);
                stream.set_bg(1.0);
                stream.set_bb(0.0);
                stream.set_rb(1.0);
            }
            (IAC, 0, MIDI_NOTE_OFF, 60.., _) => {
                stream.set_rr(1.0);
                stream.set_gg(1.0);
                stream.set_bb(1.0);
            }
            (IAC, 1, MIDI_NOTE_ON, k, _) => {
                stream.set_warp_level(k as f64 / 127.0 * 0.2);
            }
            (IAC, 1, MIDI_NOTE_OFF, _, _) => {
                stream.set_warp_level(0.0);
            }
            _ => (),
        }
    }
}

pub fn fire_cb(all_settings: &mut AllSettings, event: &MidiEvent) {
    static _CB_TX: LazyLock<Sender<SendCmd>> = LazyLock::new(|| {
        let midi_channels = MIDI_CALLBACK_CHANNELS.lock().unwrap();
        midi_channels.0.clone()
    });

    static TIME_CODES: &[f64] = &[
        1.0,
        142.0 / 15.0,
        133.0 / 10.0,
        503.0 / 30.0,
        191.0 / 10.0,
        209.0 / 10.0,
        761.0 / 30.0,
        397.0 / 15.0,
        291.0 / 10.0,
        454.0 / 15.0,
        162.0 / 5.0,
        1019.0 / 30.0,
        586.0 / 15.0,
        139.0 / 3.0,
        491.0 / 10.0,
    ];
    static LAST_CODE: Mutex<usize> = Mutex::new(0);
    static IDX: LazyLock<Option<usize>> = LazyLock::new(|| {
        let mut idx = None;
        for i in 0..PLAYBACK_NAMES.len() {
            if PLAYBACK_NAMES[i] == "fire" {
                idx.replace(i);
                break;
            }
        }
        idx
    });
    if let Some(idx) = *IDX {
        if all_settings.active_idx != idx && all_settings.display_idx != idx {
            return;
        }

        let stream = &mut all_settings.playback[idx].stream;
        // INTERNAL MATCHING FOR SETTING MODIFICATION
        match (
            event.device.as_str(),
            event.channel,
            event.kind,
            event.key,
            event.velocity,
        ) {
            (IAC, 0, MIDI_NOTE_ON, 36, _) => {
                let mut last_code = LAST_CODE.lock().unwrap();
                let curr = stream.real_ts.0 as f64 / stream.real_ts.1 as f64;
                if curr >= *TIME_CODES.last().unwrap_or(&0.0) {
                    *last_code = 0;
                    stream.set_exact_sec(*TIME_CODES.first().unwrap_or(&1.0));
                } else {
                    for tc in TIME_CODES.iter() {
                        if *tc > curr {
                            stream.set_exact_sec(*tc);
                            break;
                        }
                    }
                }
            }
            (IAC, 0, MIDI_NOTE_ON, 37, _) => stream.set_flash_enable(1),
            (IAC, 0, MIDI_NOTE_OFF, 37, _) => stream.set_flash_enable(0),
            (IAC, 0, MIDI_NOTE_ON, 38, _) => {
                stream.set_rr(2.0);
                stream.set_bb(0.5);
            }
            (IAC, 0, MIDI_NOTE_OFF, 38, _) => {
                stream.set_rr(1.0);
                stream.set_bb(1.0);
            }
            (IAC, 0, MIDI_NOTE_ON, 39, _) => {
                if stream.video_key_enable() > 0 {
                    stream.set_video_key_enable(0);
                } else {
                    stream.set_video_key_enable(1);
                }
            }
            _ => (),
        }
    }
}

pub fn toxic_cb(all_settings: &mut AllSettings, event: &MidiEvent) {
    static _CB_TX: LazyLock<Sender<SendCmd>> = LazyLock::new(|| {
        let midi_channels = MIDI_CALLBACK_CHANNELS.lock().unwrap();
        midi_channels.0.clone()
    });

    static IDX: LazyLock<Option<usize>> = LazyLock::new(|| {
        let mut idx = None;
        for i in 0..PLAYBACK_NAMES.len() {
            if PLAYBACK_NAMES[i] == "toxic_combo" {
                idx.replace(i);
                break;
            }
        }
        idx
    });
    if let Some(idx) = *IDX {
        if all_settings.active_idx != idx && all_settings.display_idx != idx {
            return;
        }

        let stream = &mut all_settings.playback[idx].stream;
        // INTERNAL MATCHING FOR SETTING MODIFICATION
        match (
            event.device.as_str(),
            event.channel,
            event.kind,
            event.key,
            event.velocity,
        ) {
            (IAC, 0, MIDI_NOTE_ON, 36.., _) => {
                stream.set_usr_var((stream.usr_var() + 1.0) % 6.0);
            }
            _ => (),
        }
    }
}

pub fn flicker_cb(all_settings: &mut AllSettings, event: &MidiEvent) {
    static _CB_TX: LazyLock<Sender<SendCmd>> = LazyLock::new(|| {
        let midi_channels = MIDI_CALLBACK_CHANNELS.lock().unwrap();
        midi_channels.0.clone()
    });

    static SCENE_IDX: LazyLock<Option<usize>> = LazyLock::new(|| {
        let mut idx = None;
        for i in 0..PLAYBACK_NAMES.len() {
            if PLAYBACK_NAMES[i] == "flicker_scene" {
                idx.replace(i);
                break;
            }
        }
        idx
    });

    static COMBO_IDX: LazyLock<Option<usize>> = LazyLock::new(|| {
        let mut idx = None;
        for i in 0..PLAYBACK_NAMES.len() {
            if PLAYBACK_NAMES[i] == "flicker_combo" {
                idx.replace(i);
                break;
            }
        }
        idx
    });

    static TIME_CODES: &[f64] = &[
        1.0,
        241.0 / 30.0,
        116.0 / 5.0,
        451.0 / 10.0,
        259.0 / 5.0,
        1.0,
        373.0 / 6.0,
    ];
    static LAST_CODE: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));
    if let (Some(scene_idx), Some(combo_idx)) = (*SCENE_IDX, *COMBO_IDX) {
        if all_settings.active_idx != scene_idx
            && all_settings.display_idx != scene_idx
            && all_settings.active_idx != combo_idx
            && all_settings.display_idx != combo_idx
        {
            return;
        }

        let stream = &mut all_settings.playback[scene_idx].stream;
        // INTERNAL MATCHING FOR SETTING MODIFICATION
        match (
            event.device.as_str(),
            event.channel,
            event.kind,
            event.key,
            event.velocity,
        ) {
            (IAC, 0, MIDI_CONTROL_CHANGE, 0, v) => {
                if v > 10 {
                    let lock = LAST_CODE.lock().unwrap();
                    let mut last = lock.borrow_mut();
                    stream.set_exact_sec(*TIME_CODES.get(*last).unwrap_or(&1.0));
                    *last = (*last + 1) % TIME_CODES.len();
                }
            }
            _ => (),
        }
    }
}

pub fn day8_cb(all_settings: &mut AllSettings, event: &MidiEvent) {
    static _CB_TX: LazyLock<Sender<SendCmd>> = LazyLock::new(|| {
        let midi_channels = MIDI_CALLBACK_CHANNELS.lock().unwrap();
        midi_channels.0.clone()
    });

    static COMBO_IDX: LazyLock<Option<usize>> = LazyLock::new(|| {
        let mut idx = None;
        for i in 0..PLAYBACK_NAMES.len() {
            if PLAYBACK_NAMES[i] == "day8_combo" {
                idx.replace(i);
                break;
            }
        }
        idx
    });

    static AGENT_IDX: LazyLock<Option<usize>> = LazyLock::new(|| {
        let mut idx = None;
        for i in 0..PLAYBACK_NAMES.len() {
            if PLAYBACK_NAMES[i] == "day8_agent" {
                idx.replace(i);
                break;
            }
        }
        idx
    });

    static TIME_CODES: &[f64] = &[
        1.0,
        127.0 / 15.0,
        316.0 / 15.0,
        86.0 / 3.0,
        734.0 / 15.0,
        699.0 / 10.0,
        3467.0 / 30.0,
        1856.0 / 15.0,
        3899.0 / 30.0,
        3983.0 / 30.0,
        1349.0 / 10.0,
        686.0 / 5.0,
        4877.0 / 30.0,
        2473.0 / 15.0,
        2516.0 / 15.0,
    ];
    static LAST_CODE: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));
    static COUNTER: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));

    if let (Some(combo_idx), Some(agent_idx)) = (*COMBO_IDX, *AGENT_IDX) {
        if all_settings.active_idx != combo_idx
            && all_settings.display_idx != combo_idx
            && all_settings.active_idx != agent_idx
            && all_settings.display_idx != agent_idx
        {
            return;
        }

        // INTERNAL MATCHING FOR SETTING MODIFICATION
        match (
            event.device.as_str(),
            event.channel,
            event.kind,
            event.key,
            event.velocity,
        ) {
            (IAC, 2, MIDI_CONTROL_CHANGE, 0, _) => {
                let lock = LAST_CODE.lock().unwrap();
                let mut last = lock.borrow_mut();
                all_settings.playback[agent_idx]
                    .stream
                    .set_exact_sec(*TIME_CODES.get(*last).unwrap_or(&1.0));
                *last = (*last + 1) % TIME_CODES.len();
                let count_lock = COUNTER.lock().unwrap();
                let mut count = count_lock.borrow_mut();
                *count += 1;
                all_settings.playback[combo_idx]
                    .stream
                    .set_usr_var(((*count / 5) % 4) as f64);
            }
            (IAC, 0, MIDI_CONTROL_CHANGE, 1, v) => {
                all_settings.playback[agent_idx]
                    .stream
                    .set_warp_level(v as f64 / 127.0 * 0.02);
                if v > 5 {
                    all_settings.playback[combo_idx]
                        .stream
                        .set_warp_selected(6.0);
                } else {
                    all_settings.playback[combo_idx]
                        .stream
                        .set_warp_selected(0.0);
                }
            }
            _ => (),
        }
    }
}

pub fn swol_cb(all_settings: &mut AllSettings, event: &MidiEvent) {
    static _CB_TX: LazyLock<Sender<SendCmd>> = LazyLock::new(|| {
        let midi_channels = MIDI_CALLBACK_CHANNELS.lock().unwrap();
        midi_channels.0.clone()
    });

    static HOW_IDX: LazyLock<Option<usize>> = LazyLock::new(|| {
        let mut idx = None;
        for i in 0..PLAYBACK_NAMES.len() {
            if PLAYBACK_NAMES[i] == "swol_how" {
                idx.replace(i);
                break;
            }
        }
        idx
    });

    static SMOKE_IDX: LazyLock<Option<usize>> = LazyLock::new(|| {
        let mut idx = None;
        for i in 0..PLAYBACK_NAMES.len() {
            if PLAYBACK_NAMES[i] == "swol_smoke" {
                idx.replace(i);
                break;
            }
        }
        idx
    });

    static _COUNTER: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));

    if let (Some(how_idx), Some(agent_idx)) = (*HOW_IDX, *SMOKE_IDX) {
        if all_settings.active_idx != how_idx
            && all_settings.display_idx != how_idx
            && all_settings.active_idx != agent_idx
            && all_settings.display_idx != agent_idx
        {
            return;
        }

        // INTERNAL MATCHING FOR SETTING MODIFICATION
        match (
            event.device.as_str(),
            event.channel,
            event.kind,
            event.key,
            event.velocity,
        ) {
            (IAC, 2, MIDI_CONTROL_CHANGE, 0, _) => {
                ();
            }
            (IAC, 0, MIDI_CONTROL_CHANGE, 1, v) => {
                all_settings.playback[agent_idx]
                    .stream
                    .set_warp_level(v as f64 / 127.0 * 0.02);
                if v > 5 {
                    all_settings.playback[how_idx].stream.set_warp_selected(6.0);
                } else {
                    all_settings.playback[how_idx].stream.set_warp_selected(0.0);
                }
            }
            _ => (),
        }
    }
}
