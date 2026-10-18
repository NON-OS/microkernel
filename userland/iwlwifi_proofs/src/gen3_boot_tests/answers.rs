// NONOS Operating System (AGPL-3.0-or-later)

//! The modeled firmware's answers to the post-ALIVE commands.

use crate::gen3_model::{reply, Out, Seen};

/// What the modeled firmware answers to each command it is sent.
type Answers = Box<dyn FnMut(&Seen) -> Vec<Out>>;

// The answers a unified-image firmware gives to the bring-up commands.
pub(super) fn firmware_answers(ax_mcc: bool) -> Answers {
    Box::new(move |s: &Seen| match (s.group, s.cmd) {
        (0x0C, 0x00) => vec![reply(s, vec![]), Out { cmd: 0x04, group: 1, seq: 0x8000, payload: vec![] }],
        (0x0C, 0x02) => {
            let mut p = vec![0u8; 468];
            p[12] = 3;
            p[16] = 3;
            p[20] = 1; // LAR
            reply_with(s, p)
        }
        (0x01, 0xC8) => {
            let mut p = vec![0u8; if ax_mcc { 20 } else { 16 }];
            p[4..6].copy_from_slice(&0x5553u16.to_le_bytes());
            reply_with(s, p)
        }
        (0x01, 0x0D) => {
            let mut beacon = vec![0u8; 64];
            beacon[0..2].copy_from_slice(&(24u16 + 12 + 6 + 4).to_le_bytes());
            beacon[2] = 0x20; // MIC/CRC length 2 << 1: four bytes at the end
            beacon[12] = 0x03; // CRC and overrun OK
            beacon[42] = 6; // channel
            beacon.extend_from_slice(&[0x80, 0, 0, 0]);
            beacon.extend_from_slice(&[0xFF; 6]);
            beacon.extend_from_slice(&[0x02, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE]);
            beacon.extend_from_slice(&[0x02, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE]);
            beacon.extend_from_slice(&[0, 0]);
            beacon.extend_from_slice(&[0; 8]);
            beacon.extend_from_slice(&[0x64, 0, 0x11, 0x04]);
            beacon.extend_from_slice(&[0, 4]);
            beacon.extend_from_slice(b"Home");
            beacon.extend_from_slice(&[0xC1, 0xC2, 0xC3, 0xC4]);
            vec![
                reply(s, vec![]),
                Out { cmd: 0xC1, group: 0, seq: 0x8000, payload: beacon },
                Out { cmd: 0x0F, group: 0, seq: 0x8000, payload: vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
            ]
        }
        _ => vec![reply(s, vec![])],
    })
}

fn reply_with(s: &Seen, p: Vec<u8>) -> Vec<Out> {
    vec![reply(s, p)]
}
