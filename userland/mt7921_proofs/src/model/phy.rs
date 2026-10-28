// NONOS Operating System (AGPL-3.0-or-later)

//! The air's physical layer, as far as the driver's rates reach it. Every
//! frame goes at the TXD's fixed rate (`MT_TXD6_TX_RATE`, its mode at bits
//! 9:6, CCK being 0), and an access point hears a frame only at a rate it
//! decodes: no 5 GHz radio decodes CCK, and an access point with 802.11b
//! turned off ("g-only", which many routers offer and enterprise networks
//! set) decodes no CCK either. Such an access point says so in its beacons
//! and probe responses: its rates are OFDM alone, 6, 12 and 24 basic.

pub const G_ONLY: [u8; 8] = [0x8c, 0x12, 0x98, 0x24, 0xb0, 0x48, 0x60, 0x6c];

/// Whether a frame sent with TXD word 6 `txd6` is heard on `tuned`.
pub fn heard(txd6: u32, tuned: Option<u8>, g_only: bool) -> bool {
    let cck = txd6 >> 16 >> 6 & 0xf == 0;
    !(cck && (g_only || tuned.is_some_and(|ch| ch >= 36)))
}

/// A beacon or probe response with its Supported Rates made g-only; any
/// other frame as it was.
pub fn g_only(mut f: Vec<u8>) -> Vec<u8> {
    if f.first().is_some_and(|&fc| fc == 0x80 || fc == 0x50) {
        let mut at = 36;
        while at + 2 <= f.len() {
            let (id, len) = (f[at], f[at + 1] as usize);
            if id == 1 && len == 8 && at + 10 <= f.len() {
                f[at + 2..at + 10].copy_from_slice(&G_ONLY);
                break;
            }
            at += 2 + len;
        }
    }
    f
}
