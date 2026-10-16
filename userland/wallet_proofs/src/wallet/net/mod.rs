// NONOS Operating System (AGPL-3.0-or-later)
//! The capsule's net module shape: the route the wallet's requests go over,
//! as the screens name it, and the reader for answers that come through an
//! anonymity network. `Route` is named here as the capsule names it, so the
//! files reach it as `super::Route` unchanged.

pub use crate::pick::Route;

#[path = "../../../../capsule_wallet_nonos/src/wallet/net/bounds.rs"]
pub mod bounds;
#[path = "../../../../capsule_wallet_nonos/src/wallet/net/handshake_end.rs"]
pub mod handshake_end;
#[allow(dead_code)]
#[path = "../../../../capsule_wallet_nonos/src/wallet/net/route_text.rs"]
pub mod route_text;
/* The blocking reader, only here: the model `step::gather` is proved against. */
#[allow(dead_code)]
#[path = "../../../../capsule_wallet_nonos/src/wallet/net/routed_read.rs"]
pub mod routed_read;

#[path = "../../../../capsule_wallet_nonos/src/wallet/net/private_route.rs"]
pub mod private_route;
#[cfg(test)]
#[path = "../../probe_bounds_tests.rs"]
mod probe_bounds_tests;
#[path = "../../../../capsule_wallet_nonos/src/wallet/net/read_snapshot.rs"]
pub mod read_snapshot;
#[cfg(test)]
#[path = "../../../../capsule_wallet_nonos/src/wallet/net/read_tls_flight.rs"]
mod read_tls_flight;
#[path = "../../../../capsule_wallet_nonos/src/wallet/net/snapshot_fields.rs"]
pub mod snapshot_fields;
#[path = "../../../../capsule_wallet_nonos/src/wallet/net/snapshot_parse.rs"]
pub mod snapshot_parse;
pub mod step;
