// NONOS Operating System (AGPL-3.0-or-later)
//! The driver sources the 802.11 and gen3 proofs run.

// The 802.11 frame layer: pure IEEE encoding, no hardware, so it is checked
// exactly rather than modeled. `src/dot11/mod.rs` pulls in the real files.
pub mod dot11;
// The gen3 (AX210-class) firmware self-load: the context-information structure,
// the peripheral-scratch control block, the image parser, and the boot
// registers. Pure layout and parsing, checked byte-exact against the real
// so-a0-gf-a0 image and the documented struct offsets.
#[path = "../../capsule_driver_iwlwifi/src/firmware/gen3/mod.rs"]
pub mod gen3;
// How often the serving loop restarts a firmware that raised its error cause.
#[path = "../../capsule_driver_iwlwifi/src/server/radio/restarts.rs"]
pub mod restarts;
