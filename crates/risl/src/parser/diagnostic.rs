use super::emitter::Emitter;

pub struct Diagnostic {}

pub struct DiagContext {
    diagnostics: Vec<Diagnostic>,
    emitter: Box<dyn Emitter>,
}
