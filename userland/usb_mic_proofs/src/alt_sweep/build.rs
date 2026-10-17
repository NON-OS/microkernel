// NONOS Operating System (AGPL-3.0-or-later)
//! One streaming alternate as stated (USB Audio 1.0 4.5 and Format 2.2;
//! 2.0 4.9, as recalled, unverified), and whether it plays or captures
//! 48 kHz 16-bit PCM: PCM in AS_GENERAL, Format Type I of two-byte
//! subframes and 16 bits, for 1.0 a listed 48 kHz or a range holding it;
//! two channels played, one or two captured; played through an isoch OUT
//! endpoint, captured through an isoch IN that is no feedback endpoint
//! and with no OUT beside it (want.rs).

use crate::capture_sweep::rng::Rng;

#[derive(Clone, Debug)]
pub struct Spec {
    pub version: u8,
    pub pcm: bool,
    pub channels: u8,
    pub bits: u8,
    pub subframe: u8,
    /// 2.0 AS_GENERAL's bFormatType; a class endpoint's bit 0, which only
    /// 1.0 reads; Format Type I ahead of AS_GENERAL.
    pub fmt_type: u8,
    pub cs_bit: bool,
    pub swap: bool,
    /// 1.0's rates, listed; empty for the range `range`.
    pub rates: Vec<u32>,
    pub range: (u32, u32),
    pub link: u8,
    /// (address, bmAttributes, wMaxPacketSize, bInterval)
    pub out: Option<(u8, u8, u16, u8)>,
    pub inp: Option<(u8, u8, u16, u8)>,
    pub rate_control: bool,
    /// A lie the spec cannot say (a descriptor cut, another protocol):
    /// the alternate is no stream.
    pub broken: bool,
}

pub fn spec(r: &mut Rng, version: u8) -> Spec {
    let play = r.range(0, 1) == 0;
    let ep = |r: &mut Rng, a: u8| {
        (a, [0x05, 0x09, 0x0D, 0x01][r.range(0, 3)], r.range(8, 1024) as u16, r.range(1, 4) as u8)
    };
    let (out, inp) = match (play, r.range(0, 2)) {
        (true, 0) => (Some(ep(r, 0x01)), Some((0x81, 0x11, 4, 4))),
        (true, 1) => (Some(ep(r, 0x01)), Some(ep(r, 0x82))),
        (true, _) => (Some(ep(r, 0x02)), None),
        (false, _) => (None, Some(ep(r, 0x83))),
    };
    let rates = [vec![48_000], vec![44_100, 48_000], vec![], vec![44_100]][r.range(0, 3)].clone();
    Spec {
        version,
        pcm: r.range(0, 5) != 0,
        channels: [2, 2, 1, 6][r.range(0, 3)],
        bits: [16, 16, 24][r.range(0, 2)],
        subframe: [2, 2, 2, 3, 4][r.range(0, 4)],
        fmt_type: [1, 1, 1, 2][r.range(0, 3)],
        cs_bit: r.range(0, 1) == 0,
        swap: r.range(0, 3) == 0,
        rates,
        range: [(8_000, 96_000), (44_100, 44_100)][r.range(0, 1)],
        link: r.range(1, 9) as u8,
        out,
        inp,
        rate_control: r.range(0, 1) == 0,
        broken: false,
    }
}
