// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

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
//! access point's group key handshakes are answered here, the new key handed
//! to the serve loop to install in the CAM. Both directions are checked against
//! the real `LinkPort`/`KeyStore` traits in `rtl8821ce_proofs`.

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
#[path = "link/keys.rs"]
mod keys;
#[path = "link/keystore.rs"]
mod keystore;
#[path = "link/leave.rs"]
mod leave;
#[path = "link/netif.rs"]
mod netif;
#[path = "link/new.rs"]
mod new;
#[path = "link/radio.rs"]
mod radio;
#[path = "link/receive.rs"]
mod receive;
#[path = "link/send.rs"]
mod send;
#[path = "link/session.rs"]
mod session;
#[path = "link/state.rs"]
mod state;
#[path = "link/transmit.rs"]
mod transmit;

pub use buffers::RxBuffers;
pub use counts::LinkStats;
pub use dead::DeadLink;
pub use keys::RtlKeys;
pub use state::RtlLink;
