#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("failed to install color_eyre hook")]
    InstallHookError(#[from] color_eyre::eyre::Report),

    #[error("failed to parse input")]
    ParseError,
}

pub type Result<T> = color_eyre::Result<T, Error>;
