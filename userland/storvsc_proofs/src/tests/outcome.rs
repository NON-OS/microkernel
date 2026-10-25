// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! What a request carries and what each answer means, packet by packet,
//! against Linux's storvsc_queuecommand, storvsc_on_io_completion and
//! storvsc_handle_error.

use nonos_scsi::{Addr, Dir, Fault};

use crate::vstor::{done, max_transfer, op, outcome, reply, srb, version, Reply};

const READ10: [u8; 10] = [0x28, 0, 0, 0, 0, 8, 0, 0, 1, 0];

fn answer(srb_status: u8, scsi: u8, moved: u32, sense: &[u8]) -> Vec<u8> {
    let mut a = vec![0u8; 64];
    a[0] = 1;
    a[14] = srb_status;
    a[15] = scsi;
    a[21] = sense.len() as u8;
    a[24..28].copy_from_slice(&moved.to_le_bytes());
    a[28..28 + sense.len()].copy_from_slice(sense);
    a
}

#[test]
fn a_read_request_is_built_as_linux_builds_it() {
    let p = srb(Addr { target: 1, lun: 7 }, &READ10, Dir::In, 512).unwrap();
    let w32 = |at: usize| u32::from_le_bytes(p[at..at + 4].try_into().unwrap());
    assert_eq!((w32(0), w32(4), w32(8)), (3, 1, 0), "EXECUTE_SRB, completion asked");
    assert_eq!(u16::from_le_bytes([p[12], p[13]]), 48);
    assert_eq!(&p[14..24], &[0, 0, 0, 0, 1, 7, 10, 20, 1, 0]);
    assert_eq!(w32(24), 512);
    assert_eq!(&p[28..38], &READ10);
    assert_eq!(&p[38..50], &[0u8; 12]);
    assert_eq!((p[50], p[51]), (0xFF, 0x20));
    assert_eq!((w32(52), w32(56), w32(60)), (0x14A, 60, 0));
}

#[test]
fn each_direction_says_itself() {
    let at = Addr { target: 0, lun: 0 };
    let w = srb(at, &[0x2A; 10], Dir::Out, 4096).unwrap();
    assert_eq!((w[22], u32::from_le_bytes(w[52..56].try_into().unwrap())), (0, 0x18A));
    let n = srb(at, &[0; 6], Dir::None, 0).unwrap();
    assert_eq!((n[22], u32::from_le_bytes(n[52..56].try_into().unwrap())), (2, 0x10A));
}

#[test]
fn a_request_that_cannot_be_said_is_none() {
    let at = Addr { target: 0, lun: 0 };
    assert!(srb(at, &[], Dir::None, 0).is_none());
    assert!(srb(at, &[0; 17], Dir::None, 0).is_none());
    assert!(srb(at, &[0; 16], Dir::None, 0).is_some());
    assert!(srb(Addr { target: 0, lun: 256 }, &[0; 6], Dir::None, 0).is_none());
    assert!(srb(Addr { target: 0, lun: 255 }, &[0; 6], Dir::None, 0).is_some());
    assert!(srb(Addr { target: 256, lun: 0 }, &[0; 6], Dir::None, 0).is_none());
    assert!(srb(Addr { target: 255, lun: 0 }, &[0; 6], Dir::None, 0).is_some());
}

