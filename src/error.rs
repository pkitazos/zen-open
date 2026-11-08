use std::{io, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown profile '{name}'. Available: {}", available.join(", "))]
    UnknownProfile {
        name: String,
        available: Vec<String>,
    },

    #[error("Zen not found at {0}")]
    ZenNotFound(PathBuf),

    #[error("profiles.ini not found at {0}")]
    IniNotFound(PathBuf),

    #[error("malformed profiles.ini: {0}")]
    MalformedIni(String),

    #[error("profile '{0}' is running but focus or URL handover failed")]
    HandoverFailed(String),

    #[error(transparent)]
    Io(#[from] io::Error),
}

impl Error {
    pub fn exit_code(&self) -> u8 {
        match self {
            Error::UnknownProfile { .. } => 2,
            Error::ZenNotFound(_) | Error::IniNotFound(_) => 3,
            Error::HandoverFailed(_) => 4,
            Error::MalformedIni(_) | Error::Io(_) => 1,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
