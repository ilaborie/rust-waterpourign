use waterpouring_model::operations::Operation;
use waterpouring_model::operations::Operation::{Empty, Fill, Pour};

#[derive(Serialize)]
pub(crate) struct WasmOperation {
    from: Option<usize>,
    to: Option<usize>,
}

impl WasmOperation {
    pub(crate) fn new(ope: &Operation) -> Self {
        let (from, to) = match ope {
            Empty { glass: i } => (Some(*i), None),
            Fill { glass: i } => (None, Some(*i)),
            Pour { from: f, to: t } => (Some(*f), Some(*t)),
        };

        Self { from, to }
    }
}
