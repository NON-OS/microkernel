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

//! A channel's record names its relid in `irq_source`, the field a driver
//! reads to learn which channel it holds and which event flag is its own.
//! Read as text: the broker's registration and the call that makes it.

/// The source without whitespace, so formatting never breaks a match.
fn code(rel: &str) -> String {
    let path = format!("{}/../../src/{rel}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{path}: {e}"))
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

#[test]
fn a_channel_record_carries_its_relid_as_its_irq_source() {
    let broker = code("hardware/broker/vmbus.rs");
    let head = "pubfnregister_channel(device:u16,relid:u32)->Option<u64>{";
    let at = broker.find(head).expect("register_channel takes the relid");
    let body = &broker[at..(at + 400).min(broker.len())];
    assert!(body.contains("irq_source:relid,"), "the relid in irq_source");
    let call = code("syscall/microkernel/hv_vmbus.rs");
    assert!(
        call.contains("register_vmbus_channel(device,relid)"),
        "MkHvChannel registers the record with the relid it binds"
    );
}
