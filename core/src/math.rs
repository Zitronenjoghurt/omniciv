pub mod modifiers;
pub mod quantity;

pub fn step_resource(rate: f64, amount: f64, dt: f64, cap: f64) -> f64 {
    (amount + rate * dt).clamp(0.0, cap)
}

pub fn scale_cost(base: f64, ratio: f64, n: u32) -> f64 {
    base * ratio.powi(n as i32)
}
