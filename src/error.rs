use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("can not load extention {0}")]
    WrongExtenshion(String),
    #[error("failed to find config: {0}")]
    NoConfig(String),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("Ron Spanned Error")]
    RonSpannedError(#[from] ron::error::SpannedError),
    #[error("Ron Error")]
    RonError(#[from] ron::Error),
    #[error("Field Not Found")]
    FieldNotFound(String),
    #[error("Found Wrong Char '{0}' should be '{1}'")]
    WrongChar(char, char),
    #[error("Float Parse Err")]
    FloatErr(#[from] std::num::ParseFloatError),
    #[error("Tag Err")]
    TagErr(#[from] crate::item::tags::TagError),
}