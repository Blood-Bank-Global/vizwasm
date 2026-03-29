use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    sync::{LazyLock, Mutex},
};

#[allow(unused_imports)]
use sdlrig::gfxinfo::{MIDI_CONTROL_CHANGE, MIDI_NOTE_OFF, MIDI_NOTE_ON};

use sdlrig::{
    gfxinfo::{Asset, GfxEvent, GfxInfo, MidiEvent, Vid, VidMixer},
    renderspec::{Mix, RenderSpec, SendCmd, SendValue},
};

use vizwasm::{
    beat_time_boilerplate,
    shaderlookup::include_files,
    vizconfig::{time_code_2_float, AllSettings, MixConfig},
};

fn main() {}

static STREAM_PATH: &'static str = "/Users/ttie/Desktop/scratch/streams";
static TECH_PATH: &'static str = "/Users/ttie/Desktop/tech_streams";
static ASSET_PATH: &'static str = "/Users/ttie/Desktop/common_data";

static STREAM_DEFS: LazyLock<Vec<Vid>> = LazyLock::new(|| {
    let mut vids = vec![];

    let vid640x480: &[&str] = &["spring", "dino", "park_screen"];
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
            .mixed("distort_caustic_adjusted_dx_dx_mix")
            .mixed("distort_caustic_adjusted_dy_dy_mix")
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

    let reloaded = reg_events
        .iter()
        .any(|e| matches!(e, GfxEvent::ReloadEvent {}));

    static LAST_LOAD: Mutex<RefCell<u64>> = Mutex::new(RefCell::new(0));
    let lock = LAST_LOAD.lock().expect("LAST_LOAD mutex corrupted");
    let mut last_load = lock.borrow_mut();

    let dino_frame_mtime: u64 = std::fs::metadata(&format!("/tmp/viz/dino_frame.txt"))
        .and_then(|meta| meta.modified())
        .map(|mtime| {
            mtime
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        })
        .unwrap_or_default(); // Convert to u64

    if reloaded || *last_load < dino_frame_mtime {
        let dino_frame = std::fs::read_to_string(format!("/tmp/viz/dino_frame.txt"))
            .expect("Failed to read dino frame data from file");
        *last_load = dino_frame_mtime;

        let mut dino_bytes = vec![];
        let mut dino_lens = vec![];
        let mut dino_starts = vec![];
        for line in dino_frame.lines() {
            let bytes = unicode_to_cp437(line);
            dino_starts.push(dino_bytes.len() as u32);
            dino_lens.push(bytes.len() as u32);
            dino_bytes.extend(bytes);
        }

        specs.push(
            SendCmd::builder()
                .name("dino_frame")
                .mix("dino_glitch_mix")
                .value(SendValue::UVector(
                    dino_bytes.iter().map(|&b| b as u32).collect(),
                ))
                .build()
                .into(),
        );
        specs.push(
            SendCmd::builder()
                .name("dino_frame_starts")
                .mix("dino_glitch_mix")
                .value(SendValue::UVector(dino_starts))
                .build()
                .into(),
        );
        specs.push(
            SendCmd::builder()
                .name("dino_frame_lens")
                .mix("dino_glitch_mix")
                .value(SendValue::UVector(dino_lens))
                .build()
                .into(),
        );
    }

    specs.append(&mut settings.update_record_and_get_specs(reg_events, frame, Some(mega_cb))?);
    Ok(specs)
}

const _IAC: &str = "IAC Driver Bus 1";
pub fn mega_cb(all_settings: &mut AllSettings, event: &MidiEvent) {
    dino_cb(all_settings, event);
}

pub fn dino_cb(all_settings: &mut AllSettings, event: &MidiEvent) {
    let timing = vec![
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
    ];

    beat_time_boilerplate!(all_settings, event, "dino_glitch", timing);
}

pub static CP437: LazyLock<HashMap<char, u8>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert('\u{000000}', 0x00); // NULL
    m.insert('☺', 0x01); // WHITE SMILING FACE
    m.insert('\u{00263B}', 0x02); // BLACK SMILING FACE
    m.insert('\u{002665}', 0x03); // BLACK HEART SUIT
    m.insert('\u{002666}', 0x04); // BLACK DIAMOND SUIT
    m.insert('\u{002663}', 0x05); // BLACK CLUB SUIT
    m.insert('\u{002660}', 0x06); // BLACK SPADE SUIT
    m.insert('\u{002022}', 0x07); // BULLET
    m.insert('\u{0025D8}', 0x08); // INVERSE BULLET
    m.insert('\u{0025CB}', 0x09); // WHITE CIRCLE
    m.insert('\u{0025D9}', 0x0A); // INVERSE WHITE CIRCLE
    m.insert('\u{002642}', 0x0B); // MALE SIGN
    m.insert('\u{002640}', 0x0C); // FEMALE SIGN
    m.insert('\u{0026A5}', 0x0D); // MALE AND FEMALE SIGN, VARIANT
    m.insert('\u{00266B}', 0x0E); // BEAMED EIGHTH NOTES
    m.insert('\u{00263C}', 0x0F); // WHITE SUN WITH RAYS
    m.insert('\u{0025BA}', 0x10); // BLACK RIGHT-POINTING POINTER
    m.insert('\u{0025C4}', 0x11); // BLACK LEFT-POINTING POINTER
    m.insert('\u{002195}', 0x12); // UP DOWN ARROW
    m.insert('\u{00203C}', 0x13); // DOUBLE EXCLAMATION MARK
    m.insert('\u{0000B6}', 0x14); // PILCROW SIGN
    m.insert('\u{0000A7}', 0x15); // SECTION SIGN
    m.insert('\u{0025AC}', 0x16); // BLACK RECTANGLE
    m.insert('\u{0021A8}', 0x17); // UP DOWN ARROW WITH BASE
    m.insert('\u{002191}', 0x18); // UPWARDS ARROW
    m.insert('\u{002193}', 0x19); // DOWNWARDS ARROW
    m.insert('\u{002192}', 0x1A); // RIGHTWARDS ARROW
    m.insert('\u{002190}', 0x1B); // LEFTWARDS ARROW
    m.insert('\u{00221F}', 0x1C); // RIGHT ANGLE
    m.insert('\u{002194}', 0x1D); // LEFT RIGHT ARROW
    m.insert('\u{0025B2}', 0x1E); // BLACK UP-POINTING TRIANGLE
    m.insert('\u{0025BC}', 0x1F); // BLACK DOWN-POINTING TRIANGLE
                                  // BEGIN ASCII SECTION
    m.insert(' ', 0x20);
    m.insert('!', 0x21);
    m.insert('"', 0x22);
    m.insert('#', 0x23);
    m.insert('$', 0x24);
    m.insert('%', 0x25);
    m.insert('&', 0x26);
    m.insert('\'', 0x27);
    m.insert('(', 0x28);
    m.insert(')', 0x29);
    m.insert('*', 0x2A);
    m.insert('+', 0x2B);
    m.insert(',', 0x2C);
    m.insert('-', 0x2D);
    m.insert('.', 0x2E);
    m.insert('/', 0x2F);
    m.insert('0', 0x30);
    m.insert('1', 0x31);
    m.insert('2', 0x32);
    m.insert('3', 0x33);
    m.insert('4', 0x34);
    m.insert('5', 0x35);
    m.insert('6', 0x36);
    m.insert('7', 0x37);
    m.insert('8', 0x38);
    m.insert('9', 0x39);
    m.insert(':', 0x3A);
    m.insert(';', 0x3B);
    m.insert('<', 0x3C);
    m.insert('=', 0x3D);
    m.insert('>', 0x3E);
    m.insert('?', 0x3F);
    m.insert('@', 0x40);
    m.insert('A', 0x41);
    m.insert('B', 0x42);
    m.insert('C', 0x43);
    m.insert('D', 0x44);
    m.insert('E', 0x45);
    m.insert('F', 0x46);
    m.insert('G', 0x47);
    m.insert('H', 0x48);
    m.insert('I', 0x49);
    m.insert('J', 0x4A);
    m.insert('K', 0x4B);
    m.insert('L', 0x4C);
    m.insert('M', 0x4D);
    m.insert('N', 0x4E);
    m.insert('O', 0x4F);
    m.insert('P', 0x50);
    m.insert('Q', 0x51);
    m.insert('R', 0x52);
    m.insert('S', 0x53);
    m.insert('T', 0x54);
    m.insert('U', 0x55);
    m.insert('V', 0x56);
    m.insert('W', 0x57);
    m.insert('X', 0x58);
    m.insert('Y', 0x59);
    m.insert('Z', 0x5A);
    m.insert('[', 0x5B);
    m.insert('\\', 0x5C);
    m.insert(']', 0x5D);
    m.insert('^', 0x5E);
    m.insert('_', 0x5F);
    m.insert('`', 0x60);
    m.insert('a', 0x61);
    m.insert('b', 0x62);
    m.insert('c', 0x63);
    m.insert('d', 0x64);
    m.insert('e', 0x65);
    m.insert('f', 0x66);
    m.insert('g', 0x67);
    m.insert('h', 0x68);
    m.insert('i', 0x69);
    m.insert('j', 0x6A);
    m.insert('k', 0x6B);
    m.insert('l', 0x6C);
    m.insert('m', 0x6D);
    m.insert('n', 0x6E);
    m.insert('o', 0x6F);
    m.insert('p', 0x70);
    m.insert('q', 0x71);
    m.insert('r', 0x72);
    m.insert('s', 0x73);
    m.insert('t', 0x74);
    m.insert('u', 0x75);
    m.insert('v', 0x76);
    m.insert('w', 0x77);
    m.insert('x', 0x78);
    m.insert('y', 0x79);
    m.insert('z', 0x7A);
    m.insert('{', 0x7B);
    m.insert('|', 0x7C);
    m.insert('}', 0x7D);
    m.insert('~', 0x7E);
    // END ASCII SECTION
    m.insert('\u{002302}', 0x7F); // HOUSE
    m.insert('\u{0000C7}', 0x80); // LATIN CAPITAL LETTER C WITH CEDILLA
    m.insert('\u{0000FC}', 0x81); // LATIN SMALL LETTER U WITH DIAERESIS
    m.insert('\u{0000E9}', 0x82); // LATIN SMALL LETTER E WITH ACUTE
    m.insert('\u{0000E2}', 0x83); // LATIN SMALL LETTER A WITH CIRCUMFLEX
    m.insert('\u{0000E4}', 0x84); // LATIN SMALL LETTER A WITH DIAERESIS
    m.insert('\u{0000E0}', 0x85); // LATIN SMALL LETTER A WITH GRAVE
    m.insert('\u{0000E5}', 0x86); // LATIN SMALL LETTER A WITH RING ABOVE
    m.insert('\u{0000E7}', 0x87); // LATIN SMALL LETTER C WITH CEDILLA
    m.insert('\u{0000EA}', 0x88); // LATIN SMALL LETTER E WITH CIRCUMFLEX
    m.insert('\u{0000EB}', 0x89); // LATIN SMALL LETTER E WITH DIAERESIS
    m.insert('\u{0000E8}', 0x8A); // LATIN SMALL LETTER E WITH GRAVE
    m.insert('\u{0000EF}', 0x8B); // LATIN SMALL LETTER I WITH DIAERESIS
    m.insert('\u{0000EE}', 0x8C); // LATIN SMALL LETTER I WITH CIRCUMFLEX
    m.insert('\u{0000EC}', 0x8D); // LATIN SMALL LETTER I WITH GRAVE
    m.insert('\u{0000C4}', 0x8E); // LATIN CAPITAL LETTER A WITH DIAERESIS
    m.insert('\u{0000C5}', 0x8F); // LATIN CAPITAL LETTER A WITH RING ABOVE
    m.insert('\u{0000C9}', 0x90); // LATIN CAPITAL LETTER E WITH ACUTE
    m.insert('\u{0000E6}', 0x91); // LATIN SMALL LETTER AE
    m.insert('\u{0000C6}', 0x92); // LATIN CAPITAL LETTER AE
    m.insert('\u{0000F4}', 0x93); // LATIN SMALL LETTER O WITH CIRCUMFLEX
    m.insert('\u{0000F6}', 0x94); // LATIN SMALL LETTER O WITH DIAERESIS
    m.insert('\u{0000F2}', 0x95); // LATIN SMALL LETTER O WITH GRAVE
    m.insert('\u{0000FB}', 0x96); // LATIN SMALL LETTER U WITH CIRCUMFLEX
    m.insert('\u{0000F9}', 0x97); // LATIN SMALL LETTER U WITH GRAVE
    m.insert('\u{0000FF}', 0x98); // LATIN SMALL LETTER Y WITH DIAERESIS
    m.insert('\u{0000D6}', 0x99); // LATIN CAPITAL LETTER O WITH DIAERESIS
    m.insert('\u{0000DC}', 0x9A); // LATIN CAPITAL LETTER U WITH DIAERESIS
    m.insert('\u{0000A2}', 0x9B); // CENT SIGN
    m.insert('\u{0000A3}', 0x9C); // POUND SIGN
    m.insert('\u{0000A5}', 0x9D); // YEN SIGN
    m.insert('\u{0020A7}', 0x9E); // PESETA SIGN
    m.insert('\u{000192}', 0x9F); // LATIN SMALL LETTER F WITH HOOK
    m.insert('\u{0000E1}', 0xA0); // LATIN SMALL LETTER A WITH ACUTE
    m.insert('\u{0000ED}', 0xA1); // LATIN SMALL LETTER I WITH ACUTE
    m.insert('\u{0000F3}', 0xA2); // LATIN SMALL LETTER O WITH ACUTE
    m.insert('\u{0000FA}', 0xA3); // LATIN SMALL LETTER U WITH ACUTE
    m.insert('\u{0000F1}', 0xA4); // LATIN SMALL LETTER N WITH TILDE
    m.insert('\u{0000D1}', 0xA5); // LATIN CAPITAL LETTER N WITH TILDE
    m.insert('\u{0000AA}', 0xA6); // FEMININE ORDINAL INDICATOR
    m.insert('\u{0000BA}', 0xA7); // MASCULINE ORDINAL INDICATOR
    m.insert('\u{0000BF}', 0xA8); // INVERTED QUESTION MARK
    m.insert('\u{002310}', 0xA9); // REVERSED NOT SIGN
    m.insert('\u{0000AC}', 0xAA); // NOT SIGN
    m.insert('\u{0000BD}', 0xAB); // VULGAR FRACTION ONE HALF
    m.insert('\u{0000BC}', 0xAC); // VULGAR FRACTION ONE QUARTER
    m.insert('\u{0000A1}', 0xAD); // INVERTED EXCLAMATION MARK
    m.insert('\u{0000AB}', 0xAE); // LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
    m.insert('\u{0000BB}', 0xAF); // RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
    m.insert('\u{002591}', 0xB0); // LIGHT SHADE
    m.insert('\u{002592}', 0xB1); // MEDIUM SHADE
    m.insert('\u{002593}', 0xB2); // DARK SHADE
    m.insert('\u{002502}', 0xB3); // BOX DRAWINGS LIGHT VERTICAL
    m.insert('\u{002524}', 0xB4); // BOX DRAWINGS LIGHT VERTICAL AND LEFT
    m.insert('\u{002561}', 0xB5); // BOX DRAWINGS VERTICAL SINGLE AND LEFT DOUBLE
    m.insert('\u{002562}', 0xB6); // BOX DRAWINGS VERTICAL DOUBLE AND LEFT SINGLE
    m.insert('\u{002556}', 0xB7); // BOX DRAWINGS DOWN DOUBLE AND LEFT SINGLE
    m.insert('\u{002555}', 0xB8); // BOX DRAWINGS DOWN SINGLE AND LEFT DOUBLE
    m.insert('\u{002563}', 0xB9); // BOX DRAWINGS DOUBLE VERTICAL AND LEFT
    m.insert('\u{002551}', 0xBA); // BOX DRAWINGS DOUBLE VERTICAL
    m.insert('\u{002557}', 0xBB); // BOX DRAWINGS DOUBLE DOWN AND LEFT
    m.insert('\u{00255D}', 0xBC); // BOX DRAWINGS DOUBLE UP AND LEFT
    m.insert('\u{00255C}', 0xBD); // BOX DRAWINGS UP DOUBLE AND LEFT SINGLE
    m.insert('\u{00255B}', 0xBE); // BOX DRAWINGS UP SINGLE AND LEFT DOUBLE
    m.insert('\u{002510}', 0xBF); // BOX DRAWINGS LIGHT DOWN AND LEFT
    m.insert('\u{002514}', 0xC0); // BOX DRAWINGS LIGHT UP AND RIGHT
    m.insert('\u{002534}', 0xC1); // BOX DRAWINGS LIGHT UP AND HORIZONTAL
    m.insert('\u{00252C}', 0xC2); // BOX DRAWINGS LIGHT DOWN AND HORIZONTAL
    m.insert('\u{00251C}', 0xC3); // BOX DRAWINGS LIGHT VERTICAL AND RIGHT
    m.insert('\u{002500}', 0xC4); // BOX DRAWINGS LIGHT HORIZONTAL
    m.insert('\u{00253C}', 0xC5); // BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL
    m.insert('\u{00255E}', 0xC6); // BOX DRAWINGS VERTICAL SINGLE AND RIGHT DOUBLE
    m.insert('\u{00255F}', 0xC7); // BOX DRAWINGS VERTICAL DOUBLE AND RIGHT SINGLE
    m.insert('\u{00255A}', 0xC8); // BOX DRAWINGS DOUBLE UP AND RIGHT
    m.insert('\u{002554}', 0xC9); // BOX DRAWINGS DOUBLE DOWN AND RIGHT
    m.insert('\u{002569}', 0xCA); // BOX DRAWINGS DOUBLE UP AND HORIZONTAL
    m.insert('\u{002566}', 0xCB); // BOX DRAWINGS DOUBLE DOWN AND HORIZONTAL
    m.insert('\u{002560}', 0xCC); // BOX DRAWINGS DOUBLE VERTICAL AND RIGHT
    m.insert('\u{002550}', 0xCD); // BOX DRAWINGS DOUBLE HORIZONTAL
    m.insert('\u{00256C}', 0xCE); // BOX DRAWINGS DOUBLE VERTICAL AND HORIZONTAL
    m.insert('\u{002567}', 0xCF); // BOX DRAWINGS UP SINGLE AND HORIZONTAL DOUBLE
    m.insert('\u{002568}', 0xD0); // BOX DRAWINGS UP DOUBLE AND HORIZONTAL SINGLE
    m.insert('\u{002564}', 0xD1); // BOX DRAWINGS DOWN SINGLE AND HORIZONTAL DOUBLE
    m.insert('\u{002565}', 0xD2); // BOX DRAWINGS DOWN DOUBLE AND HORIZONTAL SINGLE
    m.insert('\u{002559}', 0xD3); // BOX DRAWINGS UP DOUBLE AND RIGHT SINGLE
    m.insert('\u{002558}', 0xD4); // BOX DRAWINGS UP SINGLE AND RIGHT DOUBLE
    m.insert('\u{002552}', 0xD5); // BOX DRAWINGS DOWN SINGLE AND RIGHT DOUBLE
    m.insert('\u{002553}', 0xD6); // BOX DRAWINGS DOWN DOUBLE AND RIGHT SINGLE
    m.insert('\u{00256B}', 0xD7); // BOX DRAWINGS VERTICAL DOUBLE AND HORIZONTAL SINGLE
    m.insert('\u{00256A}', 0xD8); // BOX DRAWINGS VERTICAL SINGLE AND HORIZONTAL DOUBLE
    m.insert('\u{002518}', 0xD9); // BOX DRAWINGS LIGHT UP AND LEFT
    m.insert('\u{00250C}', 0xDA); // BOX DRAWINGS LIGHT DOWN AND RIGHT
    m.insert('\u{002588}', 0xDB); // FULL BLOCK
    m.insert('\u{002584}', 0xDC); // LOWER HALF BLOCK
    m.insert('\u{00258C}', 0xDD); // LEFT HALF BLOCK
    m.insert('\u{002590}', 0xDE); // RIGHT HALF BLOCK
    m.insert('\u{002580}', 0xDF); // UPPER HALF BLOCK
    m.insert('\u{0003B1}', 0xE0); // GREEK SMALL LETTER ALPHA
    m.insert('\u{0000DF}', 0xE1); // LATIN SMALL LETTER SHARP S
    m.insert('\u{000393}', 0xE2); // GREEK CAPITAL LETTER GAMMA
    m.insert('\u{0003C0}', 0xE3); // GREEK SMALL LETTER PI
    m.insert('\u{0003A3}', 0xE4); // GREEK CAPITAL LETTER SIGMA
    m.insert('\u{0003C3}', 0xE5); // GREEK SMALL LETTER SIGMA
    m.insert('\u{0000B5}', 0xE6); // MICRO SIGN
    m.insert('\u{0003C4}', 0xE7); // GREEK SMALL LETTER TAU
    m.insert('\u{0003A6}', 0xE8); // GREEK CAPITAL LETTER PHI
    m.insert('\u{000398}', 0xE9); // GREEK CAPITAL LETTER THETA
    m.insert('\u{0003A9}', 0xEA); // GREEK CAPITAL LETTER OMEGA
    m.insert('\u{0003B4}', 0xEB); // GREEK SMALL LETTER DELTA
    m.insert('\u{00221E}', 0xEC); // INFINITY
    m.insert('\u{0003C6}', 0xED); // GREEK SMALL LETTER PHI
    m.insert('\u{0003B5}', 0xEE); // GREEK SMALL LETTER EPSILON
    m.insert('\u{002229}', 0xEF); // INTERSECTION
    m.insert('\u{002261}', 0xF0); // IDENTICAL TO
    m.insert('\u{0000B1}', 0xF1); // PLUS-MINUS SIGN
    m.insert('\u{002265}', 0xF2); // GREATER-THAN OR EQUAL TO
    m.insert('\u{002264}', 0xF3); // LESS-THAN OR EQUAL TO
    m.insert('\u{002320}', 0xF4); // TOP HALF INTEGRAL
    m.insert('\u{002321}', 0xF5); // BOTTOM HALF INTEGRAL
    m.insert('\u{0000F7}', 0xF6); // DIVISION SIGN
    m.insert('\u{002248}', 0xF7); // ALMOST EQUAL TO
    m.insert('\u{0000B0}', 0xF8); // DEGREE SIGN
    m.insert('\u{002219}', 0xF9); // BULLET OPERATOR
    m.insert('\u{0000B7}', 0xFA); // MIDDLE DOT
    m.insert('\u{00221A}', 0xFB); // SQUARE ROOT
    m.insert('\u{00207F}', 0xFC); // SUPERSCRIPT LATIN SMALL LETTER N
    m.insert('\u{0000B2}', 0xFD); // SUPERSCRIPT TWO
    m.insert('\u{0025A0}', 0xFE); // BLACK SQUARE
    m.insert('\u{0000A0}', 0xFF); // NO-BREAK SPACE
    m
});

pub fn unicode_to_cp437<S: AsRef<str>>(input: S) -> Vec<u8> {
    let input = input.as_ref();
    input
        .chars()
        .map(|c| {
            if let Some(&byte) = CP437.get(&c) {
                byte
            } else if c.is_ascii() {
                c as u8
            } else {
                // If the character is not in the CP437 mapping and not an ascii byte,
                // use a placeholder (e.g., '?')
                b'?'
            }
        })
        .collect()
}
