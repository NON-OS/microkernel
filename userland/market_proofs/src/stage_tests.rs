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

//! The install stage as the kernel writes it (market_proto::Stage).

#[test]
fn the_install_stage_reads_as_the_kernel_writes_it() {
    use nonos_market_proto::Stage;
    assert_eq!(Stage::of(0), Stage::Idle);
    assert_eq!(Stage::of(1), Stage::Queued);
    assert_eq!(Stage::of(2), Stage::Installing);
    assert_eq!(Stage::of(3), Stage::Installed);
    assert_eq!(Stage::of(4), Stage::Refused);
    assert_eq!(Stage::of(16 + 5), Stage::Failed(5));
    assert_eq!(Stage::of(16 + 255), Stage::Failed(255));
    // An exit code is a u8: past it names no reason, so it reads as an unknown stage.
    assert_eq!(Stage::of(16 + 256), Stage::Idle, "never saturated to reason 255");
    assert_eq!(Stage::of(i64::MAX), Stage::Idle);
    assert_eq!(Stage::of(7), Stage::Idle, "an unknown stage");
    // An errno for the question is no stage.
    assert_eq!(Stage::of(-22), Stage::Idle);
    assert!(Stage::Queued.pending() && Stage::Installing.pending());
    assert!(!Stage::Installed.pending() && !Stage::Failed(2).pending());
}
