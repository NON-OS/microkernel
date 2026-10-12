// NONOS Operating System (AGPL-3.0-or-later)
//! The IUs' bytes: Command and Task Management out, the status pipe's in;
//! and tags.

use crate::exchange_tests::Read10;
use nonos_uas::*;

#[test]
fn a_command_iu_carries_tag_lun_and_cdb() {
    let iu = command_iu(0x0102, 3, &Read10::new()).expect("a Command IU");
    assert_eq!(&iu[0..4], &[0x01, 0, 0x01, 0x02]);
    assert_eq!(&iu[4..8], &[0; 4], "simple, priority 0, no additional CDB");
    assert_eq!(&iu[8..16], &lun_bytes(3));
    assert_eq!(&iu[16..26], &Read10::new().cdb);
    assert_eq!(&iu[26..32], &[0; 6]);
    assert!(command_iu(0, 0, &Read10::new()).is_none(), "tag 0");
}

#[test]
fn a_task_iu_names_the_task_it_manages() {
    let iu = task_iu(9, TMF_ABORT_TASK, 4, 0);
    assert_eq!(&iu[0..8], &[0x05, 0, 0, 9, 0x01, 0, 0, 4]);
    assert_eq!(task_iu(9, TMF_LOGICAL_UNIT_RESET, 0, 1)[4], 0x08);
}

#[test]
fn status_ius_are_read_and_others_are_not() {
    assert_eq!(status_iu(&[0x06, 0, 0, 7]), Some(StatusIu::ReadReady { tag: 7 }));
    assert_eq!(status_iu(&[0x07, 0, 1, 0]), Some(StatusIu::WriteReady { tag: 256 }));
    let mut sense = vec![0x03, 0, 0, 7, 0, 0, 0x02, 0, 0, 0, 0, 0, 0, 3, 0, 0];
    sense.extend([0x70, 0, 0x05]);
    let want = StatusIu::Sense { tag: 7, status: 2, sense: &[0x70, 0, 0x05] };
    assert_eq!(status_iu(&sense), Some(want));
    assert!(status_iu(&sense[..18]).is_none(), "sense cut short");
    assert_eq!(
        status_iu(&[0x04, 0, 0, 7, 0, 0, 0, 0x09]),
        Some(StatusIu::Response { tag: 7, code: 9 })
    );
    assert!(status_iu(&[0x01, 0, 0, 7]).is_none() && status_iu(&[0x06, 0]).is_none());
}

#[test]
fn tags_start_at_1_and_come_back() {
    let mut t = Tags::new(2);
    assert_eq!((t.take(), t.take(), t.take()), (Some(1), Some(2), None));
    t.give(1);
    t.give(0);
    t.give(40);
    assert_eq!(t.take(), Some(1));
    assert_eq!(Tags::new(0).take(), Some(1), "one at least");
}
