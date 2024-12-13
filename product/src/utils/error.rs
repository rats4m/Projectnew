#[derive(Debug)]
pub enum PipelineError {
    FileLoadError(String),
    PreprocessError(String),
    DetectionError(String),
    MissingKeyError(String),
    InvalidDataError(String),
    ThresholdViolation(String),
}

impl std::fmt::Display for PipelineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineError::FileLoadError(msg) => write!(f, "File Load Error: {}", msg),
            PipelineError::PreprocessError(msg) => write!(f, "Preprocess Error: {}", msg),
            PipelineError::DetectionError(msg) => write!(f, "Detection Error: {}", msg),
            PipelineError::MissingKeyError(key) => {
                write!(f, "Missing Key Error: Required key '{}' is missing.", key)
            }
            PipelineError::InvalidDataError(data) => {
                write!(f, "Invalid Data Error: Data '{}' is not in the expected format.", data)
            }
            PipelineError::ThresholdViolation(msg) => {
                write!(f, "Threshold Violation: {}", msg)
            }
        }
    }
}

impl std::error::Error for PipelineError {}
