// NONOS Operating System (AGPL-3.0-or-later)
//! The NSMB answers.

use crate::errno::{E_INVAL, E_NXIO, E_OK};
use crate::model::{Device, Model};
use crate::server::answer::{answer, Info};
use crate::wire::{HDR_LEN, OP_HEALTHCHECK, OP_INFO, OP_TRANSFER, REPLY_MAX};

const INFO: Info = Info { vendor: 0x8086, device: 0x2930, io_ports: 32 };

fn frame(op: u16, payload: &[u8]) -> Vec<u8> {
    let mut f = vec![0u8; HDR_LEN];
    f[0..4].copy_from_slice(&0x4e53_4d42u32.to_le_bytes());
    f[4..6].copy_from_slice(&1u16.to_le_bytes());
    f[6..8].copy_from_slice(&op.to_le_bytes());
    f[12..16].copy_from_slice(&0x0102_0304u32.to_le_bytes());
    f[16..20].copy_from_slice(&(payload.len() as u32).to_le_bytes());
    f.extend_from_slice(payload);
    f
}

fn ask(m: &mut Model, msg: &[u8]) -> (i32, Vec<u8>) {
    let mut tx = [0u8; REPLY_MAX];
    let n = answer(m, &INFO, msg, &mut tx);
    let status = i32::from_le_bytes(tx[HDR_LEN..HDR_LEN + 4].try_into().unwrap());
    assert_eq!(&tx[12..16], &[4, 3, 2, 1], "answered under the request's id");
    (status, tx[HDR_LEN + 4..n].to_vec())
}

#[test]
fn a_transfer_answers_its_length_and_bytes() {
    let mut m = Model::new(vec![Device::eeprom(0x50)]);
    assert_eq!(
        ask(&mut m, &frame(OP_TRANSFER, &[0x50, 1, 3, 4, 0])),
        (E_OK, vec![2, 4 ^ 0x5A, 5 ^ 0x5A])
    );
    assert_eq!(ask(&mut m, &frame(OP_TRANSFER, &[0x51, 1, 2, 4, 0])), (E_NXIO, vec![]));
    assert_eq!(ask(&mut m, &frame(OP_TRANSFER, &[0x51, 1, 9, 4, 0])), (E_INVAL, vec![]));
}

#[test]
fn info_and_healthcheck_take_no_payload() {
    let mut m = Model::new(vec![]);
    assert_eq!(
        ask(&mut m, &frame(OP_INFO, &[])),
        (E_OK, vec![0x86, 0x80, 0x30, 0x29, 32, 0, 0, 0])
    );
    assert_eq!(ask(&mut m, &frame(OP_HEALTHCHECK, &[])), (E_OK, vec![]));
    assert_eq!(ask(&mut m, &frame(OP_HEALTHCHECK, &[1])).0, E_INVAL);
    assert_eq!(ask(&mut m, &frame(9, &[])).0, E_INVAL);
}

#[test]
fn a_frame_that_does_not_decode_is_still_answered() {
    let mut m = Model::new(vec![]);
    let mut f = frame(OP_INFO, &[]);
    f[0] ^= 1;
    assert_eq!(ask(&mut m, &f).0, E_INVAL);
    let mut short = frame(OP_TRANSFER, &[0x50, 1, 2, 0, 0]);
    short.truncate(HDR_LEN + 2);
    assert_eq!(ask(&mut m, &short).0, E_INVAL);
}
