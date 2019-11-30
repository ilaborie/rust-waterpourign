use crate::step::WasmStep;

#[derive(Serialize)]
pub(crate) struct WasmResult {
    error: Option<String>,
    steps: Vec<WasmStep>,
}

impl WasmResult {
    pub(crate) fn error(str: String) -> Self {
        Self {
            error: Some(str),
            steps: vec![],
        }
    }

    pub(crate) fn solved(steps: Vec<WasmStep>) -> Self {
        Self { error: None, steps }
    }
}
