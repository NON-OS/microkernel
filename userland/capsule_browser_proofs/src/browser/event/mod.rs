// NONOS Operating System (AGPL-3.0-or-later)
//! Event helpers that depend on nothing but the DOM, compiled from the
//! capsule's own files so their decisions are checked on the host.

#[cfg(test)]
#[path = "../../../../capsule_browser/src/browser/event/field_at.rs"]
mod field_at;
#[cfg(test)]
mod field_at_tests;
