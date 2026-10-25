// NONOS Operating System (AGPL-3.0-or-later)
//! F-123: the capsule's bring-up under the budget. A DSDT whose load is cut
//! short is refused, and the capsule's own rule (`boot/load_rule.rs`)
//! leaves an empty namespace, so nothing AML-backed answers: fail closed.
//! Each bring-up phase (connect and _REG, \_PIC, _OSC, _INI) cut at every
//! step fails as a whole, is said so by the capsule's own wording
//! (`boot/bringup_say.rs`), leaves nothing held, and leaves the other
//! phases and the routes the capsule serves as they would have been.

pub(crate) mod capsule;
mod dsdt_tests;
mod fixture;
mod phase_tests;
mod phases;
