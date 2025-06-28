use std::{
    collections::{HashMap, HashSet},
    error::Error,
    sync::{LazyLock, Mutex},
};

use sdlrig::{
    gfxinfo::{Asset, GfxEvent, GfxInfo, Vid, VidMixer},
    renderspec::{Mix, MixInput, RenderSpec, Reset},
};
use vizwasm::vizconfig::{AllSettings, MixConfig, StreamSettingsAllFieldsEnum};
fn main() {}

static ASSET_PATH: &'static str = "/Users/ttie/Desktop/ninetytwo";

static CLIP_COUNTS: LazyLock<HashMap<String, usize>> = LazyLock::new(|| {
    HashMap::from([
        ("secrets".to_string(), 34),
        ("danger".to_string(), 12),
        ("drop".to_string(), 19),
        ("heist".to_string(), 72),
        ("hideout".to_string(), 15),
        ("intro".to_string(), 13),
        ("phreak".to_string(), 14),
        ("prof".to_string(), 18),
        ("sound".to_string(), 22),
        ("truck".to_string(), 16),
        ("motzart".to_string(), 112),
        ("vacation".to_string(), 190),
        ("return".to_string(), 91),
        ("chat".to_string(), 46),
        ("carnival".to_string(), 60),
        ("office".to_string(), 128),
        ("convention".to_string(), 100),
    ])
});

