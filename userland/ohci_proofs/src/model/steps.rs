// NONOS Operating System (AGPL-3.0-or-later)
//! The steps every class driver takes first, as one call each.

use super::handles::Cx;
use super::wire::call;
use crate::wire::ops::*;

/// A slot enabled and the device on `port` addressed in it.
pub fn addressed(cx: &mut Cx, port: u8) -> u8 {
    let slot = call(cx, OP_ENABLE_SLOT, &[]);
    assert_eq!(slot.status, 0);
    let slot = slot.data[0];
    let a = call(cx, OP_ADDRESS_DEVICE, &[slot, port]);
    assert_eq!(a.status, 0, "port {port} addressed");
    slot
}

/// Bulk IN endpoint 2 and OUT endpoint 3 configured, 64-byte packets.
pub fn bulk_ready(cx: &mut Cx, slot: u8) {
    let r = call(cx, OP_CONFIGURE_BULK, &[slot, 0x82, 0x03, 0, 64, 0, 64, 0]);
    assert_eq!((r.status, r.data.as_slice()), (0, &[5u8, 6, 0, 0][..]));
}
