use thiserror::Error as ThisError;

pub type AppResult<T> = Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Cannot open config file. please check config file permissions or file path.")]
    OpenConfigFileFailed,
    #[error("Cannot parse config file. please check spellings and syntax.")]
    ParsingConfigFileFailed
}