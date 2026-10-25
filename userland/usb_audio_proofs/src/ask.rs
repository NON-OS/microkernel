// NONOS Operating System (AGPL-3.0-or-later)
//! Requests the audio server's own sink client builds, answered by the
//! driver; the tests read the replies back with the sink's own readers.

use crate::peers::sink;
use crate::serve::answer::answer;
use crate::serve::play::{Device, Play};
use crate::serve::reply::REPLY_MAX;
use crate::uac::Outputs;

pub fn ask(play: &mut Play, msg: &[u8]) -> Vec<u8> {
    let mut tx = [0u8; REPLY_MAX];
    let n = answer(play, msg, &mut tx);
    tx[..n].to_vec()
}

pub fn ctl(build: fn(u32, &mut [u8]) -> usize, id: u32) -> Vec<u8> {
    let mut tx = [0u8; sink::HDR_LEN];
    let n = build(id, &mut tx);
    tx[..n].to_vec()
}

pub fn write(pcm: &[u8]) -> Vec<u8> {
    let mut tx = vec![0u8; sink::HDR_LEN + pcm.len()];
    let n = sink::request(7, pcm, &mut tx);
    tx[..n].to_vec()
}

pub fn bound(outputs: Outputs) -> Play {
    let mut play = Play::new();
    play.device = Some(Device { ids: (0x46f4, 0x0002), outputs });
    play
}
