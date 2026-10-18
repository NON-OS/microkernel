// NONOS Operating System (AGPL-3.0-or-later)
//! Whom the driver serves, through its real dispatch: every op from any
//! sender but the kernel's client is refused, a health check aside, and the
//! codec ops never touch the bound device's state. A capsule that reached
//! the inbox by another name reset the bound stick's residue quirk with one
//! configuration descriptor, and every read after it failed.

use crate::bot::CommandStatus;
use crate::fixture::bot_config;
use crate::protocol::{
    CBW_LEN, E_ACCES, OP_BUILD_TEST_UNIT_READY, OP_HEALTHCHECK, OP_PROBE_CONFIG,
};
use crate::server::admits::admits;
use crate::server::serve::Server;
use crate::serving::ask;

const INQUIRY: u8 = 0x12;
const READ10: u8 = 0x28;

/// A server whose stick was found to report bogus residues.
fn quirked() -> Server {
    let mut server = Server::new();
    let whole_inquiry = CommandStatus { tag: 1, residue: 36, status: 0 };
    assert_eq!(server.state.residue(INQUIRY, 36, 36, whole_inquiry), 0);
    server
}

fn residues_ignored(server: &mut Server) -> bool {
    let read = CommandStatus { tag: 1, residue: 512, status: 0 };
    server.state.residue(READ10, 512, 512, read) == 0
}

#[test]
fn every_op_from_anyone_but_the_kernel_is_refused_but_a_health_check() {
    let mut server = Server::new();
    for sender in [1, 42, 0x7FFF_FFFF, u32::MAX] {
        for op in (0..=0x40).chain([0xFFFF]) {
            let reply = ask(&mut server, sender, op, &[]);
            let want = if op == OP_HEALTHCHECK { 0 } else { E_ACCES };
            assert_eq!((reply.to, reply.op, reply.status), (sender, op, want), "op {op:#x}");
            assert_eq!(admits(op, sender), op == OP_HEALTHCHECK);
        }
    }
    assert!((0..=0xFFFF).all(|op| admits(op, 0)), "the kernel is refused nothing");
}

#[test]
fn a_configuration_from_a_capsule_leaves_the_bound_sticks_quirk() {
    let mut server = quirked();
    let reply = ask(&mut server, 42, OP_PROBE_CONFIG, &bot_config());
    assert_eq!(reply.status, E_ACCES);
    assert!(residues_ignored(&mut server), "its residues still count for nothing");
}

#[test]
fn the_kernels_codec_ops_work_on_a_state_of_their_own() {
    let mut server = quirked();
    let reply = ask(&mut server, 0, OP_PROBE_CONFIG, &bot_config());
    assert_eq!(reply.status, 0, "the kernel is still served");
    assert!(reply.data > 0);
    let built = ask(&mut server, 0, OP_BUILD_TEST_UNIT_READY, &[]);
    assert_eq!((built.status, built.data), (0, CBW_LEN));
    assert!(residues_ignored(&mut server), "the bound stick's state was not touched");
}
