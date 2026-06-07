use crate::engine::dsl::value::Value;
use crate::engine::dsl::Query;
use crate::types::stat::Stat;
use crate::view::ViewCtx;

pub struct StatsView {
    pub total_humans: f64,
    pub total_buildings: f64,
    pub power_turnover: f64,
    pub efficiency: f64,
}

impl StatsView {
    pub(crate) fn build(ctx: &ViewCtx<'_>) -> Self {
        Self {
            total_humans: ctx.eval(Value::Stat(Stat::TotalHumans)).as_f64(),
            total_buildings: ctx.eval(Value::Stat(Stat::TotalBuildings)).as_f64(),
            power_turnover: ctx.eval(Value::Stat(Stat::PowerTurnover)).as_f64(),
            efficiency: ctx
                .eval(Value::Stat(Stat::HumanProductionEfficiency))
                .as_f64(),
        }
    }
}

pub struct UnlockHint {
    pub name: String,
    pub requirement: String,
}
