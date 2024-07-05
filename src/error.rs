#[derive(Debug)]
pub enum ValueError {
    IsNaN,
    IsZero,
    IsNegative,
    IsInfinite,
}

impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::IsNaN => write!(f, "value cannot be NaN"),
            Self::IsZero => write!(f, "value cannot be zero"),
            Self::IsNegative => write!(f, "value cannot be negative"),
            Self::IsInfinite => write!(f, "value cannot be infinite"),
        }
    }
}

impl std::error::Error for ValueError {}
