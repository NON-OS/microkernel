// NONOS Operating System (AGPL-3.0-or-later)

//! The channel request and receive filter bodies against Linux v6.12's
//! `mt7921_roc_acquire_tlv`/`mt7921_roc_abort_tlv` and
//! `mt7921_mcu_set_rxfilter`.

use crate::vif::{filter, roc};

#[test]
fn a_channel_request_and_its_abort() {
    let a = roc::acquire(7, 149, true, roc::Req::Join, 1000);
    assert_eq!(&a[4..8], &[0, 0, 24, 0], "UNI_ROC_ACQUIRE, 24 bytes");
    assert_eq!(&a[8..20], &[0, 7, 149, 0, 2, 0, 149, 0, 0, 149, 0, 0], "bss, token, chan, band 5");
    assert_eq!((u32::from_le_bytes(a[20..24].try_into().unwrap()), a[24]), (1000, 0xff));
    assert_eq!(
        (
            roc::acquire(1, 6, false, roc::Req::Listen, 500)[12],
            roc::acquire(1, 6, false, roc::Req::Listen, 500)[19]
        ),
        (1, 1)
    );
    let x = roc::abort(7);
    assert_eq!(&x[4..11], &[1, 0, 12, 0, 0, 7, 0xff]);
}

#[test]
fn the_filter_is_set_whole_then_other_networks_beacons_let_through() {
    let d = filter::default();
    assert_eq!((d[4], u32::from_le_bytes(d[8..12].try_into().unwrap())), (1, 1 << 31));
    let h = filter::hear_beacons();
    assert_eq!((h[4], u32::from_le_bytes(h[12..16].try_into().unwrap()), h[16]), (2, 1 << 11, 2));
    assert_eq!((d.len(), h.len()), (68, 68));
}
