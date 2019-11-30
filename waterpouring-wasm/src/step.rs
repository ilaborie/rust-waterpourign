use waterpouring_model::operations::Operation;
use waterpouring_model::state::State;

use crate::operation::WasmOperation;

#[derive(Serialize)]
pub(crate) struct WasmStep {
    from: String,
    operation: Option<WasmOperation>,
    to: Option<String>,
}

impl WasmStep {
    pub(crate) fn init(state: &State) -> Self {
        let from: String = format!("{}", state);

        Self {
            from,
            operation: None,
            to: None,
        }
    }

    pub(crate) fn step(state: &State, ope: &Operation) -> (Self, State) {
        let from: String = format!("{}", state);
        let operation = Some(WasmOperation::new(ope));
        let next = state.apply(*ope);
        let to: Option<String> = Some(format!("{}", next));
        let step = Self {
            from,
            operation,
            to,
        };

        (step, next)
    }
}
