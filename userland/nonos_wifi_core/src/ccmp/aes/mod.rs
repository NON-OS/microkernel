// NONOS Operating System (AGPL-3.0-or-later)

//! AES-128 block encryption (FIPS-197). WPA2 CCMP encrypts and authenticates
//! data frames with AES-128 in CCM mode, which uses the forward cipher only.
//! no_std, no allocation, and constant time: no table is indexed by a byte of
//! key or state and no branch turns on one (the S-box is computed, `sbox`),
//! and the expanded key is wiped on drop. Checked against the FIPS-197
//! known-answer vectors in `nonos_wifi_core_proofs`.

mod cipher;
mod forward;
mod inverse;
mod planes;
mod sbox;
#[cfg(test)]
mod sbox_tests;
mod schedule;

pub use cipher::Aes128;
