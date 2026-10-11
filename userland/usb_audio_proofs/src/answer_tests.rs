// NONOS Operating System (AGPL-3.0-or-later)
//! Status, start, stop and writes, as the audio server sees them.

use crate::ask::{ask, bound, ctl, write};
use crate::peers::sink;
use crate::serve::play::Play;
use crate::uac::Outputs;
use crate::wire::{E_AGAIN, E_NODEV, E_OK, OUTPUT_NO_DEVICE};

#[test]
fn without_a_device_the_stream_is_refused_and_status_says_so() {
    let mut play = Play::new();
    let r = ask(&mut play, &ctl(sink::output_status_request, 3));
    assert_eq!(sink::reply_status(&r), E_OK);
    assert_eq!(r.len(), sink::OUTPUT_REPLY_LEN);
    assert_eq!(sink::read_output_status(&r), Some((OUTPUT_NO_DEVICE, 0)));
    assert_eq!(sink::reply_status(&ask(&mut play, &ctl(sink::start_request, 4))), E_NODEV);
    assert_eq!(sink::reply_status(&ask(&mut play, &write(&[0; 64]))), E_NODEV);
    assert!(!play.running);
}

#[test]
fn a_bound_device_is_ready_with_its_outputs() {
    let worn = Outputs { headphone: true, ..Outputs::default() };
    let r = ask(&mut bound(worn), &ctl(sink::output_status_request, 3));
    assert_eq!(sink::read_output_status(&r), Some((0, 1 << 1 | 1 << 8)));
    assert_eq!(&r[28..32], &[0xf4, 0x46, 0x02, 0x00]);
    let both = Outputs { speaker: true, line: true, ..Outputs::default() };
    let r = ask(&mut bound(both), &ctl(sink::output_status_request, 3));
    assert_eq!(sink::read_output_status(&r), Some((0, 1 | 1 << 2)));
}

#[test]
fn writes_queue_until_full_then_ask_again() {
    let mut play = bound(Outputs::default());
    assert_eq!(sink::reply_status(&ask(&mut play, &ctl(sink::start_request, 1))), E_OK);
    assert!(play.running);
    let piece = write(&[1u8; 4096]);
    for _ in 0..4 {
        assert_eq!(sink::reply_status(&ask(&mut play, &piece)), E_OK);
    }
    assert_eq!(sink::reply_status(&ask(&mut play, &piece)), E_AGAIN);
    assert_eq!(sink::reply_status(&ask(&mut play, &ctl(sink::stop_request, 2))), E_OK);
    assert!(!play.running);
    // The stop dropped what had not played.
    assert_eq!(sink::reply_status(&ask(&mut play, &piece)), E_OK);
}
