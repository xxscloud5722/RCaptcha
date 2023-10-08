use image::ImageError;

#[derive(thiserror::Error, Debug)]
pub enum CaptchaError {
    #[error("An error occurred: {0}")]
    Error(String),

    #[error("An IO error occurred: {0}")]
    IoError(#[from] std::io::Error),

    #[error("An option error occurred")]
    OptionError,

    #[error("An image error occurred: {0}")]
    ImageError(#[from] ImageError),

    #[error("An image error occurred: {0}")]
    JniError(#[from] jni::errors::Error),
}

impl From<CaptchaError> for Option<()> {
    fn from(error: CaptchaError) -> Self {
        match error {
            CaptchaError::OptionError => None,
            _ => Some(()),
        }
    }
}