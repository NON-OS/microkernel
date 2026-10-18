// NONOS Operating System (AGPL-3.0-or-later)
//! F-107: the capsule ends in the middle of an EC transaction, at every
//! byte of it, and a new one starts. The EC is left with a command half
//! sent or an answer unread. A byte to the command port starts a new
//! command (ACPI 6.5, 12.3; recalled, unverified), and every command of
//! ours reads away a stale answer first, so the new capsule's reads and
//! writes are right and no half-sent write lands later. A query the dead
//! capsule sent loses that one event: the EC gave it up to nobody.

use crate::ec::{pending, query, read, write, QR_EC, RD_EC, WR_EC};
use crate::ec_model::{EcModel, PORTS};
use crate::events::EventIo;

/// The writes the dead capsule made before it ended.
fn cut(ec: &mut EcModel, writes: &[(u16, u16)]) {
    for (port, v) in writes {
        while ec.read(PORTS.command, 1).is_some_and(|s| s & 2 != 0) {}
        assert!(ec.write(*port, 1, *v));
    }
    // A restart takes a second at least (init's settle), and an EC answers
    // in microseconds: by then it has finished. Its answer, unread, stays
    // in the output buffer; it is not dropped for a new command.
    while ec.read(PORTS.command, 1).is_some_and(|s| s & 2 != 0) {}
    if let Some(b) = ec.out.take() {
        ec.tricks.stale.push_back(b);
    }
}

#[test]
fn a_cut_transaction_never_misleads_the_next_capsule() {
    let (cmd, data) = (PORTS.command, PORTS.data);
    let whole: [&[(u16, u16)]; 3] = [
        &[(cmd, RD_EC), (data, 0x10)],
        &[(cmd, WR_EC), (data, 0x20), (data, 0x99)],
        &[(cmd, QR_EC)],
    ];
    for (op, seq) in whole.iter().enumerate() {
        for k in 1..=seq.len() {
            for delay in [0, 2] {
                let mut ec = EcModel::new(delay);
                ec.mem[0x10] = 0x11;
                ec.mem[0x20] = 0x22;
                ec.events.extend([0x0A, 0x0B]);
                cut(&mut ec, &seq[..k]);
                let done_write = op == 1 && k == 3;
                let at20 = if done_write { 0x99 } else { 0x22 };
                assert_eq!(read(&PORTS, &mut ec, 0x20), Ok(at20), "op {op} cut {k}");
                assert_eq!(read(&PORTS, &mut ec, 0x10), Ok(0x11), "op {op} cut {k}");
                assert_eq!(write(&PORTS, &mut ec, 0x30, 0x5A), Ok(()));
                assert_eq!(read(&PORTS, &mut ec, 0x30), Ok(0x5A));
                assert_eq!(ec.mem[0x20], at20, "no half-sent write lands later");
                let left: &[u8] = if op == 2 { &[0x0B] } else { &[0x0A, 0x0B] };
                for e in left {
                    assert_eq!(pending(&PORTS, &mut ec), Ok(true));
                    assert_eq!(query(&PORTS, &mut ec), Ok(Some(*e)), "op {op} cut {k}");
                }
                assert_eq!(query(&PORTS, &mut ec), Ok(None));
            }
        }
    }
}
