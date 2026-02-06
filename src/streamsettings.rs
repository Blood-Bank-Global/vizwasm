use std::{collections::HashMap, hash::Hash, sync::LazyLock};

use sdlrig::{
    renderspec::{RenderSpec, SendCmd, SendValue},
    seek,
};
use serde::{Deserialize, Serialize};

use crate::vizconfig::AllSettings;
use std::f64::consts::PI;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[repr(C)]
pub struct StreamSettingsFieldProperties {
    pub channel: Option<u8>,
    pub cc: Option<u8>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
    pub default: Option<f64>,
    pub label: Option<String>,
    pub do_not_record: bool,
    pub tween: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum StreamSettingsField {
    FlashEnable,
    Tween,
    UsrVar,
    MixRr,
    MixRg,
    MixRb,
    MixRa,
    MixGr,
    MixGb,
    MixGg,
    MixGa,
    MixBr,
    MixBg,
    MixBb,
    MixBa,
    MixAr,
    MixAg,
    MixAb,
    MixAa,
    Boost,
    Threshold,
    DistortLevel,
    WarpLevel,
    SkewX0,
    SkewY0,
    SkewX1,
    SkewY1,
    SkewX2,
    SkewY2,
    SkewX3,
    SkewY3,
    ColorKeySim,
    VideoKeyBlend,
    ColorKeyEnable,
    ScrollH,
    ScrollV,
    ScrolledH,
    ScrolledV,
    ShiftRh,
    ShiftRv,
    ShiftGh,
    ShiftGv,
    ShiftBv,
    ShiftBh,
    ShiftAh,
    ShiftAv,
    Negate,
    FeedbackDx,
    FeedbackDy,
    FeedbackRotation,
    DistortScan,
    DistortSelected,
    DistEdgeScan,
    DistEdgeSelected,
    WarpScan,
    WarpSelected,
    LutScan,
    LutSelected,
    OverlayBlendScan,
    OverlayBlendSelected,
    OverlayScan,
    OverlaySelected,
    ScanlinesScan,
    ScanlinesSelected,
    DeltaSec,
    Scrub,
    ExactSec,
    Pause,
    FeedbackModeScan,
    FeedbackModeSelected,
}

pub const ALL_FIELDS: &[StreamSettingsField] = &[
    StreamSettingsField::FlashEnable,
    StreamSettingsField::Tween,
    StreamSettingsField::UsrVar,
    StreamSettingsField::MixRr,
    StreamSettingsField::MixRg,
    StreamSettingsField::MixRb,
    StreamSettingsField::MixRa,
    StreamSettingsField::MixGr,
    StreamSettingsField::MixGb,
    StreamSettingsField::MixGg,
    StreamSettingsField::MixGa,
    StreamSettingsField::MixBr,
    StreamSettingsField::MixBg,
    StreamSettingsField::MixBb,
    StreamSettingsField::MixBa,
    StreamSettingsField::MixAr,
    StreamSettingsField::MixAg,
    StreamSettingsField::MixAb,
    StreamSettingsField::MixAa,
    StreamSettingsField::Boost,
    StreamSettingsField::Threshold,
    StreamSettingsField::DistortLevel,
    StreamSettingsField::WarpLevel,
    StreamSettingsField::SkewX0,
    StreamSettingsField::SkewY0,
    StreamSettingsField::SkewX1,
    StreamSettingsField::SkewY1,
    StreamSettingsField::SkewX2,
    StreamSettingsField::SkewY2,
    StreamSettingsField::SkewX3,
    StreamSettingsField::SkewY3,
    StreamSettingsField::ColorKeySim,
    StreamSettingsField::VideoKeyBlend,
    StreamSettingsField::ColorKeyEnable,
    StreamSettingsField::ScrollH,
    StreamSettingsField::ScrollV,
    StreamSettingsField::ScrolledH,
    StreamSettingsField::ScrolledV,
    StreamSettingsField::ShiftRh,
    StreamSettingsField::ShiftRv,
    StreamSettingsField::ShiftGh,
    StreamSettingsField::ShiftGv,
    StreamSettingsField::ShiftBv,
    StreamSettingsField::ShiftBh,
    StreamSettingsField::ShiftAh,
    StreamSettingsField::ShiftAv,
    StreamSettingsField::Negate,
    StreamSettingsField::FeedbackDx,
    StreamSettingsField::FeedbackDy,
    StreamSettingsField::FeedbackRotation,
    StreamSettingsField::DistortScan,
    StreamSettingsField::DistortSelected,
    StreamSettingsField::DistEdgeScan,
    StreamSettingsField::DistEdgeSelected,
    StreamSettingsField::WarpScan,
    StreamSettingsField::WarpSelected,
    StreamSettingsField::LutScan,
    StreamSettingsField::LutSelected,
    StreamSettingsField::OverlayBlendScan,
    StreamSettingsField::OverlayBlendSelected,
    StreamSettingsField::OverlayScan,
    StreamSettingsField::OverlaySelected,
    StreamSettingsField::ScanlinesScan,
    StreamSettingsField::ScanlinesSelected,
    StreamSettingsField::DeltaSec,
    StreamSettingsField::Scrub,
    StreamSettingsField::ExactSec,
    StreamSettingsField::Pause,
    StreamSettingsField::FeedbackModeScan,
    StreamSettingsField::FeedbackModeSelected,
];

static PROPERTIES: LazyLock<HashMap<StreamSettingsField, StreamSettingsFieldProperties>> =
    LazyLock::new(|| {
        let mut m = HashMap::new();
        macro_rules! mk {
            ($p:ident) => {
                m.insert(
                    StreamSettingsField::$p,
                    StreamSettingsFieldProperties {
                        ..Default::default()
                    },
                );
            };
            ($p:ident,  $channel:expr, $cc:expr, $min:expr, $max:expr, $step:expr, $default:expr, $label:expr) => {
                m.insert(
                    StreamSettingsField::$p,
                    StreamSettingsFieldProperties {
                        channel: Some($channel),
                        cc: Some($cc),
                        min: Some($min),
                        max: Some($max),
                        step: Some($step),
                        default: Some($default),
                        label: Some($label.to_string()),
                        do_not_record: false,
                        tween: true,
                    },
                );
            };
            ($p:ident,  $channel:expr, $cc:expr, $min:expr, $max:expr, $step:expr, $label:expr, $tween:expr) => {
                m.insert(
                    StreamSettingsField::$p,
                    StreamSettingsFieldProperties {
                        channel: Some($channel),
                        cc: Some($cc),
                        min: Some($min),
                        max: Some($max),
                        step: Some($step),
                        default: Some($default),
                        label: Some($label.to_string()),
                        do_not_record: false,
                        tween: $tween,
                    },
                );
            };
            ($p:ident, $channel:expr, $cc:expr, $min:expr, $max:expr, $step:expr, $label:expr, $do_not_record:expr, $tween:expr) => {
                m.insert(
                    StreamSettingsField::$p,
                    StreamSettingsFieldProperties {
                        channel: Some($channel),
                        cc: Some($cc),
                        min: Some($min),
                        max: Some($max),
                        step: Some($step),
                        default: Some($default),
                        label: Some($label.to_string()),
                        do_not_record: $do_not_record,
                        tween: $tween,
                    },
                );
            };
        }
        macro_rules! internal {
            ($p:ident, $default:expr) => {
                m.insert(
                    StreamSettingsField::$p,
                    StreamSettingsFieldProperties {
                        default: Some($default),
                        do_not_record: true,
                        ..Default::default()
                    },
                );
            };
            ($p:ident, $default:expr, $label:expr) => {
                m.insert(
                    StreamSettingsField::$p,
                    StreamSettingsFieldProperties {
                        default: Some($default),
                        label: Some($label.to_string()),
                        do_not_record: true,
                        ..Default::default()
                    },
                );
            };
            ($p:ident, $default:expr, $label:expr, $dnr:expr) => {
                m.insert(
                    StreamSettingsField::$p,
                    StreamSettingsFieldProperties {
                        default: Some($default),
                        label: Some($label.to_string()),
                        do_not_record: $dnr,
                        ..Default::default()
                    },
                );
            };
        }
        // row 1
        mk!(Threshold, 0, 0, 0.0, 1.0, 0.001, 0.0, "thresh");
        mk!(DistortLevel, 0, 1, 0.0, 1.0, 0.001, 0.05, "distort_level");
        mk!(WarpLevel, 0, 2, 0.0, 1.0, 0.001, 0.05, "warp_level");
        mk!(ColorKeySim, 0, 3, 0.001, 1.0, 0.001, 0.001, "color_key_sim");
        mk!(ColorKeyEnable, 1, 3, 0.0, 1.0, 1.0, 0.0, "color_key_enable");
        //row 2
        mk!(
            DistEdgeScan,
            0,
            4,
            0.0,
            AllSettings::distort_edge_types().len() as f64,
            0.2,
            0.0,
            "distort_edge"
        );
        mk!(
            DistEdgeSelected,
            1,
            4,
            0.0,
            AllSettings::distort_edge_types().len() as f64,
            0.2,
            0.0,
            "distort_edge"
        );
        mk!(
            DistortScan,
            0,
            5,
            0.0,
            AllSettings::distort_names().len() as f64,
            0.2,
            0.0,
            "distort"
        );
        mk!(
            DistortSelected,
            1,
            5,
            0.0,
            AllSettings::distort_names().len() as f64,
            0.2,
            0.0,
            "distort"
        );
        mk!(
            WarpScan,
            0,
            6,
            0.0,
            AllSettings::distort_names().len() as f64,
            0.2,
            0.0,
            "warp"
        );
        mk!(
            WarpSelected,
            1,
            6,
            0.0,
            AllSettings::distort_names().len() as f64,
            0.2,
            0.0,
            "warp"
        );
        mk!(
            LutScan,
            0,
            7,
            0.0,
            AllSettings::lut_names().len() as f64,
            0.2,
            0.0,
            "lut"
        );
        mk!(
            LutSelected,
            1,
            7,
            0.0,
            AllSettings::lut_names().len() as f64,
            0.2,
            0.0,
            "lut"
        );
        // row 3
        mk!(ScrollH, 0, 8, -1.0, 1.0, 0.0001, 0.0, "scroll_h");
        mk!(ScrollV, 0, 9, -1.0, 1.0, 0.0001, 0.0, "scroll_v");
        internal!(ScrolledH, 0.0, "scrolled_h");
        internal!(ScrolledV, 0.0, "scrolled_v");
        mk!(FeedbackDx, 0, 10, -1.0, 1.0, 0.001, 0.0, "distort_dx");
        mk!(FeedbackDy, 0, 11, -1.0, 1.0, 0.001, 0.0, "distort_dy");
        // row 4
        mk!(
            FeedbackRotation,
            0,
            12,
            -2.0 * PI,
            2.0 * PI,
            2.0 * PI / 400.0,
            0.0,
            "feedback_rotation"
        );
        mk!(
            ScanlinesScan,
            0,
            13,
            0.0,
            AllSettings::blend_modes().len() as f64,
            0.2,
            0.0,
            "scanline_scan"
        );
        mk!(
            ScanlinesSelected,
            1,
            13,
            0.0,
            AllSettings::blend_modes().len() as f64,
            0.2,
            0.0,
            "scanline_kind"
        );
        mk!(
            OverlayBlendScan,
            0,
            14,
            0.0,
            AllSettings::blend_modes().len() as f64,
            0.2,
            0.0,
            "overlay_blend_scan"
        );
        mk!(
            OverlayBlendSelected,
            1,
            14,
            0.0,
            AllSettings::blend_modes().len() as f64,
            0.2,
            0.0,
            "overlay_kind"
        );
        mk!(
            OverlayScan,
            0,
            15,
            0.0,
            AllSettings::overlay_vids().len() as f64,
            0.2,
            0.0,
            "overlay_vid"
        );
        mk!(
            OverlaySelected,
            1,
            15,
            0.0,
            AllSettings::overlay_vids().len() as f64,
            0.2,
            0.0,
            "overlay_vid"
        );

        mk!(MixRr, 0, 16, -2.0, 2.0, 0.01, 1.0, "mix_rr");
        mk!(MixRb, 0, 17, -2.0, 2.0, 0.01, 0.0, "mix_rb");
        mk!(MixRg, 0, 18, -2.0, 2.0, 0.01, 0.0, "mix_rg");
        mk!(MixRa, 0, 19, -2.0, 2.0, 0.01, 0.0, "mix_ra");
        mk!(MixGr, 0, 20, -2.0, 2.0, 0.01, 0.0, "mix_gr");
        mk!(MixGg, 0, 21, -2.0, 2.0, 0.01, 1.0, "mix_gg");
        mk!(MixGb, 0, 22, -2.0, 2.0, 0.01, 0.0, "mix_gb");
        mk!(MixGa, 0, 23, -2.0, 2.0, 0.01, 0.0, "mix_ga");
        mk!(MixBr, 0, 24, -2.0, 2.0, 0.01, 0.0, "mix_br");
        mk!(MixBg, 0, 25, -2.0, 2.0, 0.01, 0.0, "mix_bg");
        mk!(MixBb, 0, 26, -2.0, 2.0, 0.01, 1.0, "mix_bb");
        mk!(MixBa, 0, 27, -2.0, 2.0, 0.01, 0.0, "mix_ba");
        mk!(MixAr, 0, 28, -2.0, 2.0, 0.01, 0.0, "mix_ar");
        mk!(MixAg, 0, 29, -2.0, 2.0, 0.01, 0.0, "mix_ag");
        mk!(MixAb, 0, 30, -2.0, 2.0, 0.01, 0.0, "mix_ab");
        mk!(MixAa, 0, 31, -2.0, 2.0, 0.01, 1.0, "mix_aa");
        /*
           rh
           rv
           skewx0
           skewy0
           ...
        */
        mk!(ShiftRh, 0, 32, -1.0, 1.0, 0.001, 0.0, "shift_rh");
        mk!(ShiftRv, 0, 33, -1.0, 1.0, 0.001, 0.0, "shift_rv");
        mk!(SkewX0, 0, 34, -5.0, 5.0, 0.001, 0.0, "skew_x0");
        mk!(SkewY0, 0, 35, -5.0, 5.0, 0.001, 0.0, "skew_y0");
        mk!(ShiftGh, 0, 36, -1.0, 1.0, 0.001, 0.0, "shift_gh");
        mk!(ShiftGv, 0, 37, -1.0, 1.0, 0.001, 0.0, "shift_gv");
        mk!(SkewX1, 0, 38, -5.0, 5.0, 0.001, 1.0, "skew_x1");
        mk!(SkewY1, 0, 39, -5.0, 5.0, 0.001, 0.0, "skew_y1");
        mk!(ShiftBh, 0, 40, -1.0, 1.0, 0.001, 0.0, "shift_bh");
        mk!(ShiftBv, 0, 41, -1.0, 1.0, 0.001, 0.0, "shift_bv");
        mk!(SkewX2, 0, 42, -5.0, 5.0, 0.001, 0.0, "skew_x2");
        mk!(SkewY2, 0, 43, -5.0, 5.0, 0.001, 1.0, "skew_y2");
        mk!(ShiftAh, 0, 44, -1.0, 1.0, 0.001, 0.0, "shift_ah");
        mk!(ShiftAv, 0, 45, -1.0, 1.0, 0.001, 0.0, "shift_av");
        mk!(SkewX3, 0, 46, -5.0, 5.0, 0.001, 1.0, "skew_x3");
        mk!(SkewY3, 0, 47, -5.0, 5.0, 0.001, 1.0, "skew_y3");

        /*
        feedback mode
         */
        mk!(FeedbackModeScan, 0, 48, 0.0, 3.0, 0.2, 0.0, "feedback_mode");
        mk!(
            FeedbackModeSelected,
            1,
            48,
            0.0,
            3.0,
            0.2,
            0.0,
            "feedback_mode_selected"
        );

        internal!(FlashEnable, 0.0, "flash_enable", false);
        internal!(Tween, 0.0);
        internal!(UsrVar, 0.0, "usr_var", false);
        internal!(DeltaSec, 0.0);
        internal!(Scrub, 0.0);
        internal!(ExactSec, 0.0, "exact_sec", false);
        internal!(Pause, 0.0, "pause", false);
        m
    });

static FIELDS_BY_CC: LazyLock<HashMap<(u8, u8), StreamSettingsField>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    for (field, props) in PROPERTIES.iter() {
        if let (Some(channel), Some(cc)) = (props.channel, props.cc) {
            m.insert((channel, cc), field.clone());
        }
    }
    m
});

