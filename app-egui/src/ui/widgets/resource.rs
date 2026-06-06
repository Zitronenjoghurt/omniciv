use crate::ui::widgets::form::FormWidget;
use egui::{ScrollArea, Ui};
use omniciv_core::{ResourceView, Submit};

pub struct ResourceViewsWidget<'a> {
    views: &'a [ResourceView],
}

impl<'a> ResourceViewsWidget<'a> {
    pub fn new(views: &'a [ResourceView]) -> Self {
        Self { views }
    }

    pub fn show(self, ui: &mut Ui) -> Vec<Submit> {
        ui.heading("Resources");
        let mut submits = Vec::new();
        ScrollArea::vertical().id_salt("resources").show(ui, |ui| {
            for view in self.views {
                submits.extend(ResourceViewWidget::new(view).show(ui));
            }
        });
        submits
    }
}

pub struct ResourceViewWidget<'a> {
    view: &'a ResourceView,
}

impl<'a> ResourceViewWidget<'a> {
    pub fn new(view: &'a ResourceView) -> Self {
        Self { view }
    }

    pub fn show(self, ui: &mut Ui) -> Vec<Submit> {
        ui.label(format!("{}: {:.0}", self.view.kind, self.view.amount));
        let mut submits = Vec::new();
        for form in &self.view.forms {
            if let Some(submit) = FormWidget::new(form).submit_button().show(ui) {
                submits.push(submit);
            }
        }
        submits
    }
}
