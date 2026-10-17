// NONOS Operating System (AGPL-3.0-or-later)

//! AES-128 block encryption (FIPS-197). WPA2 CCMP encrypts and authenticates
//! data frames with AES-128 in CCM mode, which uses the forward cipher only.
//! no_std, no allocation, no lookup-table timing beyond the S-box. Checked
//! against the FIPS-197 known-answer vector in `nonos_wifi_core_proofs`.

mod cipher;
mod forward;
mod inverse;
mod tables;

pub use cipher::Aes128;
