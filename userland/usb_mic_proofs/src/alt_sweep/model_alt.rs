// NONOS Operating System (AGPL-3.0-or-later)
//! What model.rs knows of an alternate, and the stream it gives: played
//! through its OUT endpoint, or captured through its IN with no OUT, when
//! it streams PCM in Format Type I on two channels (one or two captured).

use crate::uac::Stream;

#[derive(Default)]
pub struct Alt {
    pub s: Stream,
    pub stream: bool,
    pub pcm: bool,
    pub format: bool,
    pub out: Option<(u8, u8, u16, u8)>,
    pub inp: Option<(u8, u8, u16, u8)>,
}

pub fn taken(a: &Alt, play: bool) -> Option<Stream> {
    let ep = if play {
        a.out?
    } else if a.out.is_none() {
        a.inp?
    } else {
        return None;
    };
    let ch = a.s.channels == 2 || (!play && a.s.channels == 1);
    let s =
        Stream { ep_address: ep.0, sync: ep.1 >> 2 & 3, max_packet: ep.2, b_interval: ep.3, ..a.s };
    (a.stream && a.pcm && a.format && ch).then_some(s)
}
