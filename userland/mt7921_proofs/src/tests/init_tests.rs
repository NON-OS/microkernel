// NONOS Operating System (AGPL-3.0-or-later)

//! The MAC after the firmware: the capability reply read for its PHY alone
//! (never the factory address), malformed replies leaving the defaults;
//! the channel domain the world domain's; the receive path's channel
//! information; header translation and A-MSDU splitting left off.

use nonos_wifi_core::regdom::{Band, Permit, CHANNELS_2GHZ, CHANNELS_5GHZ, WORLD};

use crate::init::caps::{parse, Caps, DEFAULT};
use crate::init::{chan, domain};
use crate::model::reply::capab;

#[test]
fn the_phy_record_gives_bands_and_streams() {
    let b = capab([1, 2, 3, 4, 5, 6]);
    assert_eq!(parse(&b), Caps { ghz2: true, ghz5: true, antennas: 0x3 });
    let mut one = b.clone();
    let phy = 4 + 14 + 8;
    (one[phy + 4], one[phy + 10]) = (1, 1);
    assert_eq!(parse(&one), Caps { ghz2: true, ghz5: false, antennas: 0x1 });
}

#[test]
fn a_reply_that_lies_about_its_records_leaves_the_defaults() {
    let b = capab([0; 6]);
    let mut long = b.clone();
    long[8] = 200; // the address record claims more than the reply holds
    assert_eq!(parse(&long), DEFAULT);
    assert_eq!(parse(&b[..3]), DEFAULT);
    let mut short_phy = b.clone();
    short_phy[4 + 14 + 4] = 10; // a PHY record too short for hw_path
    assert_eq!(parse(&short_phy[..4 + 14 + 8 + 10]), DEFAULT);
}

#[test]
fn the_channel_domain_is_the_world_domains() {
    let b = domain::body(DEFAULT);
    assert_eq!(&b[..8], &[b'0', b'0', 0, 0, 0, 3, 3, 0]);
    let mut want = Vec::new();
    for (band, chans) in [(Band::Ghz2, &CHANNELS_2GHZ[..]), (Band::Ghz5, &CHANNELS_5GHZ[..])] {
        for &ch in chans {
            let flags = match WORLD.permit(band, ch) {
                Permit::Disabled => continue,
                Permit::Passive { dfs } => 2 | if dfs { 8 } else { 0 },
                Permit::Active => 0,
            };
            want.push((ch, flags));
        }
    }
    let got: Vec<(u8, u32)> =
        b[12..].chunks(8).map(|c| (c[0], u32::from_le_bytes([c[4], c[5], c[6], c[7]]))).collect();
    assert_eq!(got, want);
    assert_eq!(b[8] as usize + b[9] as usize, want.len());
    let only2 = domain::body(Caps { ghz5: false, ..DEFAULT });
    assert_eq!((only2[9], only2.len()), (0, 12 + 8 * only2[8] as usize));
}

#[test]
fn the_receive_paths_channel_is_2_4_ghz_channel_1_at_20_mhz() {
    let b = chan::body(1, 0, DEFAULT);
    assert_eq!(b.len(), 76);
    assert_eq!(&b[..8], &[1, 1, 0, 2, 3, 0, 0, 0], "control, centre, 20 MHz, 2 streams, mask");
    assert_eq!(b[10], 0, "2.4 GHz");
}
