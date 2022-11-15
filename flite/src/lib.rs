#![warn(clippy::all, clippy::cargo, clippy::pedantic, clippy::nursery)]
use std::ffi::{CString, NulError};

/// This function accepts a string of text to speak, and a voice to speak it with.
/// # Errors
/// This function returns an error when the voice is invalid, if the returned waveform is null,
/// or if the voice or text are not valid `CStrings`.
/// # Panics
/// This function panics if flite returns a negative length for the sample count.
pub fn text_to_wave(text: String, voice: String) -> Result<WaveformAudio, Error> {
    let unsafe_wf = unsafe {
        flite_sys::flite_init();
        let c_text = CString::new(text)?.as_ptr();
        let c_voice = flite_sys::flite_voice_select(CString::new(voice)?.as_ptr());
        if c_voice.is_null() {
            return Err(Error::NoVoice);
        }
        flite_sys::flite_text_to_wave(c_text, c_voice)
    };
    let wf = if unsafe_wf.is_null() {
        return Err(Error::TextToWaveNull);
    } else {
        unsafe { *unsafe_wf }
    };
    // flite should never return a negative length
    let samples: Vec<i16> = Vec::with_capacity(wf.num_samples.try_into().unwrap());
    Ok(WaveformAudio {
        channels: wf.num_channels,
        sample_rate: wf.sample_rate,
        kind: CString::new("")?,
        samples,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaveformAudio {
    channels: i32,
    sample_rate: i32,
    kind: CString,
    samples: Vec<i16>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("CString conversion error: {0}")]
    CStringConversion(#[from] NulError),
    #[error("There was no voice with the selected name!")]
    NoVoice,
    #[error("Text-to-Wave return value was NULL")]
    TextToWaveNull,
}
