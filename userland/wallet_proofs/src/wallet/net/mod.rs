// NONOS Operating System (AGPL-3.0-or-later)
//! The capsule's net module shape: the route the wallet's requests go over,
//! as the screens name it, and the reader for answers that come through an
//! anonymity network. `Route` is named here as the capsule names it, so the
//! files reach it as `super::Route` unchanged.

pub use crate::pick::Route;

#[allow(dead_code)]
#[path = "../../../../capsule_wallet_nonos/src/wallet/net/route_text.rs"]
pub mod route_text;
#[path = "../../../../capsule_wallet_nonos/src/wallet/net/bounds.rs"]
pub mod bounds;
/* The blocking reader, only here: the model `step::gather` is proved against. */
#[allow(dead_code)]
#[path = "../../../../capsule_wallet_nonos/src/wallet/net/routed_read.rs"]
pub mod routed_read;

pub mod step;
#[path = "../../../../capsule_wallet_nonos/src/wallet/net/read_snapshot.rs"]
pub mod read_snapshot;
#[path = "../../../../capsule_wallet_nonos/src/wallet/net/send_cap.rs"]
pub mod send_cap;
