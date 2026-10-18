// NONOS Operating System (AGPL-3.0-or-later)
//! The UAS alternate and its four pipes, named by Pipe Usage.

use crate::disks::*;
use nonos_uas::{uas_pipes, Pipe};

#[test]
fn a_high_speed_disk_names_its_four_pipes_on_alternate_1() {
    let p = uas_pipes(&disk(false)).expect("a UAS alternate");
    assert_eq!((p.interface, p.alt), (0, 1));
    let pipe = |address| Pipe { address, max_packet: 512, burst: 0, streams: 0 };
    assert_eq!((p.command, p.status), (pipe(0x04), pipe(0x83)));
    assert_eq!((p.data_in, p.data_out), (pipe(0x85), pipe(0x06)));
}

#[test]
fn a_superspeed_disk_gives_its_bursts_and_streams() {
    let p = uas_pipes(&disk(true)).expect("a UAS alternate");
    assert_eq!((p.command.burst, p.command.streams), (15, 0));
    assert_eq!((p.status.streams, p.data_in.streams, p.data_out.streams), (5, 5, 5));
    assert_eq!(p.data_in.max_packet, 1024);
}

#[test]
fn bot_only_or_a_pipe_missing_or_misdirected_is_no_uas() {
    let bot = config(&[iface(0, 2, 0x50), bulk(0x81, 512), bulk(0x02, 512)]);
    assert!(uas_pipes(&bot).is_none());
    let mut three = vec![iface(1, 3, 0x62)];
    for (a, id) in [(0x04, 1), (0x83, 2), (0x85, 3)] {
        three.extend([bulk(a, 512), usage(id)]);
    }
    assert!(uas_pipes(&config(&three)).is_none(), "no data-out");
    let mut wrong = vec![iface(1, 4, 0x62)];
    for (a, id) in [(0x84, 1), (0x83, 2), (0x85, 3), (0x06, 4)] {
        wrong.extend([bulk(a, 512), usage(id)]);
    }
    assert!(uas_pipes(&config(&wrong)).is_none(), "a command pipe IN");
}
