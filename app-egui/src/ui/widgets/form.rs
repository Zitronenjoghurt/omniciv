use egui::{Id, Ui};
use omniciv_core::{FieldInput, FieldValue, Form, FormMode, Note, Submit};

pub struct FormWidget<'a> {
    form: &'a Form,
}

impl<'a> FormWidget<'a> {
    pub fn new(form: &'a Form) -> Self {
        Self { form }
    }

    pub fn show(self, ui: &mut Ui) -> Option<Submit> {
        let live = matches!(self.form.mode, FormMode::Live);
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
                    let (value, changed) = show_field(ui, id, &field.input, live);
                    values.push(value);
                    if changed && live {
                        commit = true;
                    }
                });
            }

            if !live {
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
                .map(|c| format!("{:.2} {}", c.amount, c.resource))
                .collect::<Vec<_>>()
                .join(", ");
            ui.label(format!("Cost: {text}"));
        }
        Note::Tooltip(text) | Note::Description(text) => {
            ui.label(text.as_str());
        }
    }
}

fn field_state<T>(ui: &mut Ui, id: Id, value: T, live: bool) -> T
where
    T: Clone + Send + Sync + 'static,
{
    if live {
        value
    } else {
        ui.data_mut(|d| d.get_temp::<T>(id)).unwrap_or(value)
    }
}

fn show_field(ui: &mut Ui, id: Id, input: &FieldInput, live: bool) -> (FieldValue, bool) {
    match input {
        FieldInput::Toggle { value } => {
            let mut v = field_state(ui, id, *value, live);
            let changed = ui.checkbox(&mut v, "").changed();
            ui.data_mut(|d| d.insert_temp(id, v));
            (FieldValue::Bool(v), changed)
        }
        FieldInput::Stepper {
            value,
            min,
            max,
            quick_steps,
            allow_max,
        } => {
            let mut v = field_state(ui, id, *value, live).clamp(*min, *max);
            let mut changed = false;
            for step in quick_steps.iter().rev() {
                if ui.small_button(format!("-{step}")).clicked() {
                    v = (v - step).clamp(*min, *max);
                    changed = true;
                }
            }
            changed |= ui
                .add(egui::DragValue::new(&mut v).range((*min as f64)..=(*max as f64)))
                .changed();
            for step in quick_steps {
                if ui.small_button(format!("+{step}")).clicked() {
                    v = (v + step).clamp(*min, *max);
                    changed = true;
                }
            }
            if *allow_max && ui.small_button("Max").clicked() {
                v = *max;
                changed = true;
            }
            ui.data_mut(|d| d.insert_temp(id, v));
            (FieldValue::Int(v), changed)
        }
        FieldInput::Slider {
            value,
            min,
            max,
            step,
        } => {
            let mut v = field_state(ui, id, *value, live);
            let changed = ui
                .add(egui::Slider::new(&mut v, *min..=*max).step_by(*step))
                .changed();
            ui.data_mut(|d| d.insert_temp(id, v));
            (FieldValue::Float(v), changed)
        }
        FieldInput::Selector { selected, options } => {
            let mut idx = field_state(ui, id, *selected, live);
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
