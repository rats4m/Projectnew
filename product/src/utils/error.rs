use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    FileLoadError(String),
    VisualizationError(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::FileLoadError(msg) => write!(f, "File Load Error: {}", msg),
            CustomError::VisualizationError(msg) => write!(f, "Visualization Error: {}", msg),
        }
    }
}

impl std::error::Error for CustomError {}
