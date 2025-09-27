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
    renderspec::{Mix, RenderSpec, SendCmd, SendValue},
};

use vizwasm::vizconfig::{AllSettings, MixConfig, StreamSettingsAllFieldsEnum};
fn main() {}

static STREAM_PATH: &'static str = "/Users/ttie/Desktop/labyrinth/streams";
static ASSET_PATH: &'static str = "/Users/ttie/Desktop/common_data";

static STREAM_DEFS: LazyLock<Vec<Vid>> = LazyLock::new(|| {
    let mut vids = vec![];

    let vid640x320 = [/*"the_moon",*/ "blank"];
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

    let vid640x480 = ["castles_final", "towers", "undead"];
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
    let pngs640x480 = ["wall_demo"];
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

    // let vid1280x720 = ["error2", "error3"];
    // for vid_name in vid1280x720.iter() {
    //     vids.push(
    //         Vid::builder()
    //             .name(vid_name)
    //             .path(format!("{STREAM_PATH}/{}.mp4", vid_name))
    //             .resolution((1280, 720))
    //             .tbq((1, 12800))
    //             .pix_fmt("yuv420p")
    //             .repeat(true)
    //             .realtime(false)
    //             .hardware_decode(false)
    //             .build(),
    //     );
    // }

    // let vids1920x1080 = ["logo", "statue"];
    // for vid_name in vids1920x1080.iter() {
    //     vids.push(
    //         Vid::builder()
    //             .name(vid_name)
    //             .path(format!("{STREAM_PATH}/{}.mp4", vid_name))
    //             .resolution((1920, 1080))
    //             .tbq((1, 12288))
    //             .pix_fmt("yuv420p")
    //             .repeat(true)
    //             .realtime(false)
    //             .hardware_decode(false)
    //             .build(),
    //     );
    // }

    vids
});

