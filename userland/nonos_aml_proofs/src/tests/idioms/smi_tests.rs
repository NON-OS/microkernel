// NONOS Operating System (AGPL-3.0-or-later)
//! A software SMI through SystemIO, as firmware writes it: the argument to
//! the data port, then the command to the SMI command port (the FADT's
//! SMI_CMD, 0xB2 on most boards), then the answer read back from the data
//! port. The order is the idiom: SMM takes the data byte as it stands when
//! the command write traps (ACPI 6.5, 5.2.9 for SMI_CMD; the trap on write
//! is recalled, unverified). This is a firmware pattern, not a worked
//! example in the specification.

use crate::enc::*;
use crate::object::Object;
use crate::rig::Rig;

const CMD: u64 = 0xB2;
const DATA: u64 = 0xB3;

fn rig() -> Rig {
    let body =
        cat(&[&store(&arg(1), &name("SMID")), &store(&arg(0), &name("SMIC")), &ret(&name("SMID"))]);
    let defs = cat(&[
        &region("SMIP", 1, CMD, 2),
        &field("SMIP", 0x01, &[unit("SMIC", 8), unit("SMID", 8)]),
        &method("\\SMI", 2, &body),
    ]);
    Rig::new(&defs).connected()
}

#[test]
fn data_then_command_then_the_answer() {
    let mut r = rig();
    for (cmd, data) in [(0x44, 7), (0xA1, 0), (0xFF, 0xFF)] {
        let start = r.probe.log.len();
        let args = vec![Object::Integer(cmd), Object::Integer(data)];
        assert_eq!(crate::run::num(&r.call("\\SMI", args).unwrap()), data);
        let want = [(1, DATA, 1, Some(data)), (1, CMD, 1, Some(cmd)), (1, DATA, 1, None)];
        assert_eq!(r.probe.log[start..], want, "cmd {cmd:#x}: one byte each, in order");
    }
}

#[test]
fn a_wide_argument_is_cut_to_the_byte_port() {
    // Store to an 8-bit unit keeps the low bits (19.3.5.7, recalled).
    let mut r = rig();
    let args = vec![Object::Integer(0x1_44), Object::Integer(0x3_07)];
    assert_eq!(crate::run::num(&r.call("\\SMI", args).unwrap()), 0x07);
    assert_eq!(r.probe.log[1], (1, CMD, 1, Some(0x44)), "the port never sees a wider write");
}
