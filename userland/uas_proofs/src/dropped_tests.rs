// NONOS Operating System (AGPL-3.0-or-later)
//! IUs for no command in flight: another tag's READ READY, Response or
//! Sense, and any IU after the command ended, let go and counted, never
//! taken for the command's status; a device that sends nothing else is
//! given up on after DROPS_MAX. A Sense IU's length is its own bytes 14-15
//! and must lie within the IU.

use nonos_uas::{status_iu, DataDir, Exchange, StatusIu, Step, DROPS_MAX};

use crate::exchange_tests::Read10;

#[test]
fn another_tags_ius_are_let_go_and_counted() {
    let (mut cmd, mut x) = (Read10::new(), Exchange::new(3, DataDir::In, false));
    let late = StatusIu::Sense { tag: 2, status: 2, sense: &[0x70] };
    assert_eq!(x.status(late, &mut cmd), Step::Status);
    assert_eq!(x.status(StatusIu::ReadReady { tag: 4 }, &mut cmd), Step::Status);
    assert_eq!(x.status(StatusIu::Response { tag: 1, code: 9 }, &mut cmd), Step::Status);
    assert_eq!((x.dropped(), cmd.ended.is_none()), (3, true));
    assert_eq!(x.status(StatusIu::ReadReady { tag: 3 }, &mut cmd), Step::DataIn);
    x.moved();
    let good = StatusIu::Sense { tag: 3, status: 0, sense: &[] };
    assert_eq!(x.status(good, &mut cmd), Step::Done);
    let again = StatusIu::Sense { tag: 3, status: 2, sense: &[0x70] };
    assert_eq!(x.status(again, &mut cmd), Step::Done, "after it ended");
    assert_eq!((x.dropped(), cmd.ended), (4, Some((0, vec![]))));
}

#[test]
fn a_device_sending_only_strangers_is_given_up_on() {
    let (mut cmd, mut x) = (Read10::new(), Exchange::new(3, DataDir::In, false));
    for _ in 0..DROPS_MAX {
        assert_eq!(x.status(StatusIu::ReadReady { tag: 9 }, &mut cmd), Step::Status);
    }
    assert_eq!(x.status(StatusIu::ReadReady { tag: 9 }, &mut cmd), Step::Failed(2));
}

#[test]
fn the_sense_length_is_bytes_14_and_15_and_stays_inside_the_iu() {
    let mut iu = vec![0x03, 0, 0, 5, 0, 0, 0x02, 0, 0, 0, 0, 0, 0xFF, 0xFF, 0, 2, 0x70, 0];
    let want = StatusIu::Sense { tag: 5, status: 2, sense: &[0x70, 0] };
    assert_eq!(status_iu(&iu), Some(want), "reserved bytes 12-13 not read");
    iu[15] = 3;
    assert_eq!(status_iu(&iu), None, "past the IU");
    iu[14] = 1;
    assert_eq!(status_iu(&iu[..16]), None);
}
