#[derive(Debug)]
pub enum ValueError {
    IsZero,
    IsNaN,
    IsInfinite,
    IsNegative,
}

impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::IsZero => write!(f, "value cannot be zero"),
            Self::IsNaN => write!(f, "value cannot be NaN"),
            Self::IsInfinite => write!(f, "value cannot be infinite"),
            Self::IsNegative => write!(f, "value cannot be negative"),
        }
    }
}

impl std::error::Error for ValueError {}