#[test]
fn the_starts_requests_and_their_answers() {
    assert_eq!(&op(7)[..12], &[7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(&op(7)[12..], &[0u8; 52]);
    assert_eq!(&version(0x0602)[..16], &[9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 6, 0, 0]);
    let mut a = answer(0, 0, 0, &[]);
    assert!(done(&a));
    assert!(!done(&a[..63]), "a whole packet");
    a[8] = 1;
    assert!(!done(&a), "a status");
    a[8] = 0;
    a[0] = 3;
    assert!(!done(&a), "COMPLETE_IO");
    assert_eq!(reply(&a[..12]), Some(Reply { operation: 3, status: 0 }));
    assert_eq!(reply(&a[..11]), None);
    let mut p = answer(0, 0, 0, &[]);
    p[24..28].copy_from_slice(&0x2_0000u32.to_le_bytes());
    assert_eq!(max_transfer(&p), Some(0x2_0000));
    p[8] = 5;
    assert_eq!(max_transfer(&p), None);
}

#[test]
fn success_says_what_did_not_move() {
    let o = outcome(&answer(0x01, 0, 512, &[]), &READ10, 512).unwrap();
    assert_eq!((o.status, o.residual, o.sense_len), (0, 0, 0));
    let o = outcome(&answer(0x12, 0, 200, &[]), &READ10, 512).unwrap();
    assert_eq!(o.residual, 312, "a short transfer moved 200");
    let o = outcome(&answer(0x01, 0, 9999, &[]), &READ10, 512).unwrap();
    assert_eq!(o.residual, 0, "never more than was asked");
    let o = outcome(&answer(0x01 | 0x40, 0x08, 0, &[]), &READ10, 512).unwrap();
    assert_eq!((o.status, o.residual), (0x08, 512), "a frozen queue's flag aside");
}

#[test]
fn a_check_condition_brings_its_sense() {
    let sense: Vec<u8> = (0..20).map(|i| 0x70 + i).collect();
    let o = outcome(&answer(0x04 | 0x80, 0x02, 512, &sense), &READ10, 512).unwrap();
    assert_eq!((o.status, o.residual, o.sense_len), (2, 512, 20), "nothing moved on an error");
    assert_eq!(&o.sense[..20], &sense[..]);
    let mut a = answer(0x04 | 0x80, 0x02, 0, &sense);
    a[21] = 200;
    assert_eq!(outcome(&a, &READ10, 512).unwrap().sense_len, 20, "held to 20");
    a[21] = 18;
    assert_eq!(outcome(&a, &READ10, 512).unwrap().sense_len, 18);
    let o = outcome(&answer(0x02 | 0x80, 0x00, 0, &sense), &READ10, 512).unwrap();
    assert_eq!((o.status, o.sense_len), (0, 0), "sense only with CHECK CONDITION");
}

#[test]
fn each_failure_is_named() {
    for (srb_status, fault) in [
        (0x20, Fault::NoTarget),
        (0x0A, Fault::NoTarget),
        (0x0A | 0x80, Fault::NoTarget),
        (0x0E, Fault::Reset),
        (0x02, Fault::Reset),
        (0x09, Fault::Timeout),
        (0x04, Fault::Failed),
        (0x06, Fault::Failed),
        (0x30, Fault::Failed),
        (0x00, Fault::Failed),
    ] {
        assert_eq!(outcome(&answer(srb_status, 0, 0, &[]), &READ10, 512), Err(fault), "{srb_status:#x}");
    }
}

#[test]
fn an_answer_that_is_no_completion_fails() {
    let mut a = answer(0x01, 0, 0, &[]);
    assert_eq!(outcome(&a[..63], &READ10, 512), Err(Fault::Failed));
    a[0] = 2;
    assert_eq!(outcome(&a, &READ10, 512), Err(Fault::Failed));
}

#[test]
fn test_unit_ready_failing_bare_leaves_its_status_to_the_layer_above() {
    let tur = [0u8; 6];
    let o = outcome(&answer(0x04, 0x02, 0, &[]), &tur, 0).unwrap();
    assert_eq!((o.status, o.sense_len), (2, 0));
    assert_eq!(outcome(&answer(0x04, 0x02, 0, &[]), &READ10, 0), Err(Fault::Failed));
}

#[test]
fn what_hyper_v_answers_wrongly_counts_as_success() {
    for opcode in [0x12u8, 0x1A, 0x5A] {
        let cdb = [opcode, 1, 0x80, 0, 0xFF, 0];
        let o = outcome(&answer(0x04, 0x02, 0, &[]), &cdb, 255).unwrap();
        assert_eq!((o.status, o.residual), (0, 255), "{opcode:#x}: success, nothing moved");
        let o = outcome(&answer(0x04, 0x02, 36, &[]), &cdb, 255).unwrap();
        assert_eq!(o.residual, 219, "{opcode:#x}: what the host says moved");
    }
    assert_eq!(outcome(&answer(0x04, 0x02, 0, &[]), &[], 0), Err(Fault::Failed));
}
