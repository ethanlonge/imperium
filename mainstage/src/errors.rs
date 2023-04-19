use thiserror::Error;

#[derive(Error, Debug)]
pub enum MsError {
    #[error("Could not find file `{0}` when loading Concert")]
    FileLoadError(String),

    #[error("Plist for `{0}` didnt load correctly")]
    PlistError(String),

    #[error("Missing value `{0}` in `{1}`")]
    MissingPlistValue(String, String),

    #[error("An unknown error occurred")]
    Unknown
}