use egui::{Id, Ui};
use omniciv_core::{FieldInput, FieldValue, Form, Note, Submit};

pub struct FormWidget<'a> {
    form: &'a Form,
    /// If not set, the form is submitted on every field change (live controls).
    /// If set, fields are edited freely and submitted only on button click.
    submit_button: bool,
}

impl<'a> FormWidget<'a> {
    pub fn new(form: &'a Form) -> Self {
        Self {
            form,
            submit_button: false,
        }
    }

    pub fn submit_button(mut self) -> Self {
        self.submit_button = true;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Option<Submit> {
        let mut commit = false;
        let mut values = Vec::with_capacity(self.form.fields.len());

        ui.group(|ui| {
            for note in &self.form.notes {
                show_note(ui, note);
            }

            for (index, field) in self.form.fields.iter().enumerate() {
                let id = ui.make_persistent_id((&self.form.id, index));
                ui.horizontal(|ui| {
                    ui.label(field.label.as_str());
                    let (value, changed) = show_field(ui, id, &field.input);
                    values.push(value);
                    if changed && !self.submit_button {
                        commit = true;
                    }
                });
            }

            if self.submit_button {
                let button = egui::Button::new(self.form.label.as_str());
                if ui.add_enabled(self.form.enabled, button).clicked() {
                    commit = true;
                }
            }
        });

        (commit && self.form.enabled).then(|| Submit {
            form: self.form.id.clone(),
            values,
        })
    }
}

fn show_note(ui: &mut Ui, note: &Note) {
    match note {
        Note::Cost(costs) => {
            let text = costs
                .iter()
                .map(|c| format!("{} {}", c.amount, c.resource))
                .collect::<Vec<_>>()
                .join(", ");
            ui.label(format!("Cost: {text}"));
        }
        Note::Tooltip(text) | Note::Description(text) => {
            ui.label(text.as_str());
        }
    }
}

fn show_field(ui: &mut Ui, id: Id, input: &FieldInput) -> (FieldValue, bool) {
    match input {
        FieldInput::Toggle { value } => {
            let mut v = ui.data_mut(|d| d.get_temp::<bool>(id)).unwrap_or(*value);
            let changed = ui.checkbox(&mut v, "").changed();
            ui.data_mut(|d| d.insert_temp(id, v));
            (FieldValue::Bool(v), changed)
        }
        FieldInput::Stepper { value, min, max } => {
            let mut v = ui.data_mut(|d| d.get_temp::<i64>(id)).unwrap_or(*value);
            v = v.clamp(*min, *max);
            let changed = ui
                .add(egui::DragValue::new(&mut v).range((*min as f64)..=(*max as f64)))
                .changed();
            ui.data_mut(|d| d.insert_temp(id, v));
            (FieldValue::Int(v), changed)
        }
        FieldInput::Slider {
            value,
            min,
            max,
            step,
        } => {
            let mut v = ui.data_mut(|d| d.get_temp::<f64>(id)).unwrap_or(*value);
            let changed = ui
                .add(egui::Slider::new(&mut v, *min..=*max).step_by(*step))
                .changed();
            ui.data_mut(|d| d.insert_temp(id, v));
            (FieldValue::Float(v), changed)
        }
        FieldInput::Selector { selected, options } => {
            let mut idx = ui
                .data_mut(|d| d.get_temp::<usize>(id))
                .unwrap_or(*selected);
            let mut changed = false;
            egui::ComboBox::from_id_salt(id)
                .selected_text(options.get(idx).map(String::as_str).unwrap_or(""))
                .show_ui(ui, |ui| {
                    for (i, option) in options.iter().enumerate() {
                        changed |= ui.selectable_value(&mut idx, i, option.as_str()).changed();
                    }
                });
            ui.data_mut(|d| d.insert_temp(id, idx));
            (FieldValue::Choice(idx), changed)
        }
    }
}
