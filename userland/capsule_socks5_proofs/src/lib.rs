// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the SOCKS5 proxy. Includes the real capsule source via
//! `#[path]` and drives it against hand-built bytes.

// The shared route client's sources reach the heap through `alloc` at their
// root, as they do in their own crate.
extern crate alloc;

#[path = "../../capsule_socks5/src/nym/batch.rs"]
pub mod batch;
#[path = "../../capsule_socks5/src/nym/choose.rs"]
pub mod choose;
#[path = "../../capsule_socks5/src/server/clients.rs"]
pub mod clients;
#[path = "../../capsule_socks5/src/conn/mod.rs"]
pub mod conn;
#[path = "../../capsule_socks5/src/server/gather.rs"]
pub mod gather;
#[path = "../../capsule_socks5/src/server/inbox.rs"]
pub mod inbox;
#[path = "../../capsule_socks5/src/server/kept.rs"]
pub mod kept;
#[path = "../../capsule_socks5/src/manager/mod.rs"]
pub mod manager;
#[path = "../../capsule_socks5/src/server/parked.rs"]
pub mod parked;
#[path = "../../capsule_socks5/src/server/reply.rs"]
pub mod reply;
#[path = "../../capsule_socks5/src/server/who.rs"]
pub mod who;
#[path = "../../capsule_socks5/src/server/request.rs"]
pub mod request;
#[path = "../../capsule_socks5/src/tunnel/mod.rs"]
pub mod tunnel;
#[path = "../../capsule_socks5/src/nym/watch.rs"]
pub mod watch;

/// The terminal's side of the conversation with this proxy, which is the
/// shared client in nonos_route_link: the frames it sends, the answers it
/// reads, and the rule for which network it leaves by at all.
#[path = "../../nonos_route_link/src/frame.rs"]
pub mod route_frame;
#[path = "../../nonos_route_link/src/answer.rs"]
pub mod route_answer;
#[path = "../../nonos_route_link/src/pick.rs"]
pub mod route_pick;
#[path = "../../capsule_socks5/src/wire/mod.rs"]
pub mod wire;

#[cfg(test)]
mod batch_tests;
#[cfg(test)]
mod choose_tests;
#[cfg(test)]
mod conn_tests;
#[cfg(test)]
mod ended_tests;
#[cfg(test)]
mod gather_tests;
#[cfg(test)]
mod inbox_tests;
#[cfg(test)]
mod kept_harness;
#[cfg(test)]
mod kept_tests;
#[cfg(test)]
mod manager_tests;
#[cfg(test)]
mod parked_tests;
#[cfg(test)]
mod reply_tests;
#[cfg(test)]
mod request_fuzz_tests;
#[cfg(test)]
mod stream_tests;
#[cfg(test)]
mod terminal_tests;
#[cfg(test)]
mod tunnel_tests;
#[cfg(test)]
mod watch_tests;
#[cfg(test)]
mod wire_tests;
