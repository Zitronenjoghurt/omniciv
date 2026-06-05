use crate::defs::subject::Subject;

pub struct EvalContext {
    pub this: Option<Subject>,
}

impl EvalContext {
    pub fn new(subject: Subject) -> Self {
        Self {
            this: Some(subject),
        }
    }

    pub fn empty() -> Self {
        Self { this: None }
    }

    pub fn resolve_subject(&self, subject: Subject) -> Subject {
        match subject {
            Subject::This => self.this.expect("'This' used outside an entity context"),
            other => other,
        }
    }
}
