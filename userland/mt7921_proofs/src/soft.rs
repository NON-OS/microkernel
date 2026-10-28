// NONOS Operating System (AGPL-3.0-or-later)

//! The shared Wi-Fi core proofs' access point, mounted by path at the
//! module paths its files name (`crate::soft::ap`, ...): it answers
//! authentication, association and the four-way handshake as a WPA2-PSK
//! network does, and reads the station's protected data.

#[path = "../../nonos_wifi_core_proofs/src/soft/ap.rs"]
pub mod ap;
#[path = "../../nonos_wifi_core_proofs/src/soft/ap_eapol.rs"]
pub mod ap_eapol;
#[path = "../../nonos_wifi_core_proofs/src/soft/ap_rx.rs"]
pub mod ap_rx;
#[path = "../../nonos_wifi_core_proofs/src/soft/chatter.rs"]
pub mod chatter;
#[path = "../../nonos_wifi_core_proofs/src/soft/data_frames.rs"]
pub mod data_frames;
#[path = "../../nonos_wifi_core_proofs/src/soft/frames.rs"]
pub mod frames;
#[path = "../../nonos_wifi_core_proofs/src/soft/protect.rs"]
pub mod protect;
#[path = "../../nonos_wifi_core_proofs/src/soft/quirks.rs"]
pub mod quirks;
