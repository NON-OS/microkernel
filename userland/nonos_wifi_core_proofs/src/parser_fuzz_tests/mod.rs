// NONOS Operating System (AGPL-3.0-or-later)
//! The core's frame parsers fed noise and mutated well-formed frames, fifty
//! thousand rounds each. Nothing may panic, and whatever a parser hands back
//! must lie inside the frame it was given and agree with its own lengths.

mod beacons;
mod eapol;
