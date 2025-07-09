use std::{
    collections::HashMap,
    error::Error,
    sync::{LazyLock, Mutex},
};

use sdlrig::{
    gfxinfo::{Asset, GfxEvent, GfxInfo, Vid, VidMixer},
    renderspec::{Mix, MixInput, RenderSpec},
};
use vizwasm::vizconfig::{AllSettings, MixConfig, StreamSettingsAllFieldsEnum};
fn main() {}

static ASSET_PATH: &'static str = "/Users/ttie/Desktop/news_clips";

static STREAM_DEFS: LazyLock<Vec<Vid>> = LazyLock::new(|| {
    let mut vids = vec![];
    vids.push(
        Vid::builder()
            .name("blank")
            .path(format!("{ASSET_PATH}/streams/blank.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build(),
    );
    let vid1280x720 = ["buzz", "phobos", "monolith", "fly", "bars", "two_moons"];
    for vid_name in vid1280x720.iter() {
        vids.push(
            Vid::builder()
                .name(vid_name)
                .path(format!("{ASSET_PATH}/streams/{}.mp4", vid_name))
                .resolution((1280, 720))
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
        "blank".to_string(),
        "buzz".to_string(),
        "buzz_shuffle".to_string(),
        "phobos".to_string(),
        "monolith".to_string(),
        "full".to_string(),
        "fly".to_string(),
        "bars".to_string(),
        "two_moons".to_string(),
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

    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("buzz_shuffle_mix")
            .header(include_str!("../glsl/utils.glsl"))
            .body(include_str!("../glsl/shuffle.glsl"))
            .width(1280)
            .height(720)
            .build(),
        mix: Mix::builder()
            .name("buzz_shuffle_mix")
            .mixed("buzz_mix")
            .no_display(true)
            .build(),
    });

    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("full_mix")
            .header(include_str!("../glsl/utils.glsl"))
            .width(1280)
            .height(720)
            .build(),
        mix: Mix::builder()
            .name("full_mix")
            .mixed("buzz_shuffle_feedback")
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
            ASSET_PATH,
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

    static FULL_INPUTS: LazyLock<Vec<MixInput>> = LazyLock::new(|| {
        vec![
            MixInput::Mixed("buzz_shuffle_overlay".to_string()),
            MixInput::Mixed("phobos_overlay".to_string()),
            MixInput::Mixed("monolith_overlay".to_string()),
            MixInput::Mixed("fly_overlay".to_string()),
            MixInput::Mixed("bars_overlay".to_string()),
        ]
    });

    let mix = settings.mix_configs.iter_mut().find(|m| m.0 == "full_mix");
    let playback = settings
        .playback
        .iter()
        .find(|p| p.stream.ident.name == "full");

    if let Some((_, mix)) = mix {
        if let Some(playback) = playback {
            mix.mix.inputs[0] = FULL_INPUTS
                .get(playback.stream.usr_var() as usize)
                .unwrap_or(&MixInput::Mixed("blank_overlay".to_string()))
                .clone();
        }
    }

    let mut specs = settings.update_record_and_get_specs(reg_events, frame)?;

    // Wire up usr_toggle to actually count up usr_var as well every change
    let orig = settings.playback[settings.active_idx].stream.usr_var();
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
            .get_commands(&[StreamSettingsAllFieldsEnum::USR_VAR(orig)]),
    );

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

    for inp in FULL_INPUTS.iter() {
        if let MixInput::Mixed(src) = inp {
            specs.append(&mut settings.get_playback_specs(&src, (1, 1, 1, 1), (1, 1, 1, 1)));
        }
    }

    settings.clean_up_by_specs(&mut specs);
    Ok(to_return)
}
