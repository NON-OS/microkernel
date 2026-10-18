// NONOS Operating System (AGPL-3.0-or-later)
//! A lie told in an alternate's descriptors `d` (interface, AS_GENERAL,
//! Format Type I, endpoints): one cut short with its bLength, or a 1.0
//! rate count past the rates it holds.

use super::build::Spec;
use super::lies::Cut;

fn cut(mut d: Vec<u8>, to: usize) -> Vec<u8> {
    d.truncate(to);
    d[0] = to as u8;
    d
}

pub fn told(d: &mut [Vec<u8>], s: &Spec, lie: Option<(Cut, usize)>) {
    match lie {
        Some((Cut::General, n)) => d[1] = cut(d[1].clone(), 2 + n % [5, 9][s.version as usize - 1]),
        Some((Cut::Format, n)) => d[2] = cut(d[2].clone(), 2 + n % [6, 4][s.version as usize - 1]),
        Some((Cut::RatesPast, n)) if s.version == 1 => {
            let held = (d[2].len() - 8) / 3;
            d[2][7] = (held + 1 + n % 40) as u8;
        }
        Some((Cut::Interface, n)) => d[0] = cut(d[0].clone(), 2 + n % 7),
        Some((Cut::Endpoint, n)) => {
            let at = if s.out.is_some() { 3 } else { 4 };
            d[at] = cut(d[at].clone(), 2 + n % 5);
        }
        _ => {}
    }
}
