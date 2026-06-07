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
        let view = self.view;
        ui.label(format!("{}: {:.2}", view.kind, view.amount));
        if view.production != 0.0 || view.consumption != 0.0 {
            ui.label(
                egui::RichText::new(format!(
                    "prod {:.3} - use {:.3} = net {:+.3}/s",
                    view.production, view.consumption, view.net
                ))
                .weak()
                .small(),
            );
        }
        let mut submits = Vec::new();
        for form in &view.forms {
            if let Some(submit) = FormWidget::new(form).show(ui) {
                submits.push(submit);
            }
        }
        submits
    }
}