static PLAYBACK_NAMES: LazyLock<Vec<String>> = LazyLock::new(|| {
    let names = vec![
        "castles_final".to_string(),
        "blank".to_string(),
        "towers".to_string(),
        "castle_combo".to_string(),
        "undead".to_string(),
        // "combo1".to_string(),
        // "combo2".to_string(),
        // "combo3".to_string(),
        // "combo4".to_string(),
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

    // macro_rules! generate_harmony_mix {
    //     ($i:expr) => {
    //         configs.push(MixConfig {
    //             def: VidMixer::builder()
    //                 .name(concat!("harmony", $i, "_mix"))
    //                 .header(concat!(
    //                     include_str!("../glsl/utils.glsl"),
    //                     "\n",
    //                     include_str!("../glsl/harmony_header.glsl")
    //                 ))
    //                 .body(include_str!(concat!("../glsl/harmony", $i, ".glsl")))
    //                 .width(2560)
    //                 .height(1280)
    //                 .build(),
    //             mix: Mix::builder()
    //                 .name(concat!("harmony", $i, "_mix"))
    //                 .mixed("blank_overlay")
    //                 .no_display(true)
    //                 .build(),
    //         });
    //     };
    // }

    // generate_harmony_mix!(1);
    // generate_harmony_mix!(2);
    // generate_harmony_mix!(3);
    // generate_harmony_mix!(4);

    macro_rules! generate_combo_mix {
        ($name:expr, $below:expr, $above:expr) => {
            configs.push(MixConfig {
                def: VidMixer::builder()
                    .name(concat!($name, "_mix"))
                    .header(concat!(include_str!("../glsl/utils.glsl")))
                    .body(include_str!(concat!("../glsl/", $name, ".glsl")))
                    .width(640)
                    .height(480)
                    .build(),
                mix: Mix::builder()
                    .name(concat!($name, "_mix"))
                    .mixed($below)
                    .mixed($above)
                    .no_display(true)
                    .build(),
            });
        };
    }

    generate_combo_mix!("castle_combo", "castles_final_overlay", "towers_overlay");
    // generate_combo_mix!(1, "logo_overlay", "harmony1_overlay");
    // generate_combo_mix!(2, "blank_overlay", "harmony2_overlay");
    // generate_combo_mix!(3, "the_moon_overlay", "harmony3_overlay");
    // generate_combo_mix!(4, "statue_overlay", "harmony4_overlay");

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

const IAC: &str = "IAC Driver Bus 1";
const IAC_GLSL: &str = "iac_driver_bus_1";
const MIDI_DEVICE_VARS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(IAC.to_string(), IAC_GLSL.to_string());
    m
});

#[no_mangle]
pub fn calculate(
    #[allow(unused)] canvas_w: u32,
    #[allow(unused)] canvas_h: u32,
    #[allow(unused)] frame: i64,
    #[allow(unused)] fps: i64,
    #[allow(unused)] gfx_info: &HashMap<String, GfxInfo>,
    #[allow(unused)] reg_events: &[GfxEvent],
) -> Result<Vec<RenderSpec>, Box<dyn Error>> {
    // setup midi callback
    static MIDI_CALLBACK_CHANNELS: LazyLock<Mutex<(Sender<SendCmd>, Receiver<SendCmd>)>> =
        LazyLock::new(|| Mutex::new(channel::<SendCmd>()));

    let midi_channels = MIDI_CALLBACK_CHANNELS.lock().unwrap();

    static mut LAST_PRESET: usize = 0;
    static mut LAST_VELOCITY: u8 = 0;
    let _cb_tx = midi_channels.0.clone();
    let undead_callback = move |all_settings: &mut AllSettings, event: &MidiEvent| {
        for i in 0..all_settings.playback.len() {
            if all_settings.playback[i].stream.ident.name == "undead" {
                let settings = &mut all_settings.playback[i];
                match (
                    event.device.as_str(),
                    event.channel,
                    event.kind,
                    event.key,
                    event.velocity,
                ) {
                    ("IAC Driver Bus 1", 0, MIDI_CONTROL_CHANGE, 1, v) => {
                        if v > unsafe { LAST_VELOCITY } + 20 {
                            for j in 0..settings.presets.saved.len() {
                                if j != unsafe { LAST_PRESET } {
                                    continue;
                                }
                                eprintln!("Applying preset {} to undead stream", j);
                                let changes = settings.presets.saved[j].clone();
                                for k in 0..changes.len() {
                                    if changes[k].field == StreamSettingsAllFieldsEnum::EXACT_SEC {
                                        settings.stream.apply_diff(&[changes[k].clone()]);
                                    }
                                }
                            }
                            unsafe {
                                LAST_PRESET = (LAST_PRESET + 1) % 6;
                            }
                        }
                        unsafe {
                            LAST_VELOCITY = v;
                        }
                        if v > 50 {
                            settings
                                .stream
                                .set_threshold(0.1 - 0.1 * v as f64 / 127.0 + 0.005);
                        } else {
                            settings.stream.set_threshold(0.3 - 0.3 * v as f64 / 127.0);
                        }
                    }
                    ("IAC Driver Bus 1", 0, MIDI_NOTE_ON, 41, _v) => {
                        settings.stream.set_warp_level(0.7);
                    }
                    ("IAC Driver Bus 1", 0, MIDI_NOTE_OFF, 41, _v) => {
                        settings.stream.set_warp_level(0.0);
                    }
                    _ => eprintln!(
                        "Undead MIDI event unmatched: device='{}' chan={} kind={} key={} vel={}",
                        event.device, event.channel, event.kind, event.key, event.velocity
                    ),
                }
                return;
            }
        }
    };

    let cb_tx = midi_channels.0.clone();
    let _midi_callback = move |_all_settings: &mut AllSettings, event: &MidiEvent| {
        static MIXES: [&str; 1] = ["castle_combo_mix"];
        if let Some(glsl_device) = MIDI_DEVICE_VARS.get(&event.device) {
            for mix in MIXES.iter() {
                match event.kind {
                    MIDI_NOTE_ON => {
                        let cmd = SendCmd {
                            mix: mix.to_string(),
                            name: format!("note_{}_{}_{}", glsl_device, event.channel, event.key)
                                .to_string(),
                            value: SendValue::Float(event.velocity as f32),
                        };
                        // eprintln!("Sending MIDI Note ON command to glsl: {:?}", cmd);
                        cb_tx.send(cmd).ok();
                        cb_tx
                            .send(SendCmd {
                                mix: mix.to_string(),
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
                            mix: mix.to_string(),
                            name: format!("note_{}_{}_{}", glsl_device, event.channel, event.key)
                                .to_string(),
                            value: SendValue::Float(0.0),
                        };
                        // eprintln!("Sending MIDI Note OFF command to glsl: {:?}", cmd);
                        cb_tx.send(cmd).ok();
                        cb_tx
                            .send(SendCmd {
                                mix: mix.to_string(),
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
                            mix: mix.to_string(),
                            name: format!("cc_{}_{}_{}", glsl_device, event.channel, event.key)
                                .to_string(),
                            value: SendValue::Float(event.velocity as f32),
                        };
                        // eprintln!("Sending MIDI CC command to glsl: {:?}", cmd);
                        cb_tx.send(cmd).ok();
                    }
                    _ => (),
                }
            }
        }

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
    };

    let mut lock = SETTINGS.lock().expect("Settings mutex corrupted");
    let settings = lock.as_mut();

    let mut specs =
        settings.update_record_and_get_specs(reg_events, frame, Some(undead_callback))?;

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
    for cmd in midi_channels.1.try_iter() {
        specs.push(RenderSpec::SendCmd(cmd));
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
