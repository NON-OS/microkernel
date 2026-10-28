// NONOS Operating System (AGPL-3.0-or-later)

//! A WPA3 transition-mode access point, as many home routers now run: one
//! network offering WPA2-PSK and SAE (AKMs 00-0F-AC:2 and :8), management
//! frame protection capable but not required, SAE by hash-to-element. Its
//! SAE, association and handshake are the RTL8852 proofs' WPA3 access point
//! (`sae_air`), mounted here a second time under this network's own RSNE,
//! which its beacon and message 3 both carry.

#[path = "../../rtl8852_proofs/src/sae_air/ap.rs"]
mod ap;
#[path = "../../rtl8852_proofs/src/sae_air/commit.rs"]
mod commit;
#[path = "../../rtl8852_proofs/src/sae_air/keys.rs"]
mod keys;

pub use ap::SaeAp;

pub const SSID: &[u8] = b"Mixed home";
pub const PASS: &[u8] = b"correct horse battery";
/// RSNE: CCMP-128 group and pairwise; AKMs PSK then SAE; MFPC alone.
pub const RSNE_SAE: [u8; 26] = [
    0x30, 0x18, 1, 0, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f, 0xac, 4, 2, 0, 0, 0x0f, 0xac, 2, 0, 0x0f,
    0xac, 8, 0x80, 0,
];
/// RSNXE: SAE hash-to-element.
pub const RSNXE_H2E: [u8; 3] = [244, 1, 0x20];
