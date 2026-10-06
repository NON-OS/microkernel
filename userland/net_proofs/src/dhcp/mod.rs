// NONOS Operating System (AGPL-3.0-or-later)
#[path = "../../../capsule_net_dhcp/src/dhcp/constants.rs"]
pub mod constants;
#[path = "../../../capsule_net_dhcp/src/dhcp/message.rs"]
pub mod message;
// The real parse module nests a `parse` submodule (module_inception).
#[allow(clippy::module_inception)]
#[path = "../../../capsule_net_dhcp/src/dhcp/parse/mod.rs"]
pub mod parse;
pub use message::Message;
pub use parse::parse;
#[path = "../../../capsule_net_dhcp/src/dhcp/build.rs"]
pub mod build;
pub use build::build_request;
pub use constants::{CLIENT_PORT, SERVER_PORT};
