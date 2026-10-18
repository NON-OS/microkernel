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

//! The namespace kept beside each registry entry: recorded at spawn, read
//! back only for a capsule the vendor root proved, dropped when it exits,
//! and never part of the root.

use crate::registry_support::{record, TABLE_IN_USE};
use crate::security::attest_registry::{
    bound_namespace_of, forget_attested, record_namespace, registry_root,
};
use crate::security::dev_roots::Authority;

#[test]
fn each_vendor_capsule_reads_back_its_own_namespace() {
    let _guard = TABLE_IN_USE.lock().unwrap();
    for (pid, ns) in [(9101, 0xa1), (9102, 0xa2)] {
        record(pid, 0x51, 0, Authority::Vendor);
        record_namespace(pid, [ns; 32]);
    }
    assert_eq!(bound_namespace_of(9101), Some([0xa1; 32]));
    assert_eq!(bound_namespace_of(9102), Some([0xa2; 32]));
    forget_attested(9101);
    forget_attested(9102);
}

#[test]
fn only_the_vendor_roots_capsules_have_one() {
    let _guard = TABLE_IN_USE.lock().unwrap();
    for (pid, authority) in [(9111, Authority::Publisher), (9112, Authority::Developer(0))] {
        record(pid, 0x61, 0, authority);
        record_namespace(pid, [0xb1; 32]);
        assert_eq!(bound_namespace_of(pid), None, "pid {pid}");
        forget_attested(pid);
    }
    record_namespace(9113, [0xb3; 32]);
    assert_eq!(bound_namespace_of(9113), None, "no registry entry, no namespace");
    forget_attested(9113);
}

#[test]
fn an_exited_capsule_leaves_no_namespace_behind() {
    let _guard = TABLE_IN_USE.lock().unwrap();
    record(9121, 0x62, 0, Authority::Vendor);
    record_namespace(9121, [0xb2; 32]);
    forget_attested(9121);
    record(9121, 0x63, 0, Authority::Vendor);
    assert_eq!(bound_namespace_of(9121), None, "a reused pid inherits nothing");
    forget_attested(9121);
}

#[test]
fn the_namespace_is_not_in_the_root() {
    let _guard = TABLE_IN_USE.lock().unwrap();
    record(9131, 0x71, 0, Authority::Vendor);
    let root = registry_root();
    record_namespace(9131, [0xc1; 32]);
    assert_eq!(registry_root(), root);
    forget_attested(9131);
}
