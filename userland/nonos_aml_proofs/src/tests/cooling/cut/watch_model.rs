// NONOS Operating System (AGPL-3.0-or-later)
//! The watch's model, from the rulings and not from the code: the history
//! of readings, each a time and a temperature or nothing, judged afresh.

/// _CRT, _HOT, _PSV and _AC0 of the watch proofs' zone, in m°C.
pub const CRT: i32 = 100_050;
pub const HOT: i32 = 90_050;
pub const PSV: i32 = 80_050;
pub const AC0: i32 = 70_050;

/// The last temperature believed: the last reading that was one.
pub fn temp(h: &[(u64, Option<i32>)]) -> Option<i32> {
    h.iter().rev().find_map(|r| r.1)
}

/// Critical: some real reading at or past _CRT, and a later one a second
/// or more after it, with no real reading below _CRT from it to now.
pub fn critical(h: &[(u64, Option<i32>)]) -> bool {
    let real: Vec<(u64, i32)> = h.iter().filter_map(|(t, r)| r.map(|r| (*t, r))).collect();
    let since = real.iter().rposition(|r| r.1 < CRT).map_or(0, |i| i + 1);
    let run = &real[since..];
    run.first().is_some_and(|f| run.iter().any(|r| r.0 >= f.0 + 1_000))
}

/// Waiting on a second reading: the last real one is at or past _CRT and
/// critical is not settled.
pub fn pending(h: &[(u64, Option<i32>)]) -> bool {
    temp(h).is_some_and(|t| t >= CRT) && !critical(h)
}
