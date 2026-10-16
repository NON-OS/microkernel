// NONOS Operating System (AGPL-3.0-or-later)

//! The transmit power set_channel programs: within the world domain's limit.

use super::model::Dev;
use crate::phy::channel::Bw;
use crate::phy::rxpath::set_channel;
use crate::phy::txpower::{band_index, power_index};
use nonos_wifi_core::regdom::max_eirp_dbm;

#[test]
fn set_channel_programs_a_nonzero_transmit_power() {
    let dev = Dev::new();
    set_channel(&dev, 1, Bw::W20, false);
    // Every rate register carries the band's power index in all four bytes, so
    // frames leave the antenna with real power instead of the zero default.
    for group in 0..5usize {
        let word = dev.get(0x1D00 + group * 4);
        assert_ne!(word, 0, "rate group {group} has transmit power");
        assert_eq!(word & 0xFF, 0x24, "18 dBm conducted: 20 dBm EIRP less 2 dBi");
        assert_eq!(word, 0x2424_2424, "the index is packed across all four rates");
    }
}

#[test]
fn the_index_stays_within_every_allowed_channels_limit() {
    for ch in 0..=u8::MAX {
        match (power_index(ch), max_eirp_dbm(ch)) {
            (Some(i), Some(dbm)) => assert!(i < dbm * 2 && band_index() <= i, "{ch}"),
            (None, None) => {}
            other => panic!("{ch}: {other:?}"),
        }
    }
    assert_eq!(power_index(14), None, "channel 14 is never used");
    assert_eq!(band_index(), 0x24);
}
