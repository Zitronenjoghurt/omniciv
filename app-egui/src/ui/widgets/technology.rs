use crate::ui::widgets::form::FormWidget;
use crate::ui::widgets::hint::show_unlock_hints;
use egui::{ScrollArea, Ui};
use omniciv_core::{Submit, TechnologyView, UnlockHint};

pub struct TechnologyViewsWidget<'a> {
    views: &'a [TechnologyView],
    locked: &'a [UnlockHint],
}

impl<'a> TechnologyViewsWidget<'a> {
    pub fn new(views: &'a [TechnologyView], locked: &'a [UnlockHint]) -> Self {
        Self { views, locked }
    }

    pub fn show(self, ui: &mut Ui) -> Vec<Submit> {
        ui.heading("Technologies");
        let mut submits = Vec::new();
        ScrollArea::vertical().id_salt("technologies").show(ui, |ui| {
            for view in self.views {
                ui.separator();
                submits.extend(TechnologyViewWidget::new(view).show(ui));
            }
            show_unlock_hints(ui, self.locked);
        });
        submits
    }
}

pub struct TechnologyViewWidget<'a> {
    view: &'a TechnologyView,
}

impl<'a> TechnologyViewWidget<'a> {
    pub fn new(view: &'a TechnologyView) -> Self {
        Self { view }
    }

    pub fn show(self, ui: &mut Ui) -> Vec<Submit> {
        let status = if self.view.researched {
            "researched"
        } else {
            "available"
        };
        ui.label(format!("{} — {status}", self.view.kind));
        let mut submits = Vec::new();
        for form in &self.view.forms {
            if let Some(submit) = FormWidget::new(form).show(ui) {
                submits.push(submit);
            }
        }
        submits
    }
}