static STREAM_DEFS: LazyLock<Vec<Vid>> = LazyLock::new(|| {
    let mut vids = vec![Vid::builder()
        .name("blank")
        .path(format!("{ASSET_PATH}/streams/blank.mp4"))
        .resolution((720, 480))
        .tbq((1, 12800))
        .pix_fmt("yuv420p")
        .repeat(true)
        .realtime(false)
        .hardware_decode(true)
        .build()];

    //secrets
    for i in 1..=CLIP_COUNTS["secrets"] {
        vids.push(
            Vid::builder()
                .name(format!("secrets{}", i))
                .path(format!("{ASSET_PATH}/streams/secrets-{i:02}.mp4"))
                .resolution((1920, 1080))
                .tbq((1, 15360))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }

    //danger
    for i in 1..=CLIP_COUNTS["danger"] {
        vids.push(
            Vid::builder()
                .name(format!("danger{}", i))
                .path(format!("{ASSET_PATH}/streams/danger-{i:02}.mp4"))
                .resolution((1920, 1080))
                .tbq((1, 15360))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }

    //drop
    for i in 1..=CLIP_COUNTS["drop"] {
        vids.push(
            Vid::builder()
                .name(format!("drop{}", i))
                .path(format!("{ASSET_PATH}/streams/drop-{i:02}.mp4"))
                .resolution((1920, 1080))
                .tbq((1, 15360))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }

    //heist
    for i in 1..=CLIP_COUNTS["heist"] {
        vids.push(
            Vid::builder()
                .name(format!("heist{}", i))
                .path(format!("{ASSET_PATH}/streams/heist-{i:02}.mp4"))
                .resolution((1920, 1080))
                .tbq((1, 15360))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }

    //hideout
    for i in 1..=CLIP_COUNTS["hideout"] {
        vids.push(
            Vid::builder()
                .name(format!("hideout{}", i))
                .path(format!("{ASSET_PATH}/streams/hideout-{i:02}.mp4"))
                .resolution((1920, 1080))
                .tbq((1, 15360))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }

    //intro
    for i in 1..=CLIP_COUNTS["intro"] {
        vids.push(
            Vid::builder()
                .name(format!("intro{}", i))
                .path(format!("{ASSET_PATH}/streams/intro-{i:02}.mp4"))
                .resolution((1920, 1080))
                .tbq((1, 15360))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }

    //phreak
    for i in 1..=CLIP_COUNTS["phreak"] {
        vids.push(
            Vid::builder()
                .name(format!("phreak{}", i))
                .path(format!("{ASSET_PATH}/streams/phreak-{i:02}.mp4"))
                .resolution((1920, 1080))
                .tbq((1, 15360))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }

    //prof
    for i in 1..=CLIP_COUNTS["prof"] {
        vids.push(
            Vid::builder()
                .name(format!("prof{}", i))
                .path(format!("{ASSET_PATH}/streams/prof-{i:02}.mp4"))
                .resolution((1920, 1080))
                .tbq((1, 15360))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }

    //sound
    for i in 1..=CLIP_COUNTS["sound"] {
        vids.push(
            Vid::builder()
                .name(format!("sound{}", i))
                .path(format!("{ASSET_PATH}/streams/sound-{i:02}.mp4"))
                .resolution((1920, 1080))
                .tbq((1, 15360))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }

    //truck
    for i in 1..=CLIP_COUNTS["truck"] {
        vids.push(
            Vid::builder()
                .name(format!("truck{}", i))
                .path(format!("{ASSET_PATH}/streams/truck-{i:02}.mp4"))
                .resolution((1920, 1080))
                .tbq((1, 15360))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }

    // motzart
    for i in 1..=CLIP_COUNTS["motzart"] {
        vids.push(
            Vid::builder()
                .name(format!("motzart{}", i))
                .path(format!("{ASSET_PATH}/streams/net/motzart-{i:03}.mp4"))
                .resolution((1920, 1080))
                .tbq((1, 15360))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }
    // vacation
    for i in 1..=CLIP_COUNTS["vacation"] {
        vids.push(
            Vid::builder()
                .name(format!("vacation{}", i))
                .path(format!("{ASSET_PATH}/streams/net/vacation-{i:03}.mp4"))
                .resolution((1920, 1080))
                .tbq((1, 15360))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }
    // return
    for i in 1..=CLIP_COUNTS["return"] {
        vids.push(
            Vid::builder()
                .name(format!("return{}", i))
                .path(format!("{ASSET_PATH}/streams/net/return-{i:03}.mp4"))
                .resolution((1920, 1080))
                .tbq((1, 15360))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }

    // chat
    for i in 1..=CLIP_COUNTS["chat"] {
        vids.push(
            Vid::builder()
                .name(format!("chat{}", i))
                .path(format!("{ASSET_PATH}/streams/net/chat-{i:03}.mp4"))
                .resolution((1920, 1080))
                .tbq((1, 15360))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }

    // carnival
    for i in 1..=CLIP_COUNTS["carnival"] {
        vids.push(
            Vid::builder()
                .name(format!("carnival{}", i))
                .path(format!("{ASSET_PATH}/streams/net/carnival-{i:03}.mp4"))
                .resolution((1920, 1080))
                .tbq((1, 15360))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }
    // office
    for i in 1..=CLIP_COUNTS["office"] {
        vids.push(
            Vid::builder()
                .name(format!("office{}", i))
                .path(format!("{ASSET_PATH}/streams/net/office-{i:03}.mp4"))
                .resolution((1920, 1080))
                .tbq((1, 15360))
                .pix_fmt("yuv420p")
                .repeat(true)
                .realtime(false)
                .hardware_decode(true)
                .build(),
        );
    }
    // convention
    for i in 1..=CLIP_COUNTS["convention"] {
        vids.push(
            Vid::builder()
                .name(format!("convention{}", i))
                .path(format!("{ASSET_PATH}/streams/net/convention-{i:03}.mp4"))
                .resolution((1920, 1080))
                .tbq((1, 15360))
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
        "intro".to_string(),      // Jericho
        "truck".to_string(),      // Music Reach
        "prof".to_string(),       // Your Love
        "secrets".to_string(),    // Out of Space
        "drop".to_string(),       // Everybody in the Place
        "hideout".to_string(),    // The Weather Experience
        "phreak".to_string(),     // The Weather Experience
        "danger".to_string(),     // Fire
        "sound".to_string(),      // Ruff In the Jungle
        "heist".to_string(),      // Ruff In the Jungle + Death of the Prodigy Dancers
        "motzart".to_string(),    // black cat + ghosts
        "vacation".to_string(),   // I'm not scared + Runaway
        "return".to_string(),     // Season of Illusions + Burning Up
        "chat".to_string(),       // Kletva
        "carnival".to_string(),   // they gave you a heart + predict the day
        "office".to_string(),     // the lovers + deep blue
        "convention".to_string(), // tomorrow + versus
    ];
    names
});

static MIX_CONFIGS: LazyLock<Vec<MixConfig>> = LazyLock::new(|| {
    let mut configs = vec![];
    //secrets
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("secrets_mix")
            .header(include_str!("../glsl/utils.glsl"))
            .width(1280)
            .height(720)
            .build(),
        mix: Mix::builder()
            .name("secrets_mix")
            .video("secrets1")
            .no_display(true)
            .build(),
    });
    // danger
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("danger_mix")
            .header(include_str!("../glsl/utils.glsl"))
            .width(1280)
            .height(720)
            .build(),
        mix: Mix::builder()
            .name("danger_mix")
            .video("danger1")
            .no_display(true)
            .build(),
    });
    // drop
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("drop_mix")
            .header(include_str!("../glsl/utils.glsl"))
            .width(1280)
            .height(720)
            .build(),
        mix: Mix::builder()
            .name("drop_mix")
            .video("drop1")
            .no_display(true)
            .build(),
    });
    // heist
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("heist_mix")
            .header(include_str!("../glsl/utils.glsl"))
            .width(1280)
            .height(720)
            .build(),
        mix: Mix::builder()
            .name("heist_mix")
            .video("heist1")
            .no_display(true)
            .build(),
    });
    // hideout
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("hideout_mix")
            .header(include_str!("../glsl/utils.glsl"))
            .width(1280)
            .height(720)
            .build(),
        mix: Mix::builder()
            .name("hideout_mix")
            .video("hideout1")
            .no_display(true)
            .build(),
    });
    // intro
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("intro_mix")
            .header(include_str!("../glsl/utils.glsl"))
            .width(1280)
            .height(720)
            .build(),
        mix: Mix::builder()
            .name("intro_mix")
            .video("intro1")
            .no_display(true)
            .build(),
    });
    // phreak
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("phreak_mix")
            .header(include_str!("../glsl/utils.glsl"))
            .width(1280)
            .height(720)
            .build(),
        mix: Mix::builder()
            .name("phreak_mix")
            .video("phreak1")
            .no_display(true)
            .build(),
    });
    // prof
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("prof_mix")
            .header(include_str!("../glsl/utils.glsl"))
            .width(1280)
            .height(720)
            .build(),
        mix: Mix::builder()
            .name("prof_mix")
            .video("prof1")
            .no_display(true)
            .build(),
    });
    // sound
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("sound_mix")
            .header(include_str!("../glsl/utils.glsl"))
            .width(1280)
            .height(720)
            .build(),
        mix: Mix::builder()
            .name("sound_mix")
            .video("sound1")
            .no_display(true)
            .build(),
    });
    // truck
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("truck_mix")
            .header(include_str!("../glsl/utils.glsl"))
            .width(1280)
            .height(720)
            .build(),
        mix: Mix::builder()
            .name("truck_mix")
            .video("truck1")
            .no_display(true)
            .build(),
    });
    // motzart
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("motzart_mix")
            .header(include_str!("../glsl/utils.glsl"))
            .width(1280)
            .height(720)
            .build(),
        mix: Mix::builder()
            .name("motzart_mix")
            .video("motzart1")
            .no_display(true)
            .build(),
    });
    // vacation
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("vacation_mix")
            .header(include_str!("../glsl/utils.glsl"))
            .width(1280)
            .height(720)
            .build(),
        mix: Mix::builder()
            .name("vacation_mix")
            .video("vacation1")
            .no_display(true)
            .build(),
    });
    // return
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("return_mix")
            .header(include_str!("../glsl/utils.glsl"))
            .width(1280)
            .height(720)
            .build(),
        mix: Mix::builder()
            .name("return_mix")
            .video("return1")
            .no_display(true)
            .build(),
    });
    // chat
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("chat_mix")
            .header(include_str!("../glsl/utils.glsl"))
            .width(1280)
            .height(720)
            .build(),
        mix: Mix::builder()
            .name("chat_mix")
            .video("chat1")
            .no_display(true)
            .build(),
    });
    // carnival
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("carnival_mix")
            .header(include_str!("../glsl/utils.glsl"))
            .width(1280)
            .height(720)
            .build(),
        mix: Mix::builder()
            .name("carnival_mix")
            .video("carnival1")
            .no_display(true)
            .build(),
    });
    // office
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("office_mix")
            .header(include_str!("../glsl/utils.glsl"))
            .width(1280)
            .height(720)
            .build(),
        mix: Mix::builder()
            .name("office_mix")
            .video("office1")
            .no_display(true)
            .build(),
    });
    // convention
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("convention_mix")
            .header(include_str!("../glsl/utils.glsl"))
            .width(1280)
            .height(720)
            .build(),
        mix: Mix::builder()
            .name("convention_mix")
            .video("convention1")
            .no_display(true)
            .build(),
    });
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

    // this is to make sure if there are changes to static data we pick it up here
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
    let usr_var = settings.playback[settings.active_idx].stream.usr_var();
    let input_name = settings.playback[settings.active_idx].stream.input_mix();
    if let Some(mix_config) = settings.mix_configs.get_mut(&input_name) {
        update_input(usr_var as i32, mix_config)?;
    }
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

        let playback_specs = settings.get_playback_specs(mix_name, src, dst);
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
    let usr_var = settings.playback[settings.active_idx].stream.usr_var();
    let input_name = settings.playback[settings.active_idx].stream.input_mix();
    if let Some(mix_config) = settings.mix_configs.get_mut(&input_name) {
        update_input(usr_var as i32, mix_config)?;
    }
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

        let playback_specs = settings.get_playback_specs(mix_name, src, dst);
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

    fix_seeks(&mut specs, settings);
    reset_vids(&mut specs, settings);
    settings.clean_up_by_specs(&mut specs);
    Ok(specs)
}

