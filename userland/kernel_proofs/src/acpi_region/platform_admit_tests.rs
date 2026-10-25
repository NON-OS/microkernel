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

//! Platform devices are admitted only over windows the region rule grants
//! and no device holds.

use super::machine::pc;
use super::platform_decode::decode;
use super::platform_rule::{admit, Verdict};
use super::platform_tests::record;
use super::platform_wire::I2C;
use super::rule::{MemKind, Refusal};

#[test]
fn admitted_only_where_the_region_rule_grants_and_no_device_sits() {
    let m = pc();
    let table = [(4u64, 0xFEB0_0000u64, 0x4000u64)];
    let p = |base, len| decode(&record(I2C, b"INT33C2", &[(base, len)], b"\\_SB.I2C0")).unwrap();
    assert_eq!(admit(&p(0xFE10_4000, 0x1000), &table, &m), Verdict::New, "device space");
    assert_eq!(
        admit(&p(0xFEB0_0000, 0x4000), &table, &m),
        Verdict::Present(4),
        "the scan found it"
    );
    assert_eq!(admit(&p(0xFEB0_2000, 0x1000), &table, &m), Verdict::Overlaps(4));
    assert_eq!(
        admit(&p(0x10_0000, 0x1000), &table, &m),
        Verdict::Refused(Refusal::Memory(MemKind::Ram))
    );
}
