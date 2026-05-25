use miette::{LabeledSpan, MietteDiagnostic};

use crate::cst::SpanStore;

pub type CFoodResult<T> = Result<T, CFoodError>;

#[derive(Clone, Debug, Default)]
pub struct CFoodError {
    pub message: String,
    pub help: Option<String>,
    pub labels: Vec<CFoodErrorLabel>,
}

#[derive(Clone, Debug, Default)]
pub struct CFoodErrorLabel {
    pub cst_id: usize,
    pub label: Option<String>,
}

impl CFoodError {
    pub fn to_diagnostic(self, store: SpanStore) -> MietteDiagnostic {
        let mut diag = MietteDiagnostic::new(self.message);
        diag.help = self.help;

        for label in self.labels {
            let Some(span) = store.get(&label.cst_id) else {
                continue;
            };

            diag = diag.with_label(LabeledSpan::new_with_span(
                label.label,
                span.start.offset..span.end.offset,
            ));
        }

        diag
    }
}
