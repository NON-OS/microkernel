// NONOS Operating System (AGPL-3.0-or-later)
//! What the kernel's client asks besides a read (`serving`): a write and a
//! flush the device takes, and the re-bind generation its state reply ends
//! with (server/handlers/get_state.rs).

use crate::answers::OK;
use crate::fixture::{frame, one_sector};
use crate::protocol::{HDR_LEN, OP_BLK_FLUSH, OP_BLK_WRITE, OP_GET_STATE, STATUS_LEN};
use crate::server::respond::sent;
use crate::server::serve::Server;
use crate::serving::ask;
use crate::xhci::setup;

/// The kernel's write of one sector: its status.
pub fn write(server: &mut Server) -> i32 {
    let mut body = one_sector(7);
    body.extend_from_slice(&[0xAA; 512]);
    setup(|d| d.answers = [OK].into_iter().collect());
    ask(server, 0, OP_BLK_WRITE, &body).status
}

/// The kernel's flush: its status.
pub fn flush(server: &mut Server) -> i32 {
    setup(|d| d.answers = [OK].into_iter().collect());
    ask(server, 0, OP_BLK_FLUSH, &[]).status
}

/// The generation, from a state reply of the counters, the report and it.
pub fn generation(server: &mut Server) -> u32 {
    let mut tx = vec![0u8; 64 * 1024];
    let _ = sent();
    server.serve(0, &frame(OP_GET_STATE, 9, &[]), &mut tx);
    let at = HDR_LEN + STATUS_LEN + 68;
    assert_eq!(sent()[0].data, 72);
    u32::from_le_bytes([tx[at], tx[at + 1], tx[at + 2], tx[at + 3]])
}
