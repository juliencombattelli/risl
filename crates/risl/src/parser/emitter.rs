pub trait Emitter {}

/// An emitter printing diagnostics to the standard output.
struct EmitterHumanReadable();

impl Emitter for EmitterHumanReadable {}

pub fn new_emitter_human_readable() -> Box<dyn Emitter> {
    return Box::new(EmitterHumanReadable());
}

/// An emitter discarding all diagnostics emitted.
struct EmitterNone();

impl Emitter for EmitterNone {}

pub fn new_emitter_none() -> Box<dyn Emitter> {
    return Box::new(EmitterNone());
}
