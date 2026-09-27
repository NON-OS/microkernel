// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root for the constant-time comparison primitives.
//!
//! Both of the tree's implementations are mirrored here, under the module paths
//! they really have, so a theorem can name either one without ambiguity. The
//! real sources are included via #[path]; nothing is copied or retyped.

pub mod crypto;
pub mod security;
