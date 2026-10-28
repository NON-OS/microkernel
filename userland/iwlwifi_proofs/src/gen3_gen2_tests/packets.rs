// NONOS Operating System (AGPL-3.0-or-later)

//! A received frame's version 1 descriptor read as the version 3 one it
//! carries the same facts as, and several packets to a buffer, each handed
//! on in order.

use super::rig::{polls, rig, run, QU_B, QU_B0};
use crate::gen3::dev::gen2::v3_of_v1;
use crate::gen3::dev::Flow;
use crate::gen3::rx_frame::parse as parse_frame;
use crate::gen3_model::Out;

fn frame_v3() -> Vec<u8> {
    let mut p = vec![0u8; 64];
    p[0..2].copy_from_slice(&30u16.to_le_bytes());
    p[12] = 0x03; // CRC and overrun OK
    p[36..40].copy_from_slice(&0x4003u32.to_le_bytes());
    (p[40], p[41], p[42], p[43]) = (52, 60, 11, 0);
    p[44..48].copy_from_slice(&0x1234_5678u32.to_le_bytes());
    p.extend_from_slice(&[0x80, 0, 0, 0]);
    p.extend((0..26).map(|i| i as u8));
    p
}

#[test]
fn a_version_1_frame_is_read_as_the_version_3_one() {
    let v3 = frame_v3();
    let mut v1 = v3[..20].to_vec();
    v1.extend_from_slice(&[0; 8]);
    v1.extend_from_slice(&v3[36..56]);
    v1.extend_from_slice(&v3[64..]);
    let back = v3_of_v1(&v1).expect("a whole descriptor");
    assert_eq!(parse_frame(&back), parse_frame(&v3));
    assert!(parse_frame(&back).is_some_and(|f| (f.channel, f.signal) == (11, -52)));
    assert_eq!(&back[36..48], &v3[36..48], "rate, energies, channel, air time");
    assert_eq!(v3_of_v1(&v1[..47]), None, "shorter than its descriptor");
}

#[test]
fn several_packets_in_one_buffer_are_each_handed_on_in_order() {
    let r = rig(QU_B, QU_B0, false);
    run(&r, &mut polls(), |dev, _, _| {
        r.model.s.borrow_mut().pack = 3;
        for (cmd, len) in [(0x10u8, 5usize), (0x11, 100), (0x12, 0)] {
            r.model.s.borrow_mut().queued.push(Out {
                cmd,
                group: 2,
                seq: 0x8000,
                payload: vec![cmd; len],
            });
        }
        r.model.queue(Out { cmd: 0xC1, group: 0, seq: 0x8000, payload: frame_v3() });
        let mut got = Vec::new();
        dev.drain(&mut |p| {
            got.push((p.cmd, p.payload.len(), p.cmd == 0xC1 && parse_frame(p.payload).is_some()));
            Flow::Continue
        });
        assert_eq!(got, [(0x10, 5, false), (0x11, 100, false), (0x12, 0, false), (0xC1, 94, true)]);
    })
    .0
    .expect("alive");
}
