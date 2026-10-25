// NONOS Operating System (AGPL-3.0-or-later)
//! The lies that take the framing away: a bLength that runs past the end
//! of the configuration, or one that swallows what follows it.

use super::rng::Rng;

/// A random descriptor after the header for which `f` holds.
pub fn pick_where(r: &mut Rng, d: &[Vec<u8>], f: impl Fn(&[u8]) -> bool) -> Option<usize> {
    let at: Vec<usize> = (1..d.len()).filter(|&i| f(&d[i])).collect();
    (!at.is_empty()).then(|| r.pick(&at))
}

fn rest(d: &[Vec<u8>], from: usize) -> usize {
    d[from..].iter().map(Vec::len).sum()
}

/// A bLength past the configuration's last byte: the walk stops before it.
pub fn past_end(r: &mut Rng, d: &mut [Vec<u8>]) -> Option<(usize, bool)> {
    let i = pick_where(r, d, |_| true).filter(|&i| rest(d, i) < 255)?;
    d[i][0] = r.range(rest(d, i) + 1, 255) as u8;
    Some((i, true))
}

/// A bLength that stays inside the configuration but runs into the
/// descriptors after it.
pub fn overlong(r: &mut Rng, d: &mut [Vec<u8>]) -> Option<(usize, bool)> {
    let i = r.range(1, d.len() - 1);
    let room = (255 - d[i].len()).min(rest(d, i + 1));
    if room == 0 {
        return None;
    }
    d[i][0] += r.range(1, room) as u8;
    Some((i, false))
}
