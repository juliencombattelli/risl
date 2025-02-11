use super::diagnostic::DiagContext;

pub struct ParseContext {
    diag_ctx: DiagContext,
}

impl ParseContext {
    pub fn new(diag_ctx: DiagContext) -> Self {
        Self { diag_ctx }
    }
}
