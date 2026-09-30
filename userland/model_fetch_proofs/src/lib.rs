// NONOS Operating System (AGPL-3.0-or-later)
/*
 * The model fetcher's catalogue reader and the pins it checks entries
 * against, compiled from the capsule's own sources.
 */

extern crate alloc;

#[path = "../../capsule_model_fetch/src/pins/mod.rs"]
pub mod pins;

pub mod catalogue;

#[cfg(test)]
mod tests;
