// NONOS Operating System (AGPL-3.0-or-later)

//! The receive half of an associated link.
//!
//! CCMP is checked against IEEE Std 802.11-2012 M.6.4 (the CCMP test vector:
//! input as in hostapd wlantest/test_vectors.c test_vector_ccmp(), fetched from
//! https://w1.fi/cgit/hostap/plain/wlantest/test_vectors.c, sha256
//! 9aceafbe03d32ca537dab030de0350c5d9cc658de176bc21c76ab95b7e30a633; the
//! expected encrypted MPDU is the published one). The QoS vector was computed
//! independently with the Python `cryptography` AES-CCM and hostapd's
//! ccmp_aad_nonce() rules (TID in the nonce, QoS Control with only the TID in
//! the AAD), since the Annex vector is non-QoS and access points send QoS data.
//!
//! Then `LinkStation::receive`: frames from the BSS are delivered, everything
//! else (another BSS, ToDS, another station, fragments, A-MSDUs, unprotected
//! data on a protected link, replays, the station's own broadcasts coming
//! back) is dropped with its reason, a chip-decrypted frame loses its CCMP
//! header and MIC, and the AP's group key handshake is answered and its key
//! taken.

mod data;
mod forged_rekey;
mod frames;
mod fuzz;
mod keys;
mod rekey;
mod stations;
mod vectors;
