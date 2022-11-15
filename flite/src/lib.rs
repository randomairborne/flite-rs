#![warn(clippy::all, clippy::cargo, clippy::pedantic, clippy::nursery)]
use std::ffi::{CString, NulError};

/// This function accepts a string of text to speak, and a voice to speak it with.
/// # Errors
/// This function returns an error when the voice is invalid, if the returned waveform is null,
/// or if the voice or text are not valid `CStrings`.
/// # Panics
/// This function panics if flite returns a negative length for the sample count.
pub fn text_to_wave<'a>(
    text: impl Into<String>,
    voice: impl Into<String>,
) -> Result<WaveformAudio<'a>, Error>
{
    let unsafe_wf = unsafe {
        flite_sys::flite_init();
        let c_text = CString::new(text.into())?.as_ptr();
        let c_voice = flite_sys::flite_voice_select(CString::new(voice.into())?.as_ptr());
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
    let sample_count: usize = wf.num_samples.try_into().unwrap();
    if wf.samples.is_null() {
        return Err(Error::SamplesNull);
    }
    if wf.samples.is_null() {
        return Err(Error::SamplesNull);
    }
    let samples = unsafe { std::slice::from_raw_parts(wf.samples, sample_count) };
    Ok(WaveformAudio {
        channels: wf.num_channels,
        sample_rate: wf.sample_rate,
        kind: unsafe { CString::from_raw(wf.type_ as *mut i8) },
        samples,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaveformAudio<'a> {
    channels: i32,
    sample_rate: i32,
    kind: CString,
    samples: &'a [i16],
}

impl WaveformAudio<'_> {
    #[must_use]
    pub const fn channels(&self) -> i32 {
        self.channels
    }
    #[must_use]
    pub const fn sample_rate(&self) -> i32 {
        self.sample_rate
    }
    #[must_use]
    pub const fn kind(&self) -> &CString {
        &self.kind
    }
    #[must_use]
    pub const fn samples(&self) -> &[i16] {
        self.samples
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("CString conversion error: {0}")]
    CStringConversion(#[from] NulError),
    #[error("There was no voice with the selected name!")]
    NoVoice,
    #[error("Text-to-Wave return value was NULL")]
    TextToWaveNull,
    #[error("Samples in waveform are NULL")]
    SamplesNull,
    #[error("Type in waveform is NULL")]
    TypeNull,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[should_panic]
    fn invalid_voice() {
        text_to_wave("Example text", "NAGIGADJG").unwrap();
    }
}
