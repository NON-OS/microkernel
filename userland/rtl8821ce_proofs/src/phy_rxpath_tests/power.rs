// NONOS Operating System (AGPL-3.0-or-later)

//! The transmit power set_channel programs.

use super::model::Dev;
use crate::phy::channel::Bw;
use crate::phy::rxpath::set_channel;

#[test]
fn set_channel_programs_a_nonzero_transmit_power() {
    let dev = Dev::new();
    set_channel(&dev, 1, Bw::W20, false);
    // Every rate register carries the fixed power index in all four bytes, so
    // frames leave the antenna with real power instead of the zero default.
    for group in 0..5usize {
        let word = dev.get(0x1D00 + group * 4);
        assert_ne!(word, 0, "rate group {group} has transmit power");
        assert_eq!(word & 0xFF, 0x28, "power index in the low byte");
        assert_eq!(word, 0x2828_2828, "the index is packed across all four rates");
    }
}
