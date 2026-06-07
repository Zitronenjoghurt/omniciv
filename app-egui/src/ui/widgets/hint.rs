use egui::{RichText, Ui};
use omniciv_core::UnlockHint;

pub fn show_unlock_hints(ui: &mut Ui, hints: &[UnlockHint]) {
    if hints.is_empty() {
        return;
    }
    ui.separator();
    ui.label(RichText::new("Locked").weak());
    for hint in hints {
        ui.label(RichText::new(format!("{} — needs {}", hint.name, hint.requirement)).weak());
    }
}
