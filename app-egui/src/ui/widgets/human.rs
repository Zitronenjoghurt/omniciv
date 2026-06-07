use crate::ui::widgets::form::FormWidget;
use crate::ui::widgets::hint::show_unlock_hints;
use egui::{ScrollArea, Ui};
use omniciv_core::{HumanView, Submit, UnlockHint};

pub struct HumanViewsWidget<'a> {
    views: &'a [HumanView],
    locked: &'a [UnlockHint],
}

impl<'a> HumanViewsWidget<'a> {
    pub fn new(views: &'a [HumanView], locked: &'a [UnlockHint]) -> Self {
        Self { views, locked }
    }

    pub fn show(self, ui: &mut Ui) -> Vec<Submit> {
        ui.heading("Roles");
        let mut submits = Vec::new();
        ScrollArea::vertical().id_salt("humans").show(ui, |ui| {
            for view in self.views {
                ui.separator();
                submits.extend(HumanViewWidget::new(view).show(ui));
            }
            show_unlock_hints(ui, self.locked);
        });
        submits
    }
}

pub struct HumanViewWidget<'a> {
    view: &'a HumanView,
}

impl<'a> HumanViewWidget<'a> {
    pub fn new(view: &'a HumanView) -> Self {
        Self { view }
    }

    pub fn show(self, ui: &mut Ui) -> Vec<Submit> {
        ui.label(format!("{} — {:.0}", self.view.kind, self.view.count));
        let mut submits = Vec::new();
        for form in &self.view.forms {
            if let Some(submit) = FormWidget::new(form).show(ui) {
                submits.push(submit);
            }
        }
        submits
    }
}
