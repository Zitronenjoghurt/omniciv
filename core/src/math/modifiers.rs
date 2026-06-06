#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ModifierKind {
    Add,
    Sub,
    Mul,
    Exp,
}

#[derive(Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Modifiers {
    additive: f64,
    multiplicative: f64,
    exponential: f64,
}

impl Default for Modifiers {
    fn default() -> Self {
        Self {
            additive: 0.0,
            multiplicative: 1.0,
            exponential: 1.0,
        }
    }
}

impl Modifiers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_value(value: f64) -> Self {
        Self {
            additive: value,
            multiplicative: 1.0,
            exponential: 1.0,
        }
    }

    pub fn insert(&mut self, kind: ModifierKind, value: f64) {
        match kind {
            ModifierKind::Add => self.add(value),
            ModifierKind::Sub => self.add(-value),
            ModifierKind::Mul => self.mul(value),
            ModifierKind::Exp => self.exp(value),
        }
    }

    pub fn add(&mut self, value: f64) {
        self.additive += value;
    }

    pub fn sub(&mut self, value: f64) {
        self.additive -= value;
    }

    pub fn mul(&mut self, value: f64) {
        self.multiplicative *= value;
    }

    pub fn exp(&mut self, value: f64) {
        self.exponential *= value;
    }

    pub fn apply(&self, value: f64) -> f64 {
        ((value + self.additive) * (self.multiplicative)).powf(self.exponential)
    }

    pub fn value(&self) -> f64 {
        self.apply(0.0)
    }
}
