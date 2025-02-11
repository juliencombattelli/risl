use super::emitter::Emitter;

pub enum Level {
    /// For bugs in the compiler. Manifests as an ICE (internal compiler error) panic.
    Bug,
    /// An error that causes an immediate abort. Used for things like configuration errors,
    /// internal overflows, some file operation errors.
    Fatal,
    /// An error in the code being compiled, which prevents compilation from finishing. This is the
    /// most common case.
    Error,
    /// A warning about the code being compiled. Does not prevent compilation from finishing.
    /// Will be skipped if `can_emit_warnings` is false.
    Warning,
    /// A message giving additional context.
    Note,
    /// A message suggesting how to fix something.
    Help,
}

pub struct Diagnostic {
    level: Level,
}

pub struct DiagContext {
    diagnostics: Vec<Diagnostic>,
    emitter: Box<dyn Emitter>,
}

impl DiagContext {
    pub fn new(emitter: Box<dyn Emitter>) -> Self {
        Self {
            diagnostics: Vec::new(),
            emitter,
        }
    }
}
