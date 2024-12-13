#[derive(Debug)]
pub enum PipelineError {
    FileLoadError(String),
    PreprocessError(String),
    DetectionError(String),
}

impl std::fmt::Display for PipelineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineError::FileLoadError(msg) => write!(f, "File Load Error: {}", msg),
            PipelineError::PreprocessError(msg) => write!(f, "Preprocess Error: {}", msg),
            PipelineError::DetectionError(msg) => write!(f, "Detection Error: {}", msg),
        }
    }
}

impl std::error::Error for PipelineError {}
