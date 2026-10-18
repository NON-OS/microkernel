// NONOS Operating System (AGPL-3.0-or-later)
//! driver.xhci0 as the proofs play it, on its real wire: one device on
//! root port 1, QEMU's usb-net, which answers GET_DESCRIPTOR from its
//! descriptors and takes every other request; its bulk IN is always empty.

use crate::qemu_usb_net::{CONFIG_ECM, CONFIG_RNDIS, DEVICE};

pub const OP_PORT_STATUS: u16 = 0x0003;
pub const OP_ENABLE_SLOT: u16 = 0x0004;
pub const OP_DISABLE_SLOT: u16 = 0x0005;
pub const OP_ADDRESS_DEVICE: u16 = 0x0006;
pub const OP_CONTROL_TRANSFER: u16 = 0x000B;
pub const OP_BULK_IN_POLL: u16 = 0x0014;
/// The slot the controller hands out.
pub const SLOT: u8 = 5;

pub fn op(req: &[u8]) -> u16 {
    u16::from_le_bytes([req[6], req[7]])
}

/// A reply to `req`: its header echoed with the payload length, the
/// status word, then `data`.
pub fn reply(req: &[u8], status: i32, data: &[u8]) -> Vec<u8> {
    let mut r = req[..20].to_vec();
    r[16..20].copy_from_slice(&((4 + data.len()) as u32).to_le_bytes());
    r.extend_from_slice(&status.to_le_bytes());
    r.extend_from_slice(data);
    r
}

/// What the controller and the device answer to `req`.
pub fn answer(req: &[u8]) -> Vec<u8> {
    let body = &req[20..];
    match op(req) {
        // One port, id 1, free, Current Connect Status set.
        OP_PORT_STATUS => reply(req, 0, &[1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]),
        OP_ENABLE_SLOT => reply(req, 0, &[SLOT, 0, 0, 0]),
        OP_ADDRESS_DEVICE => reply(req, 0, &[body[0], body[1], 0, 0, 0, 0, 0, 0]),
        OP_CONTROL_TRANSFER => control(req, body),
        OP_BULK_IN_POLL => reply(req, -11, &[]),
        _ => reply(req, 0, &[0, 0, 0, 0]),
    }
}

fn control(req: &[u8], body: &[u8]) -> Vec<u8> {
    let (kind, index) = (body[5], body[4]);
    let want = u16::from_le_bytes([body[8], body[9]]) as usize;
    let bytes: &[u8] = match (body[2], body[3], kind, index) {
        (0x80, 0x06, 1, _) => &DEVICE,
        (0x80, 0x06, 2, 0) => &CONFIG_RNDIS,
        (0x80, 0x06, 2, 1) => &CONFIG_ECM,
        _ => &[],
    };
    let n = bytes.len().min(want);
    let mut data = (n as u16).to_le_bytes().to_vec();
    data.extend_from_slice(&bytes[..n]);
    reply(req, 0, &data)
}
