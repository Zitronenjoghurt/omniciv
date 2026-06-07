use crate::ui::widgets::building::BuildingViewsWidget;
use crate::ui::widgets::human::HumanViewsWidget;
use crate::ui::widgets::resource::ResourceViewsWidget;
use crate::ui::widgets::stats::StatsWidget;
use crate::ui::widgets::technology::TechnologyViewsWidget;
use eframe::{CreationContext, Frame};
use egui::{CentralPanel, Context, FontDefinitions, Ui};
use egui_notify::Toasts;
use omniciv_core::{Game, State, Submit};
use std::time::Duration;

pub struct App {
    game: Game,
    toasts: Toasts,
}

impl App {
    pub fn new(cc: &CreationContext) -> Self {
        Self::setup_fonts(&cc.egui_ctx);
        Self {
            game: Game::new(State::default()),
            toasts: Toasts::default(),
        }
    }

    fn setup_fonts(ctx: &Context) {
        let mut fonts = FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        ctx.set_fonts(fonts);
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        self.game.update();
        self.toasts.show(ui.ctx());

        CentralPanel::default().show_inside(ui, |ui| {
            let view = self.game.view();
            let mut submits: Vec<Submit> = Vec::new();

            ui.columns(5, |columns| {
                submits.extend(ResourceViewsWidget::new(&view.resources).show(&mut columns[0]));
                submits.extend(
                    BuildingViewsWidget::new(&view.buildings, &view.locked_buildings)
                        .show(&mut columns[1]),
                );
                submits.extend(
                    HumanViewsWidget::new(&view.humans, &view.locked_humans).show(&mut columns[2]),
                );
                submits.extend(
                    TechnologyViewsWidget::new(&view.technologies, &view.locked_technologies)
                        .show(&mut columns[3]),
                );
                StatsWidget::new(&view.stats).show(&mut columns[4]);
            });

            for submit in submits {
                if let Err(err) = self.game.submit(submit) {
                    self.toasts.error(err.to_string());
                }
            }

            for event in view.events {
                self.toasts.info(event.to_string());
            }

            ui.ctx().request_repaint_after(Duration::from_millis(100));
        });
    }
}
