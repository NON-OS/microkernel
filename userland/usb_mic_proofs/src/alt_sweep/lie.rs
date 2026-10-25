// NONOS Operating System (AGPL-3.0-or-later)
//! A function of one to four streaming alternates (after alternate 0,
//! which streams nothing), one of them told a lie that makes it no stream:
//! a descriptor cut short, a rate count past what is held, another
//! protocol, a bulk endpoint, or a capture endpoint that is feedback. The
//! stream played and the stream captured are then the first alternate of
//! the others that plays or captures as stated.

use super::build::{spec, Spec};
use super::encode::alt;
pub use super::lies::{Cut, Lie, LIES};
use super::want::{matches, stream};
use crate::capture_sweep::rng::Rng;
use crate::devices::{config, iface, output_terminal};
use crate::uac::Stream;

pub fn tell(r: &mut Rng) -> (Lie, Vec<u8>, [Option<Stream>; 2]) {
    let mut lie = LIES[r.range(0, LIES.len() - 1)];
    let version = r.range(1, 2) as u8;
    let mut specs: Vec<Spec> = (0..r.range(1, 4)).map(|_| spec(r, version)).collect();
    let any = |s: &Spec| matches(s, true) || matches(s, false);
    let interface = lie == Lie::Bytes(Cut::Interface);
    // A cut interface descriptor folds its alternate into the one before;
    // told in the first, that is alternate 0, which has none of its own.
    let k = match interface {
        true => 0,
        false => specs.iter().position(any).unwrap_or(r.range(0, specs.len() - 1)),
    };
    if lie == Lie::Bytes(Cut::RatesPast) && version == 2 {
        lie = Lie::Bytes(Cut::Format);
    }
    let s = &mut specs[k];
    match lie {
        Lie::Bytes(Cut::Interface) => {}
        Lie::Bytes(Cut::Endpoint) => {}
        Lie::Bytes(_) => s.broken = true,
        Lie::Bulk => match (&mut s.out, &mut s.inp) {
            (Some(e), _) | (None, Some(e)) => e.1 = e.1 & !3 | 2,
            _ => {}
        },
        Lie::Feedback if s.out.is_none() => s.inp.as_mut().unwrap().1 |= 0x10,
        Lie::Feedback => lie = Lie::Sound,
        Lie::Sound => {}
    }
    let proto = if version == 1 { 0 } else { 0x20 };
    let (ac, term) = (iface(0, 0, 1, proto), output_terminal(0x0301));
    let off = vec![9, 0x04, 1, 0, 0, 0x01, 0x02, proto, 0];
    let n = r.range(0, 255);
    let alts: Vec<Vec<u8>> = specs
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let cut = match lie {
                Lie::Bytes(c) if i == k => Some((c, n)),
                _ => None,
            };
            alt(s, i as u8 + 1, cut)
        })
        .collect();
    let mut parts: Vec<&[u8]> = vec![&ac, &term, &off];
    parts.extend(alts.iter().map(|a| a.as_slice()));
    // A cut endpoint is no endpoint: the OUT one if there is one.
    let mut seen = specs.clone();
    if lie == Lie::Bytes(Cut::Endpoint) {
        let s = &mut seen[k];
        if s.out.take().is_none() {
            s.inp = None;
        }
    }
    let want = |play: bool| {
        let i = seen.iter().position(|s| matches(s, play))?;
        Some(stream(&seen[i], if interface && i == 0 { 0 } else { i as u8 + 1 }, play))
    };
    (lie, config(&parts), [want(true), want(false)])
}
