use nih_plug::prelude::*;
pub mod noise;
pub mod phantomsilhouette;
pub mod spectral;
pub mod world;
use std::sync::Arc;

// This is a shortened version of the gain example with most comments removed, check out
// https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs to get
// started

struct PhantomSilhouetteRs {
    params: Arc<PhantomSilhouetteRsParams>,
    sample_rate: f32,
}

#[derive(Params)]
struct PhantomSilhouetteRsParams {
    /// Cross-fade between the unprocessed input and the whispered "Phantom
    /// Silhouette" signal. `1.0` keeps only the dry signal while `0.0`
    /// outputs only the whisper.
    #[id = "mix"]
    mix: FloatParam,
    // Additional parameters were removed in the simplified implementation.
}

impl Default for PhantomSilhouetteRs {
    fn default() -> Self {
        Self {
            params: Arc::new(PhantomSilhouetteRsParams::default()),
            sample_rate: 44100.0,
        }
    }
}

impl Default for PhantomSilhouetteRsParams {
    fn default() -> Self {
        Self {
            mix: FloatParam::new("Mix", 1.0, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_unit("")
                .with_smoother(SmoothingStyle::Linear(50.0)),
        }
    }
}

impl Plugin for PhantomSilhouetteRs {
    const NAME: &'static str = "Phantom Silhouette Rs";
    const VENDOR: &'static str = "Daishi Suzuki";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "zukky.rikugame@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        // Individual ports and the layout as a whole can be named here. By default these names
        // are generated as needed. This layout will be called 'Stereo', while a layout with
        // only one input and output channel would be called 'Mono'.
        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        self.sample_rate = buffer_config.sample_rate;
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let mix = self.params.mix.smoothed.next();

        let channels = buffer.as_slice();
        if channels.is_empty() {
            return ProcessStatus::Normal;
        }

        let input: Vec<f64> = channels[0].iter().map(|&s| s as f64).collect();
        let processed = crate::world::phantom_silhouette_signal(&input, self.sample_rate as i32);
        let processed: Vec<f32> = processed.iter().map(|&v| v as f32).collect();

        for ch in channels.iter_mut() {
            for (i, sample) in ch.iter_mut().enumerate() {
                let dry = *sample;
                let phantom = processed[i];
                *sample = mix * dry + (1.0 - mix) * phantom;
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for PhantomSilhouetteRs {
    const CLAP_ID: &'static str = "com.zukky.phantom-silhouette-rs";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("phantom silhouette effect");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for PhantomSilhouetteRs {
    const VST3_CLASS_ID: [u8; 16] = *b"PSPSPSPSPSPSPSPS";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!(PhantomSilhouetteRs);
nih_export_vst3!(PhantomSilhouetteRs);
