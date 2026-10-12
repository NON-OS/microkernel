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

//! An embedded controller's fields read as firmware reads them (ACPI 6.5,
//! 12.11 and the Mutex/Acquire examples of 19.6.1 and 19.6.88): a Lock
//! field in the EmbeddedControl space, under a mutex taken with Acquire
//! (ECMX, 0xFFFF), and the same mutex taken again by a method it calls.

use super::more::*;
use super::run::ok;
use crate::enc::*;
use crate::rig::Rig;

fn ec() -> Rig {
    let read = cat(&[
        &acquire("ECMX", 0xFFFF),
        &store(&name("BRC0"), &local(0)),
        &release("ECMX"),
        &ret(&local(0)),
    ]);
    let nested = cat(&[
        &acquire("ECMX", 0xFFFF),
        &store(&bin(ADD, &invoke("RBRC", &[]), &int(1)), &local(0)),
        &release("ECMX"),
        &ret(&local(0)),
    ]);
    // Acquire answers Zero when it got the mutex, so this is Zero only if
    // every evaluation before it let the mutex go.
    let free = cat(&[&store(&acquire("ECMX", 0), &local(0)), &release("ECMX"), &ret(&local(0))]);
    let body = cat(&[
        &mutex("ECMX", 0),
        &region("ERAM", 3, 0, 0xFF),
        &field("ERAM", 0x11, &[skip(0x30 * 8), unit("BST0", 8), unit("BRC0", 16)]),
        &method("RBRC", 0, &read),
        &method("NEST", 0, &nested),
        &method("FREE", 0, &free),
    ]);
    let mut r = Rig::new(&device("\\_SB.EC0", &body)).connected();
    r.probe.bytes.extend([((3, 0x31), 0x34), ((3, 0x32), 0x12)]);
    r
}

#[test]
fn a_lock_field_under_the_ec_mutex() {
    let mut r = ec();
    let (v, ran) = ok(&mut r, "\\_SB.EC0.RBRC");
    assert_eq!(v, 0x1234);
    let ec_reads: Vec<_> = r.probe.log.iter().filter(|l| l.0 == 3).map(|l| (l.1, l.2)).collect();
    assert_eq!(ec_reads, [(0x31, 1), (0x32, 1)], "ByteAcc: two byte reads, nothing else");
    assert!(ran.steps < 100 && ran.time_us == 0, "{} steps, {} us", ran.steps, ran.time_us);
}

#[test]
fn the_mutex_taken_again_inside_and_free_after() {
    let mut r = ec();
    assert_eq!(ok(&mut r, "\\_SB.EC0.NEST").0, 0x1235, "the owner may acquire what it holds");
    assert_eq!(ok(&mut r, "\\_SB.EC0.FREE").0, 0, "released on every path out");
    assert_eq!(ok(&mut r, "\\_SB.EC0.RBRC").0, 0x1234);
}
