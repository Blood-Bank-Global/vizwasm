use std::{
    collections::HashMap,
    error::Error,
    sync::{LazyLock, Mutex},
};

use sdlrig::{
    gfxinfo::{Asset, GfxEvent, GfxInfo, Vid, VidMixer},
    renderspec::{Mix, RenderSpec},
};
use vizwasm::vizconfig::{AllSettings, MixConfig};
fn main() {}

static ASSET_PATH: &'static str = "/Users/ttie/Desktop/tech";

static STREAM_DEFS: LazyLock<Vec<Vid>> = LazyLock::new(|| {
    vec![
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
        Vid::builder()
            .name("blob")
            .path(&format!("{ASSET_PATH}/streams/blob.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build(),
        Vid::builder()
            .name("blur_lights")
            .path(&format!("{ASSET_PATH}/streams/blur_lights.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("burns")
            .path(&format!("{ASSET_PATH}/streams/burns.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("circles")
            .path(&format!("{ASSET_PATH}/streams/circles.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("circuit1")
            .path(&format!("{ASSET_PATH}/streams/circuit1.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("circuit2")
            .path(&format!("{ASSET_PATH}/streams/circuit2.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("clock")
            .path(&format!("{ASSET_PATH}/streams/clock.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("corp")
            .path(&format!("{ASSET_PATH}/streams/corp.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("cube")
            .path(&format!("{ASSET_PATH}/streams/cube.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("dna")
            .path(&format!("{ASSET_PATH}/streams/dna.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("epic")
            .path(&format!("{ASSET_PATH}/streams/epic.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("ether")
            .path(&format!("{ASSET_PATH}/streams/ether.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("volume")
            .path(&format!("{ASSET_PATH}/streams/volume.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("fluffy_clouds")
            .path(&format!("{ASSET_PATH}/streams/fluffy_clouds.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("holo_city")
            .path(&format!("{ASSET_PATH}/streams/holo_city.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("masque")
            .path(&format!("{ASSET_PATH}/streams/masque.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("night_sky")
            .path(&format!("{ASSET_PATH}/streams/night_sky.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("nyc")
            .path(&format!("{ASSET_PATH}/streams/nyc.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("old_clouds")
            .path(&format!("{ASSET_PATH}/streams/old_clouds.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("phone")
            .path(&format!("{ASSET_PATH}/streams/phone.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("silver_lining")
            .path(&format!("{ASSET_PATH}/streams/silver_lining.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("stars")
            .path(&format!("{ASSET_PATH}/streams/stars.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("target")
            .path(&format!("{ASSET_PATH}/streams/target.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("tv")
            .path(&format!("{ASSET_PATH}/streams/tv.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("vestial1")
            .path(&format!("{ASSET_PATH}/streams/vestial1.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("vestial2")
            .path(&format!("{ASSET_PATH}/streams/vestial2.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("world_grid")
            .path(&format!("{ASSET_PATH}/streams/world_grid.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("the_moon")
            .path(&format!("{ASSET_PATH}/streams/the_moon.mp4"))
            .resolution((720, 480))
            .tbq((1, 12800))
            .pix_fmt("yuv420p")
            .repeat(true)
            .realtime(false)
            .hardware_decode(true)
            .build()
            .into(),
        Vid::builder()
            .name("front cam")
            .path("MacBook Pro Camera")
            .format("avfoundation")
            .opts(&vec![
                ("pixel_format", "bgr0"),
                ("framerate", "30.0"),
                ("video_size", "1280x720"),
            ])
            .resolution((1280, 720))
            .tbq((1, 1000000))
            .pix_fmt("bgr0")
            .repeat(false)
            .realtime(true)
            .hardware_decode(false)
            .build()
            .into(),
        //   Vid::builder()
        //     .name("capture")
        //     .path("USB3 Video")
        //     .format("avfoundation")
        //     .resolution((1280, 720))
        //     .tbq((1, 1000000))
        //     .pix_fmt("bgr0")
        //     .opts(&vec![
        //         ("pixel_format", "bgr0"),
        //         ("framerate", "30.0"),
        //         ("video_size", "1280x720"),
        //     ])
        //     .realtime(true)
        //     .repeat(false)
        //     .build(),
        //   Vid::builder()
        //     .name("StreamCam")
        //     .path("Logitech StreamCam")
        //     .format("avfoundation")
        //     .resolution((848, 480))
        //     .tbq((1, 1000000))
        //     .pix_fmt("bgr0")
        //     .opts(&vec![
        //         ("pixel_format", "bgr0"),
        //         ("framerate", "30.0"),
        //         ("video_size", "848,480"),
        //     ])
        //     .realtime(true)
        //     .repeat(false)
        //     .build(),
    ]
});

static PLAYBACK_NAMES: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        "blank",
        "generate",
        "grid",
        "compose",
        "blood",
        "blob",
        "blur_lights",
        "burns",
        "circles",
        "circuit1",
        "circuit2",
        "clock",
        "corp",
        "cube",
        "dna",
        "epic",
        "ether",
        "volume",
        "fluffy_clouds",
        "holo_city",
        "masque",
        "night_sky",
        "nyc",
        "old_clouds",
        "phone",
        "silver_lining",
        "stars",
        "target",
        "tv",
        "vestial1",
        "vestial2",
        "world_grid",
        "the_moon",
    ]
});

static MIX_CONFIGS: LazyLock<Vec<MixConfig>> = LazyLock::new(|| {
    let mut configs = vec![];
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("generate_mix")
            .width(720)
            .height(480)
            .header(include_str!("../glsl/utils.glsl"))
            .body(include_str!("../glsl/generate.glsl"))
            .build(),
        mix: Mix::builder()
            .name("generate_mix")
            .mixed("blank_mix")
            .no_display(true)
            .build(),
    });
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("blood_mix")
            .width(720)
            .height(480)
            .header(concat!(
                include_str!("../glsl/utils.glsl"),
                "\n",
                include_str!("../glsl/blood_funcs.glsl")
            ))
            .body(include_str!("../glsl/blood.glsl"))
            .build(),
        mix: Mix::builder()
            .name("blood_mix")
            .mixed("blank_mix")
            .no_display(true)
            .build(),
    });
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("grid_mix")
            .width(720)
            .height(480)
            .build(),
        mix: Mix::builder()
            .name("grid_mix")
            .mixed("generate_feedback")
            .no_display(true)
            .build(),
    });
    configs.push(MixConfig {
        def: VidMixer::builder()
            .name("compose_mix")
            .header(include_str!("../glsl/utils.glsl"))
            .body(include_str!("../glsl/compose.glsl"))
            .width(720)
            .height(480)
            .build(),
        mix: Mix::builder()
            .name("compose_mix")
            .mixed("fluffy_clouds_main_mix")
            .mixed("generate_feedback")
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
    *settings = serde_json::from_slice(bytes).unwrap();
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

    // LHS
    let mix_name = settings.playback[settings.active_idx].stream.overlay_mix();
    if let Some(mix_config) = settings.mix_configs.get(&mix_name) {
        let iw = mix_config.def.width as i32;
        let ih = mix_config.def.height as i32;
        let mut ow = iw;
        let mut oh = ih;
        let mut ix = 0;
        let mut iy = 0;

        let iaspect = iw as f32 / ih as f32;
        let oaspect = (canvas_w / 2) as f32 / canvas_h as f32;

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
        let dst = (0, 0, canvas_w / 2 as u32, canvas_h as u32);

        specs.extend(settings.get_playback_specs(settings.active_idx, src, dst));
    }

    // RHS
    let mix_name = settings.playback[settings.display_idx].stream.overlay_mix();
    if let Some(mix_config) = settings.mix_configs.get(&mix_name) {
        let iw = mix_config.def.width as i32;
        let ih = mix_config.def.height as i32;
        let mut ow = iw;
        let mut oh = ih;
        let mut ix = 0;
        let mut iy = 0;

        let iaspect = iw as f32 / ih as f32;
        let oaspect = (canvas_w / 2) as f32 / canvas_h as f32;

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
        let dst = (canvas_w as i32 / 2, 0, canvas_w / 2 as u32, canvas_h as u32);
        specs.extend(settings.get_playback_specs(settings.display_idx, src, dst));
    }

    Ok(specs)
}
