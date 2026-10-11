// NONOS Operating System (AGPL-3.0-or-later)
//! What is refused, and what is answered whatever the device.

use crate::ask::{ask, bound, ctl, write};
use crate::peers::sink;
use crate::serve::play::Play;
use crate::uac::Outputs;
use crate::wire::{E_INVAL, E_OK};

#[test]
fn malformed_requests_are_answered_with_einval() {
    let mut play = bound(Outputs::default());
    let mut short = write(&[0u8; 64]);
    short.truncate(sink::HDR_LEN + 32);
    assert_eq!(sink::reply_status(&ask(&mut play, &short)), E_INVAL);
    let mut start = ctl(sink::start_request, 9);
    start[16] = 4;
    start.extend([0; 4]);
    assert_eq!(sink::reply_status(&ask(&mut play, &start)), E_INVAL);
    assert!(!play.running);
    let mut codec_list = ctl(sink::start_request, 9);
    codec_list[6] = 5;
    assert_eq!(sink::reply_status(&ask(&mut play, &codec_list)), E_INVAL);
    let mut bad_magic = ctl(sink::start_request, 0x0102_0304);
    bad_magic[0] ^= 1;
    let r = ask(&mut play, &bad_magic);
    // Refused under the request's own id, so the caller's wait ends.
    assert_eq!((sink::reply_status(&r), &r[12..16]), (E_INVAL, &[4u8, 3, 2, 1][..]));
    let mut big = vec![0u8; sink::HDR_LEN + 4100];
    big[..sink::HDR_LEN].copy_from_slice(&write(&[0; 4])[..sink::HDR_LEN]);
    big[16..20].copy_from_slice(&4100u32.to_le_bytes());
    assert_eq!(sink::reply_status(&ask(&mut play, &big)), E_INVAL);
}

#[test]
fn healthcheck_answers_ok_with_or_without_a_device() {
    let mut hc = ctl(sink::start_request, 1);
    hc[6] = 1;
    assert_eq!(sink::reply_status(&ask(&mut Play::new(), &hc)), E_OK);
    assert_eq!(sink::reply_status(&ask(&mut bound(Outputs::default()), &hc)), E_OK);
}
