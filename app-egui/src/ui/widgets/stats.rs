use egui::{ScrollArea, Ui};
use omniciv_core::StatsView;

pub struct StatsWidget<'a> {
    stats: &'a StatsView,
}

impl<'a> StatsWidget<'a> {
    pub fn new(stats: &'a StatsView) -> Self {
        Self { stats }
    }

    pub fn show(self, ui: &mut Ui) {
        ui.heading("Stats");
        ScrollArea::vertical().id_salt("stats").show(ui, |ui| {
            let stats = self.stats;
            ui.label(format!("Humans: {:.0}", stats.total_humans));
            ui.label(format!("Buildings: {:.0}", stats.total_buildings));
            ui.label(format!("Power: {:.2} W", stats.power_turnover));
            ui.label(format!("Human efficiency: {:.0}%", stats.efficiency * 100.0));
        });
    }
}
