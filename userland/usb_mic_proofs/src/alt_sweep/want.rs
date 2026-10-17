// NONOS Operating System (AGPL-3.0-or-later)
//! Whether an alternate as stated plays or captures (build.rs), and the
//! stream it gives.

use super::build::Spec;
use crate::uac::Stream;

/// The stream a matching spec on alternate `alt` gives.
pub fn stream(s: &Spec, alt: u8, play: bool) -> Stream {
    let (a, attrs, max_packet, b_interval) = if play { s.out.unwrap() } else { s.inp.unwrap() };
    Stream {
        interface: 1,
        alt,
        ep_address: a,
        sync: attrs >> 2 & 3,
        max_packet,
        b_interval,
        rate_control: s.version == 1 && s.rate_control,
        channels: s.channels,
        version: s.version,
        terminal: s.link,
    }
}

pub fn matches(s: &Spec, play: bool) -> bool {
    let rated = s.version == 2
        || s.rates.contains(&48_000)
        || (s.rates.is_empty() && s.range.0 <= 48_000 && 48_000 <= s.range.1);
    let isoch = |e: &Option<(u8, u8, u16, u8)>| e.is_some_and(|e| e.1 & 3 == 1);
    let ep = match play {
        true => isoch(&s.out),
        false => !isoch(&s.out) && isoch(&s.inp) && s.inp.unwrap().1 >> 4 & 3 != 1,
    };
    let channels = s.channels == 2 || (!play && s.channels == 1);
    let pcm = s.pcm && (s.version == 1 || s.fmt_type == 1);
    !s.broken && pcm && s.subframe == 2 && s.bits == 16 && rated && channels && ep
}