impl StreamSettingsField {
    pub fn properties(&self) -> Option<StreamSettingsFieldProperties> {
        PROPERTIES.get(self).cloned()
    }

    pub fn find(channel: u8, cc: u8) -> Option<StreamSettingsField> {
        FIELDS_BY_CC.get(&(channel, cc)).cloned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[repr(C)]
pub struct StreamIdent {
    pub name: String,
    pub first_video: String,
    pub input_mix: String,
    pub main_mix: String,
    pub feedback_mix: String,
    pub overlay_mix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[repr(C)]
pub struct StreamSettings {
    pub ident: StreamIdent,
    pub real_ts: (i32, i32),
    pub continuous_ts: (i32, i32),
    pub mark: Option<f64>,
    pub fields: HashMap<StreamSettingsField, f64>,
}

impl StreamSettings {
    pub fn new(ident: StreamIdent) -> Self {
        let mut fields = HashMap::new();
        for field in ALL_FIELDS.iter() {
            let props = field.properties().unwrap_or(StreamSettingsFieldProperties {
                do_not_record: true,
                ..Default::default()
            });
            if let Some(default) = props.default {
                fields.insert(field.clone(), default);
            } else {
                fields.insert(field.clone(), 0.0);
            }
        }
        StreamSettings {
            ident,
            real_ts: (0, 1),
            continuous_ts: (0, 1),
            mark: None,
            fields,
        }
    }

    pub fn set_field(&mut self, field: StreamSettingsField, value: f64) {
        self.fields.insert(field, value);
    }

    pub fn get_field(&self, field: &StreamSettingsField) -> f64 {
        self.fields.get(field).cloned().unwrap_or_default()
    }

    pub fn adjust_field(&mut self, field: &StreamSettingsField, steps: f64) {
        let current = self.get_field(field);
        let step = field.properties().unwrap_or_default().step.unwrap_or(0.0);
        let delta = step * steps;
        match field {
            StreamSettingsField::OverlayBlendSelected => self.set_field(
                StreamSettingsField::OverlayBlendSelected,
                self.get_field(&StreamSettingsField::OverlayBlendScan),
            ),
            StreamSettingsField::OverlaySelected => self.set_field(
                StreamSettingsField::OverlaySelected,
                self.get_field(&StreamSettingsField::OverlayScan),
            ),
            StreamSettingsField::DistortSelected => self.set_field(
                StreamSettingsField::DistortSelected,
                self.get_field(&StreamSettingsField::DistortScan),
            ),
            StreamSettingsField::DistEdgeSelected => self.set_field(
                StreamSettingsField::DistEdgeSelected,
                self.get_field(&StreamSettingsField::DistEdgeScan),
            ),
            StreamSettingsField::WarpSelected => self.set_field(
                StreamSettingsField::WarpSelected,
                self.get_field(&StreamSettingsField::WarpScan),
            ),
            StreamSettingsField::LutSelected => self.set_field(
                StreamSettingsField::LutSelected,
                self.get_field(&StreamSettingsField::LutScan),
            ),
            StreamSettingsField::ScanlinesSelected => self.set_field(
                StreamSettingsField::ScanlinesSelected,
                self.get_field(&StreamSettingsField::ScanlinesScan),
            ),
            StreamSettingsField::FeedbackModeSelected => self.set_field(
                StreamSettingsField::FeedbackModeSelected,
                self.get_field(&StreamSettingsField::FeedbackModeScan),
            ),
            StreamSettingsField::FeedbackRotation => {
                let mut new_value = current + delta;
                while new_value < -2.0 * PI {
                    new_value += 4.0 * PI;
                }
                while new_value > 2.0 * PI {
                    new_value -= 4.0 * PI;
                }
                self.set_field(field.clone(), new_value);
            }
            StreamSettingsField::FlashEnable
            | StreamSettingsField::Tween
            | StreamSettingsField::ColorKeyEnable
            | StreamSettingsField::Negate
            | StreamSettingsField::Pause => {
                if current != 0.0 {
                    self.set_field(field.clone(), 0.0);
                } else {
                    self.set_field(field.clone(), 1.0);
                }
            }
            _ => {
                let props = field.properties().unwrap_or_default();
                if let Some(min) = props.min {
                    if current + delta < min {
                        self.set_field(field.clone(), min);
                        return;
                    }
                }
                if let Some(max) = props.max {
                    if current + delta > max {
                        self.set_field(field.clone(), max);
                        return;
                    }
                }
                self.set_field(field.clone(), current + delta);
            }
        }
    }

    pub fn command(&self, field: &StreamSettingsField) -> Vec<RenderSpec> {
        match field {
            // special case for skew
            StreamSettingsField::SkewX0
            | StreamSettingsField::SkewY0
            | StreamSettingsField::SkewX1
            | StreamSettingsField::SkewY1
            | StreamSettingsField::SkewX2
            | StreamSettingsField::SkewY2
            | StreamSettingsField::SkewX3
            | StreamSettingsField::SkewY3 => {
                vec![SendCmd::builder()
                    .mix(self.main_mix())
                    .name("new_corners")
                    .value(SendValue::Vector(vec![
                        self.get_field(&StreamSettingsField::SkewX0) as f32,
                        self.get_field(&StreamSettingsField::SkewY0) as f32,
                        self.get_field(&StreamSettingsField::SkewX1) as f32,
                        self.get_field(&StreamSettingsField::SkewY1) as f32,
                        self.get_field(&StreamSettingsField::SkewX2) as f32,
                        self.get_field(&StreamSettingsField::SkewY2) as f32,
                        self.get_field(&StreamSettingsField::SkewX3) as f32,
                        self.get_field(&StreamSettingsField::SkewY3) as f32,
                    ]))
                    .build()
                    .into()]
            }
            //Time seek commands
            StreamSettingsField::DeltaSec => {
                vec![seek!(self.first_video() => self.get_field(field), false)]
            }
            StreamSettingsField::Scrub => {
                let value = self.get_field(field);
                if value >= 0.0 {
                    vec![seek!(self.first_video() => value, false)]
                } else {
                    vec![seek!(self.first_video() => value - 0.1, false)]
                }
            }
            StreamSettingsField::ExactSec => {
                vec![seek!(self.first_video() => self.get_field(field), true)]
            }
            StreamSettingsField::UsrVar => {
                vec![SendCmd::builder()
                    .mix(self.input_mix())
                    .name("usr_var")
                    .value(SendValue::Integer(self.get_field(field) as i32))
                    .build()
                    .into()]
            }
            //Send unsigned for these fields
            StreamSettingsField::ColorKeyEnable
            | StreamSettingsField::Negate
            | StreamSettingsField::DistEdgeSelected
            | StreamSettingsField::FeedbackModeSelected => {
                if let Some(props) = PROPERTIES.get(field) {
                    if let Some(name) = &props.label {
                        let value = self.get_field(field) as u32;
                        vec![SendCmd::builder()
                            .mix(self.main_mix())
                            .name(name)
                            .value(SendValue::Unsigned(value))
                            .build()
                            .into()]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
            StreamSettingsField::OverlayBlendSelected | StreamSettingsField::ScanlinesSelected => {
                if let Some(props) = PROPERTIES.get(field) {
                    if let Some(name) = &props.label {
                        let value = self.get_field(field) as u32;
                        vec![SendCmd::builder()
                            .mix(self.overlay_mix())
                            .name(name)
                            .value(SendValue::Unsigned(value))
                            .build()
                            .into()]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
            // send nothing for these fields
            StreamSettingsField::DistEdgeScan
            | StreamSettingsField::DistortScan
            | StreamSettingsField::WarpScan
            | StreamSettingsField::LutScan
            | StreamSettingsField::OverlayBlendScan
            | StreamSettingsField::OverlayScan
            | StreamSettingsField::ScanlinesScan
            | StreamSettingsField::FeedbackModeScan => vec![],
            // Fall down to send float
            _ => {
                if let Some(props) = PROPERTIES.get(field) {
                    if let Some(name) = &props.label {
                        let value = self.get_field(field);
                        vec![SendCmd::builder()
                            .mix(self.main_mix())
                            .name(name)
                            .value(SendValue::Float(value as f32))
                            .build()
                            .into()]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
        }
    }

    pub fn first_video(&self) -> String {
        self.ident.first_video.clone()
    }
    pub fn input_mix(&self) -> String {
        self.ident.input_mix.clone()
    }
    pub fn main_mix(&self) -> String {
        self.ident.main_mix.clone()
    }
    pub fn feedback_mix(&self) -> String {
        self.ident.feedback_mix.clone()
    }
    pub fn overlay_mix(&self) -> String {
        self.ident.overlay_mix.clone()
    }
    pub fn reset(&mut self) {
        *self = Self::new(StreamIdent {
            name: self.ident.name.clone(),
            first_video: self.first_video(),
            input_mix: self.input_mix(),
            main_mix: self.main_mix(),
            feedback_mix: self.feedback_mix(),
            overlay_mix: self.overlay_mix(),
        });
    }

    pub fn diff(&self, other: &StreamSettings) -> Vec<(StreamSettingsField, f64)> {
        let mut diffs = HashMap::new();
        for field in ALL_FIELDS.iter() {
            let v1 = self.get_field(field);
            let v2 = other.get_field(field);
            if (v1 - v2).abs() > std::f64::EPSILON {
                diffs.insert(field.clone(), v2);
            }
        }
        diffs.into_iter().collect()
    }

    pub fn apply_diff(&mut self, diffs: &[(StreamSettingsField, f64)]) {
        for (field, value) in diffs.iter() {
            self.set_field(field.clone(), *value);
        }
    }

    pub fn get_commands(&self, fields: &[StreamSettingsField]) -> Vec<RenderSpec> {
        let mut cmds = vec![];
        for field in fields.iter() {
            let mut field_cmds = self.command(field);
            cmds.append(&mut field_cmds);
        }
        cmds
    }

    pub fn tween_diff(
        &self,
        field: StreamSettingsField,
        start: f64,
        end: f64,
        p: f64,
    ) -> Option<f64> {
        let props = field.properties().unwrap_or_default();
        if !props.tween {
            return None;
        }
        Some(start + (end - start) * p)
    }

    pub fn find_field(channel: u8, cc: u8) -> Option<StreamSettingsField> {
        FIELDS_BY_CC.get(&(channel, cc)).cloned()
    }
}
