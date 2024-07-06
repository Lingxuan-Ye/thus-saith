use anyhow::{ensure, Result};

pub struct Arg<'a> {
    pub name: &'a str,
    pub value: f64,
}

impl<'a> Arg<'a> {
    pub fn new(name: &'a str, value: f64) -> Self {
        Self { name, value }
    }

    pub fn ensure_non_nan(&self) -> Result<&Self> {
        ensure!(!self.value.is_nan(), "`{}` cannot be NaN", self.name);
        Ok(self)
    }

    pub fn ensure_finite(&self) -> Result<&Self> {
        ensure!(self.value.is_finite(), "`{}` must be finite", self.name);
        Ok(self)
    }

    pub fn ensure_non_zero(&self) -> Result<&Self> {
        ensure!(self.value != 0.0, "`{}` cannot be zero", self.name);
        Ok(self)
    }

    pub fn ensure_non_negative(&self) -> Result<&Self> {
        ensure!(
            !self.value.is_sign_negative(),
            "`{}` cannot be negative",
            self.name
        );
        Ok(self)
    }
}
