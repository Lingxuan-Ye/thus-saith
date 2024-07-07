use anyhow::{ensure, Result};

#[derive(Clone, Copy)]
pub struct NonNegFinite(f64);

impl NonNegFinite {
    pub fn build(value: f64) -> Result<Self> {
        let value = F64(value)
            .ensure_non_nan()?
            .ensure_finite()?
            .ensure_non_negative()?
            .value();
        Ok(Self(value))
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct PosFinite(f64);

impl PosFinite {
    pub fn build(value: f64) -> Result<Self> {
        let value = F64(value)
            .ensure_non_nan()?
            .ensure_finite()?
            .ensure_non_zero()?
            .ensure_non_negative()?
            .value();
        Ok(Self(value))
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

struct F64(f64);

impl F64 {
    fn value(self) -> f64 {
        self.0
    }

    fn ensure_non_nan(self) -> Result<Self> {
        ensure!(!self.0.is_nan(), "value cannot be NaN");
        Ok(self)
    }

    fn ensure_finite(self) -> Result<Self> {
        ensure!(self.0.is_finite(), "value must be finite");
        Ok(self)
    }

    fn ensure_non_zero(self) -> Result<Self> {
        ensure!(self.0 != 0.0, "value cannot be zero");
        Ok(self)
    }

    fn ensure_non_negative(self) -> Result<Self> {
        ensure!(
            self.0.is_sign_positive() || self.0 == -0.0,
            "value cannot be negative"
        );
        Ok(self)
    }
}