fn reset_vids(specs: &mut Vec<RenderSpec>, settings: &mut AllSettings) {
    let mut seen = HashSet::new();
    for spec in specs.iter() {
        if let RenderSpec::Mix(mix) = spec {
            seen.insert(mix.name.clone());
        }
    }

    for (i, playback) in settings.playback.iter().enumerate() {
        if !settings.initial_reset_complete[i] {
            continue;
        }
        if seen.contains(&playback.stream.input_mix()) {
            // If the input mix is seen, skip resetting the video.
            continue;
        }

        if let Some(prefix) = playback.stream.input_mix().strip_suffix("_mix") {
            if let Some(count) = CLIP_COUNTS.get(prefix) {
                if !seen.contains(&playback.stream.input_mix()) {
                    eprintln!("Resetting videos for {}", playback.stream.input_mix());
                    // If the input mix is not seen, reset all the videos.
                    for i in 1..=*count {
                        specs.push(RenderSpec::Reset(Reset {
                            target: format!("{prefix}{i}"),
                        }));
                    }
                }
            } else {
                eprintln!(
                    "Skipping reset for {} as it does not have a valid count",
                    playback.stream.input_mix()
                );
            }
        } else {
            eprintln!(
                "Skipping reset for {} as it does not have a valid prefix",
                playback.stream.input_mix()
            );
        }
    }
}

