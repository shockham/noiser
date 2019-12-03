use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, OscillatorType, BiquadFilterType};

/// Converts a midi note to frequency
///
/// A midi note is an integer, generally in the range of 21 to 108
pub fn midi_to_freq(note: u8) -> f32 {
    27.5 * 2f32.powf((note as f32 - 21.0) / 12.0)
}

#[wasm_bindgen]
pub struct FmOsc {
    ctx: AudioContext,
    primary: web_sys::OscillatorNode,
    gain: web_sys::GainNode,
    filter: web_sys::BiquadFilterNode,
    dist: web_sys::WaveShaperNode,
    fm_gain: web_sys::GainNode,
    fm_osc: web_sys::OscillatorNode,
    fm_freq_ratio: f32,
    fm_gain_ratio: f32,
}

impl Drop for FmOsc {
    fn drop(&mut self) {
        let _ = self.ctx.close();
    }
}

#[wasm_bindgen]
impl FmOsc {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<FmOsc, JsValue> {
        let ctx = web_sys::AudioContext::new()?;

        // Create our web audio objects.
        let primary = ctx.create_oscillator()?;
        let fm_osc = ctx.create_oscillator()?;
        let gain = ctx.create_gain()?;
        let fm_gain = ctx.create_gain()?;
        let filter = ctx.create_biquad_filter()?;
        let dist = ctx.create_wave_shaper()?;
        let comp = ctx.create_dynamics_compressor()?;

        // Some initial settings:
        primary.set_type(OscillatorType::Sine);
        primary.frequency().set_value(440.0); // A4 note
        gain.gain().set_value(0.0); // starts muted
        fm_gain.gain().set_value(0.0); // no initial frequency modulation
        fm_osc.set_type(OscillatorType::Sine);
        fm_osc.frequency().set_value(0.0);
        filter.set_type(BiquadFilterType::Lowpass);

        comp.threshold().set_value(-50f32);
        comp.ratio().set_value(5f32);

        // Connect the nodes up!
        primary.connect_with_audio_node(&filter)?;
        filter.connect_with_audio_node(&dist)?;
        dist.connect_with_audio_node(&gain)?;
        gain.connect_with_audio_node(&comp)?;
        comp.connect_with_audio_node(&ctx.destination())?;

        fm_osc.connect_with_audio_node(&fm_gain)?;
        fm_gain.connect_with_audio_param(&primary.frequency())?;

        // Start the oscillators!
        primary.start()?;
        fm_osc.start()?;

        Ok(FmOsc {
            ctx,
            primary,
            gain,
            filter,
            dist,
            fm_gain,
            fm_osc,
            fm_freq_ratio: 0.0,
            fm_gain_ratio: 0.0,
        })
    }

    #[wasm_bindgen]
    pub fn set_primary(&self, state: bool) -> Result<(), JsValue>  {
        if state {
            self.primary.start()?;
        } else {
            self.primary.stop()?;
        }

        Ok(())
    }

    /// Sets the gain for this oscillator, between 0.0 and 1.0.
    #[wasm_bindgen]
    pub fn set_gain(&self, mut gain: f32) {
        if gain > 1.0 {
            gain = 1.0;
        }
        if gain < 0.0 {
            gain = 0.0;
        }
        self.gain.gain().set_value(gain);
    }

    #[wasm_bindgen]
    pub fn set_primary_frequency(&self, freq: f32) {
        self.primary.frequency().set_value(freq);

        // The frequency of the FM oscillator depends on the frequency of the
        // primary oscillator, so we update the frequency of both in this method.
        self.fm_osc.frequency().set_value(self.fm_freq_ratio * freq);
        self.fm_gain.gain().set_value(self.fm_gain_ratio * freq);
    }

    #[wasm_bindgen]
    pub fn set_note(&self, note: u8) {
        let freq = midi_to_freq(note);
        self.set_primary_frequency(freq);
    }

    /// This should be between 0 and 1, though higher values are accepted.
    #[wasm_bindgen]
    pub fn set_fm_amount(&mut self, amt: f32) {
        self.fm_gain_ratio = amt;

        self.fm_gain
            .gain()
            .set_value(self.fm_gain_ratio * self.primary.frequency().value());
    }

    /// This should be between 0 and 1, though higher values are accepted.
    #[wasm_bindgen]
    pub fn set_fm_frequency(&mut self, amt: f32) {
        self.fm_freq_ratio = amt;
        self.fm_osc
            .frequency()
            .set_value(self.fm_freq_ratio * self.primary.frequency().value());
    }

    #[wasm_bindgen]
    pub fn set_filter(&mut self, amt: f32) {
        self.filter
            .frequency()
            .set_value(amt);
    }

    #[wasm_bindgen]
    pub fn set_filter_q(&mut self, amt: f32) {
        self.filter
            .q()
            .set_value(amt);
    }
}
