// NONOS Operating System (AGPL-3.0-or-later)
//! A mocked pool and lander, with the follow rules the service applies:
//! a spend published to the lander is waiting until the lander lands it,
//! published again after 15 minutes, offered to its owner after 30, and
//! seen landed once both its nullifiers are on chain, by whoever settled it.

/// A spend the mock follows, minute by minute.
pub struct Spend {
    pub published_at: u32,
    pub last_published: u32,
    pub times: u32,
    /// The minute the lander lands it, if it ever does.
    pub lands_at: Option<u32>,
    /// The minute its owner's own settlement lands, if they sent one.
    pub owner_lands_at: Option<u32>,
    pub settling: bool,
}

pub const REPUBLISH_AFTER: u32 = 15;
pub const SELF_SETTLE_AFTER: u32 = 30;

/// What the follow says at minute `now`: its state, minutes since first
/// published, whether the owner is offered to settle, and the settlement.
pub fn look(s: &mut Spend, now: u32) -> (&'static str, u32, bool, Option<&'static str>) {
    let minutes = now.saturating_sub(s.published_at);
    if s.lands_at.is_some_and(|m| now >= m) {
        return ("landed", minutes, false, Some("0xlander"));
    }
    if s.owner_lands_at.is_some_and(|m| now >= m) {
        return ("spent elsewhere", minutes, false, Some(""));
    }
    if s.settling {
        return ("waiting", minutes, false, None);
    }
    let offered = minutes >= SELF_SETTLE_AFTER;
    if now >= s.last_published + REPUBLISH_AFTER {
        s.last_published = now;
        s.times += 1;
        return ("republished", minutes, offered, None);
    }
    ("waiting", minutes, offered, None)
}