fn rotate_usr_var(usr_var: i32, count: usize) -> i32 {
    // if count == 0 {
    //     return usr_var;
    // }
    // let usr_var = if usr_var < 0 { usr_var + 201 } else { usr_var };
    usr_var.rem_euclid(count as i32) + 1
}

fn update_input(usr_var: i32, mix_config: &mut MixConfig) -> Result<(), Box<dyn Error>> {
    if let Some(prefix) = mix_config.def.name.strip_suffix("_mix") {
        if CLIP_COUNTS.contains_key(prefix) {
            let count = CLIP_COUNTS[prefix];
            if let Some(MixInput::Video(inp)) = mix_config.mix.inputs.get_mut(0) {
                *inp = format!("{prefix}{}", rotate_usr_var(usr_var, count));
            }
        }
    }
    Ok(())
}

fn fix_seeks(specs: &mut [RenderSpec], settings: &mut AllSettings) {
    for event in specs.iter_mut() {
        if let RenderSpec::SeekVid(seek) = event {
            for playback in &settings.playback {
                if playback.stream.first_video() == seek.target {
                    if let Some(prefix) = playback.stream.input_mix().strip_suffix("_mix") {
                        if let Some(count) = CLIP_COUNTS.get(prefix) {
                            seek.target = format!(
                                "{prefix}{}",
                                rotate_usr_var(playback.stream.usr_var() as i32, *count)
                            );
                        }
                    }
                }
            }
        }
    }
}
