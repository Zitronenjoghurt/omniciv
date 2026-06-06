use crate::ui::widgets::form::FormWidget;
use egui::{ScrollArea, Ui};
use omniciv_core::{BuildingView, Submit};

pub struct BuildingViewsWidget<'a> {
    views: &'a [BuildingView],
}

impl<'a> BuildingViewsWidget<'a> {
    pub fn new(views: &'a [BuildingView]) -> Self {
        Self { views }
    }

    pub fn show(self, ui: &mut Ui) -> Vec<Submit> {
        ui.heading("Buildings");
        let mut submits = Vec::new();
        ScrollArea::vertical().id_salt("buildings").show(ui, |ui| {
            for view in self.views {
                ui.separator();
                submits.extend(BuildingViewWidget::new(view).show(ui));
            }
        });
        submits
    }
}

pub struct BuildingViewWidget<'a> {
    view: &'a BuildingView,
}

impl<'a> BuildingViewWidget<'a> {
    pub fn new(view: &'a BuildingView) -> Self {
        Self { view }
    }

    pub fn show(self, ui: &mut Ui) -> Vec<Submit> {
        ui.label(format!("{} — {:.0}", self.view.kind, self.view.count));
        let mut submits = Vec::new();
        for form in &self.view.forms {
            if let Some(submit) = FormWidget::new(form).submit_button().show(ui) {
                submits.push(submit);
            }
        }
        submits
    }
}
