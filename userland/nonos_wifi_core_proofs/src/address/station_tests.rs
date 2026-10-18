// NONOS Operating System (AGPL-3.0-or-later)

//! A station takes a new address only between associations: a session keeps
//! the address it began with.

use nonos_wifi_core::station::{Ccmp, LinkStation};

#[test]
fn a_station_takes_a_new_address_only_between_associations() {
    let (bssid, home, work) = ([2, 0, 0, 0, 0, 1], [2, 0, 0, 0, 0, 0xAA], [2, 0, 0, 0, 0, 0xBB]);
    let eth = |src: [u8; 6]| [&[0x33; 6][..], &src, &[0x08, 0x00], b"payload"].concat();
    let mut st = LinkStation::new([2, 0, 0, 0, 0, 2]);
    assert!(st.readdress(home), "unassociated: taken");
    st.associate(bssid, Ccmp::Hardware);
    // The chip holds the keys, so a hardware station frames at all (`chip_keys`).
    st.chip_keys(true);
    assert_eq!(&st.tx_frame(&eth(home)).expect("framed")[10..16], &home, "sent from it");
    assert!(!st.readdress(work), "associated: refused");
    assert_eq!(&st.tx_frame(&eth(home)).expect("framed")[10..16], &home, "unchanged");
    st.deassociate();
    assert!(st.readdress(work));
    st.associate(bssid, Ccmp::Hardware);
    st.chip_keys(true);
    assert_eq!(&st.tx_frame(&eth(work)).expect("framed")[10..16], &work);
}
