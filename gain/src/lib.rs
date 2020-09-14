#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(min_specialization)]

use serde::{Serialize, Deserialize};

use baseplug::{
    ProcessContext, Plugin, WindowOpenResult,
};
use raw_window_handle::RawWindowHandle;

mod ui;

baseplug::model! {
    #[derive(Debug, Serialize, Deserialize)]
    struct GainModel {
        #[model(min = -90.0, max = 3.0)]
        #[parameter(name = "gain left", unit = "Decibels",
            gradient = "Power(0.15)")]
        gain_left: f32,

        #[model(min = -90.0, max = 3.0)]
        #[parameter(name = "gain right", unit = "Decibels",
            gradient = "Power(0.15)")]
        gain_right: f32,

        #[model(min = -90.0, max = 3.0)]
        #[parameter(name = "gain master", unit = "Decibels",
            gradient = "Power(0.15)")]
        gain_master: f32,
    }
}

impl Default for GainModel {
    fn default() -> Self {
        Self {
            // "gain" is converted from dB to coefficient in the parameter handling code,
            // so in the model here it's a coeff.
            // -0dB == 1.0
            gain_left: 1.0,
            gain_right: 1.0,
            gain_master: 1.0,
        }
    }
}

struct Gain { }

impl Plugin for Gain {
    const NAME: &'static str = "iced-baseplug gain";
    const PRODUCT: &'static str = "iced-baseplug gain";
    const VENDOR: &'static str = "spicy plugins & co";

    const INPUT_CHANNELS: usize = 2;
    const OUTPUT_CHANNELS: usize = 2;

    type Model = GainModel;

    #[inline]
    fn new(_sample_rate: f32, _model: &GainModel) -> Self {
        Self {}
    }

    #[inline]
    fn process(&mut self, model: &GainModelProcess, ctx: &mut ProcessContext) {
        let input = &ctx.inputs[0].buffers;
        let output = &mut ctx.outputs[0].buffers;

        for i in 0..ctx.nframes {
            output[0][i] = input[0][i] * model.gain_left[i] * model.gain_master[i];
            output[1][i] = input[1][i] * model.gain_right[i] * model.gain_master[i];
        }
    }
}

impl baseplug::PluginUI for Gain {
    fn ui_size(&self) -> (i16, i16) {
        (230, 130)
    }

    fn ui_open(&mut self, parent: RawWindowHandle) -> WindowOpenResult {
        let (width, height) = self.ui_size();

        let settings = iced_baseview::Settings {
            window: iced_baseview::settings::Window {
                title: String::from("iced-baseplug gain"),
                size: (width as u32, height as u32),
                min_size: None,
                max_size: None,
                resizable: false,
            },
        };

        let _handle = iced_baseview::Handler::<ui::GainUI>::open(settings, Some(parent));

        Ok(())
    }

    fn ui_close(&mut self) {
        
    }
}

baseplug::vst2!(Gain, b"tAnE");