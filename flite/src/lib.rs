use std::ffi::NulError;

fn text_to_wave(text: String, voice: String) -> Result<Vec<u8>, Error> {
    unsafe {
        flite_sys::flite_init();
        let c_text = std::ffi::CString::new(text)?.as_ptr();
        let c_voice = flite_sys::flite_voice_select(std::ffi::CString::new(voice)?.as_ptr());
        if c_voice.is_null() {
            return Err(Error::Voice(""))
        }
        flite_sys::flite_text_to_wave(c_text, c_voice);
    }
    Ok(Vec::new())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Nul error: {0}")]
    Nul(#[from] NulError),
    #[error("Voice error: {0}")]
    Voice(&'static str),
}
