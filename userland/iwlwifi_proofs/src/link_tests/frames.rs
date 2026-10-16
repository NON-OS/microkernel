// NONOS Operating System (AGPL-3.0-or-later)

//! The addresses and the Ethernet frame both directions carry.

pub(super) const OUR: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x02];
pub(super) const BSSID: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];
pub(super) const RATE_N_FLAGS: u32 = 0x0000_0003;

// An example RX descriptor size; the running firmware reports the real one.
pub(super) const RX_DESC_LEN: usize = 32;
pub(super) const LLC_SNAP: [u8; 6] = [0xAA, 0xAA, 0x03, 0x00, 0x00, 0x00];
pub(super) const FC_PROTECTED: u16 = 0x4000;

pub(super) fn ethernet() -> Vec<u8> {
    let mut f = Vec::new();
    f.extend_from_slice(&[0x33, 0x33, 0x00, 0x00, 0x00, 0x16]); // destination
    f.extend_from_slice(&OUR); // source
    f.extend_from_slice(&[0x08, 0x00]); // ethertype IPv4
    f.extend_from_slice(b"the ip stack rides the radio");
    f
}
