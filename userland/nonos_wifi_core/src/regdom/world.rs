// NONOS Operating System (AGPL-3.0-or-later)

//! What the world domain allows on each channel.

/// How a channel may be used.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Use {
    /// The station may transmit first: probe, then join.
    Active,
    /// Listen only; join an access point only once its beacon was heard.
    Passive,
}

/// What the world domain allows on `channel`; `None` where it allows nothing.
pub fn world(channel: u8) -> Option<Use> {
    match channel {
        1..=11 => Some(Use::Active),
        12 | 13 => Some(Use::Passive),
        _ if five_ghz(channel) => Some(Use::Passive),
        _ => None,
    }
}

/// The 5 GHz channel numbers the world list names: every fourth from 36 to
/// 64 and from 100 to 144, and from 149 to 165.
fn five_ghz(channel: u8) -> bool {
    match channel {
        36..=64 | 100..=144 => channel.is_multiple_of(4),
        149..=165 => channel % 4 == 1,
        _ => false,
    }
}

/// The highest EIRP the world domain allows on `channel`, in dBm.
pub fn max_eirp_dbm(channel: u8) -> Option<u8> {
    world(channel).map(|_| 20)
}
