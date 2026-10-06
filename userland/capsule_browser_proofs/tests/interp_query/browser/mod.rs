// NONOS Operating System (AGPL-3.0-or-later)
//! The module tree the interpreter's sources expect (crate::browser::*): the
//! proof crate's own css and dom, and the interpreter compiled from the
//! capsule's source beside them.

pub use capsule_browser_proofs::browser::{css, dom};
pub mod js;
