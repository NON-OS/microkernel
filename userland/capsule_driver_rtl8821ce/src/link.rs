// NONOS Operating System (AGPL-3.0-or-later)

//! Bind the RTL8821CE to the shared WiFi core. The WPA2 supplicant in
//! `nonos_wifi_core` derives the pairwise and group keys and installs them
//! through the `KeyStore` seam; on this chip that means writing the hardware
//! security CAM, after which the radio does CCMP itself. Group keys take the
//! reserved CAM entries 0..3 by their key index; pairwise keys take an entry
//! from 4 up, one per peer.
//!
//! The `LinkPort` (`RtlLink`) is the Ethernet data path net_core drives. It holds
//! a shared `LinkStation` in `Ccmp::SoftwareTxHwRx`: transmit frames are CCMP
//! encrypted in software (this chip's transmit encryption did not work), and
//! receive frames are decrypted by the radio from the CAM where it can, by the
//! station otherwise. Every received frame goes through the station's checks
//! (from the BSS, protected, not a replay) before the stack sees it, and the
//! access point's group key handshakes are answered here, the new key handed to
//! the serve loop for the CAM; both directions are checked in `rtl8821ce_proofs`.

#[path = "link/address.rs"]
mod address;
#[path = "link/buffers.rs"]
mod buffers;
#[path = "link/counts.rs"]
mod counts;
#[path = "link/dead.rs"]
mod dead;
#[path = "link/ends.rs"]
mod ends;
#[path = "link/frames.rs"]
mod frames;
#[path = "link/gone.rs"]
mod gone;
#[path = "link/held.rs"]
mod held;
#[path = "link/history.rs"]
mod history;
#[path = "link/keys.rs"]
mod keys;
#[path = "link/keystore.rs"]
mod keystore;
#[path = "link/netif.rs"]
mod netif;
#[path = "link/new.rs"]
mod new;
#[path = "link/radio.rs"]
mod radio;
#[path = "link/receive.rs"]
mod receive;
#[path = "link/rings.rs"]
mod rings;
#[path = "link/send.rs"]
mod send;
#[path = "link/session.rs"]
mod session;
#[path = "link/sleep.rs"]
mod sleep;
#[path = "link/stall.rs"]
mod stall;
#[path = "link/state.rs"]
mod state;
#[path = "link/transmit.rs"]
mod transmit;
#[path = "link/watch.rs"]
mod watch;

pub use address::NoAddress;
pub use buffers::RxBuffers;
pub use counts::LinkStats;
pub use dead::DeadLink;
pub use keys::RtlKeys;
pub use rings::engines_on;
pub use state::RtlLink;
