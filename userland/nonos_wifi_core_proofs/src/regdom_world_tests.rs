// NONOS Operating System (AGPL-3.0-or-later)
//! The world-safe default over every channel number, and the channel a join
//! takes from a beacon anyone could have forged.

use nonos_wifi_core::regdom::{join_channel, max_eirp_dbm, world, Use};

#[test]
fn the_world_domain_over_every_channel_number() {
    let five: Vec<u8> =
        (36..=64).step_by(4).chain((100..=144).step_by(4)).chain((149..=165).step_by(4)).collect();
    for ch in 0..=255u8 {
        let want = match ch {
            1..=11 => Some(Use::Active),
            12 | 13 => Some(Use::Passive),
            _ if five.contains(&ch) => Some(Use::Passive),
            _ => None,
        };
        assert_eq!(world(ch), want, "channel {ch}");
        assert_eq!(max_eirp_dbm(ch), want.map(|_| 20), "channel {ch}");
    }
    assert_eq!(world(14), None, "channel 14 is never used");
    assert_eq!((world(169), world(37), world(0)), (None, None, None));
}

#[test]
fn a_join_takes_the_named_channel_only_where_it_could_be_true() {
    // Named and heard agree, or overlap on 2.4 GHz: the named one.
    assert_eq!(join_channel(6, 6), Some(6));
    assert_eq!(join_channel(5, 6), Some(6), "heard on the neighbour");
    assert_eq!(join_channel(11, 13), Some(13), "a passive channel, beacon heard");
    // Forged or missing: the channel the radio was on.
    for named in [0, 14, 15, 200, 255] {
        assert_eq!(join_channel(6, named), Some(6), "named {named}");
    }
    assert_eq!(join_channel(1, 11), Some(1), "too far to be heard on 1");
    assert_eq!(join_channel(36, 40), Some(36), "5 GHz channels do not overlap");
    assert_eq!(join_channel(6, 36), Some(6), "another band");
    // Heard where the world domain allows nothing: no join.
    for heard in [0, 14, 169, 37] {
        assert_eq!(join_channel(heard, 6), None, "heard on {heard}");
    }
}
