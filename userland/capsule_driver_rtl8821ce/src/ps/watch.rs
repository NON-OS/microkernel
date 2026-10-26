// NONOS Operating System (AGPL-3.0-or-later)

//! When the link may sleep: rtw88's watchdog, every two seconds, sleeps a link
//! whose traffic stayed under its threshold (`rtw_watch_dog_work`,
//! RTW_LPS_THRESHOLD), and not one whose access point went quiet (its
//! `beacon_loss`): here, any frame the station's own watch sent.

/// rtw88's watchdog period (RTW_WATCH_DOG_DELAY_TIME).
pub const PERIOD_MS: u64 = 2_000;
/// Frames either way in a period that keep the link awake (RTW_LPS_THRESHOLD).
pub const BUSY_FRAMES: u32 = 50;

/// Running totals of what the link has done.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Seen {
    pub tx: u32,
    pub rx: u32,
    /// Probes and SA Queries the station's watch sent.
    pub sends: u32,
}

#[derive(Clone, Copy, Default)]
pub struct Watch {
    next_ms: Option<u64>,
    last: Seen,
}

impl Watch {
    /// At the end of each period: whether the link may sleep through the next
    /// one. `may_sleep` is the station's own say (associated, nothing in
    /// flight). Between periods, and at the first call, nothing.
    pub fn due(&mut self, now_ms: u64, seen: Seen, may_sleep: bool) -> Option<bool> {
        let restart = Self { next_ms: Some(now_ms.saturating_add(PERIOD_MS)), last: seen };
        let Some(next) = self.next_ms else {
            *self = restart;
            return None;
        };
        if now_ms < next {
            return None;
        }
        let quiet = seen.tx.wrapping_sub(self.last.tx) <= BUSY_FRAMES
            && seen.rx.wrapping_sub(self.last.rx) <= BUSY_FRAMES
            && seen.sends == self.last.sends;
        *self = restart;
        Some(may_sleep && quiet)
    }
}
