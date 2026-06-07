pub mod modifiers;
pub mod quantity;

pub fn step_resource(rate: f64, amount: f64, dt: f64, cap: f64) -> f64 {
    (amount + rate * dt).clamp(0.0, cap)
}
