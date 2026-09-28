// NONOS Operating System (AGPL-3.0-or-later)
// The real signal numbers and the real delivery predicates from
// src/process/signal. `helpers.rs` reads its constants through `super`, which
// is this module, so both have to be mirrored and nothing else does.
#[path = "../../../../../../src/process/signal/constants.rs"]
pub mod constants;

#[path = "../../../../../../src/process/signal/helpers.rs"]
pub mod helpers;
