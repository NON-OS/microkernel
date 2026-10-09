// NONOS Operating System (AGPL-3.0-or-later)
#[path = "../../capsule_net_l2/src/ethernet/types.rs"]
pub mod types;
pub use types::{MacAddress, ETHERTYPE_ARP, MAC_BROADCAST};
#[path = "../../capsule_net_l2/src/ethernet/frame.rs"]
pub mod frame;
pub use frame::{EthHeader, HDR_LEN};
