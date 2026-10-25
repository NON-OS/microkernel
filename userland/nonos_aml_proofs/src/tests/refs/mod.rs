// NONOS Operating System (AGPL-3.0-or-later)
//! F-124: what the interpreter keeps when a reference is stored, and that
//! no path through what it keeps reaches a removed object, after an
//! Unload or a method's end, even once the freed slots are reused.
//!
//! What is kept (stated apart; ACPI 6.5, 19.6.16, 19.6.63, 19.6.115 and
//! 19.6.4 as recalled, unverified):
//! - Store or CopyObject of RefOf, Index or a field into a Name keeps the
//!   value read through it, never the reference.
//! - Store of RefOf(name) into a Package element keeps a reference to the
//!   object and its generation: read after the object is gone, it is
//!   stale, whatever now holds the slot.
//! - Store of Index(package, i) into an element keeps the package itself,
//!   detached from the namespace once its name is gone.
//! - A buffer field made at load time finds its buffer by name at its
//!   first use and keeps it from then on, detached the same way.
//! - A field unit finds its region by name at each access.
//! - An alias goes with its object.

mod fixture;
mod sweep_tests;
