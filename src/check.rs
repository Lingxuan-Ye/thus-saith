use crate::error::ValueError;

pub trait SanityCheck: Sized {
    fn ensure_non_nan(self) -> Result<Self, ValueError>;

    fn ensure_non_zero(self) -> Result<Self, ValueError>;

    fn ensure_positive(self) -> Result<Self, ValueError>;

    fn ensure_finite(self) -> Result<Self, ValueError>;
}

impl SanityCheck for f64 {
    fn ensure_non_nan(self) -> Result<Self, ValueError> {
        if self.is_nan() {
            Err(ValueError::IsNaN)
        } else {
            Ok(self)
        }
    }

    fn ensure_non_zero(self) -> Result<Self, ValueError> {
        if self == 0.0 {
            Err(ValueError::IsZero)
        } else {
            Ok(self)
        }
    }

    fn ensure_positive(self) -> Result<Self, ValueError> {
        if self.is_sign_negative() {
            Err(ValueError::IsNegative)
        } else {
            Ok(self)
        }
    }

    fn ensure_finite(self) -> Result<Self, ValueError> {
        if self.is_infinite() {
            Err(ValueError::IsInfinite)
        } else {
            Ok(self)
        }
    }
}
