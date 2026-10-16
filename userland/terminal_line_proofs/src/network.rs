// NONOS Operating System (AGPL-3.0-or-later)
//! The network commands' pure parts: the direct gate, offline words, job waits.

// The commands that can only leave directly, and the rule that lets them:
// nonos_route_link's, the one every capsule holding Network goes by.
#[path = "../../capsule_terminal/src/command/builtin/direct_gate.rs"]
pub mod direct_gate;
#[path = "../../nonos_route_link/src/direct_only.rs"]
pub mod direct_only;
// What those commands say, before anything is sent, when the machine has
// no address: no cable and no Wi-Fi joined.
#[path = "../../capsule_terminal/src/command/builtin/offline_gate.rs"]
pub mod offline_gate;

// What a network job makes of the far end's silence each tick, and the one
// request a stepped `git clone` holds for its job to carry.
#[path = "../../capsule_terminal/src/mixnet/exchange/wait.rs"]
pub mod exchange_wait;
#[path = "../../capsule_terminal/src/git/transport/ask.rs"]
pub mod git_ask;

#[cfg(test)]
mod direct_gate_tests;
#[cfg(test)]
mod network_job_tests;
#[cfg(test)]
mod offline_gate_tests;
