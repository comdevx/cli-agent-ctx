pub mod decisions;
pub mod formatter;
/// Core business logic — no CLI/terminal awareness.
///
/// Pure functions: input in, output out, no side effects.
pub mod git;
pub mod snapshot;
