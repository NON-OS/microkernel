// NONOS Operating System (AGPL-3.0-or-later)
pub mod packet;
// The real neighbour cache and inbound handler; the cache's style lints are
// its own choices.
#[allow(clippy::new_without_default)]
#[path = "../../../capsule_net_l2/src/arp/cache/mod.rs"]
pub mod cache;
#[path = "../../../capsule_net_l2/src/arp/handle.rs"]
pub mod handle;
pub use cache::Cache;
pub use handle::{on_inbound, Iface};
#[path = "../../../capsule_net_l2/src/arp/sender.rs"]
pub mod sender;
