use std::{error::Error, fmt, io};

#[derive(Debug)]
pub enum TgError {
    IO(io::Error),
    Decoding,
    Serializing,
}

impl Error for TgError {
    // fn source(&self) -> Option<&(dyn Error + 'static)> {
    //     match self {
    //         Self::IO(e) => None,
    //         _ => None,
    //     }
    // }
}

impl fmt::Display for TgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IO(_e) => {
                write!(f, "{}", _e)
            }
            Self::Decoding | Self::Serializing => {
                write!(
                    f,
                    "{}",
                    match self {
                        Self::IO(_e) => "Doing IO",
                        Self::Serializing => "Serializing",
                        _ => "Unknown",
                    }
                )
            }
        }
    }
}
