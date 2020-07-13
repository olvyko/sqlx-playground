pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, BoxError>;
