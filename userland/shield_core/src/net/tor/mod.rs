// NONOS Operating System (AGPL-3.0-or-later)
//! The anonymity network, as the phones' `net::tor` presents it.
//!
//! The phones run an embedded Tor. NONOS has its own networks, chosen once
//! for the whole machine: the Nym mixnet or the Anyone network. This module
//! keeps the phones' interface, `Tor`, `Purpose`, `TorStream` and `tls`, so
//! every caller above it is the phones' code unchanged, and carries each
//! stream over the chosen network through `nonos_route_link`. It never falls
//! back to a direct connection: a chosen network that is down refuses with
//! its reason, and so does Direct, which is not anonymous.
//!
//! What it cannot reach is a `.onion` address: neither Nym nor Anyone serves
//! Tor's onion services. A connection to one is refused at once, and the
//! wallet settles from the owner's own account, as the phones do when no
//! lander answers. An Anyone onion service, `.anyone`, is reached: the route
//! link sends it through net.anon whatever the default network is.

mod network;
mod purpose;
mod route;
mod stream;
mod tls;

pub use network::{as_account, Tor};
pub use purpose::Purpose;
pub use stream::TorStream;
pub use tls::{tls, TlsStream};
