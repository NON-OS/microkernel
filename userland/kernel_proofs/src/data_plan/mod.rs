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

/*
 * What a caller is told when the disk plan refuses: no plan is no data
 * volume, ENODEV; a plan that breaks its rules is EIO. The real mapping is
 * included by path, against the real plan error and errno values.
 */

#[path = "../../../../src/syscall/microkernel/data/plan_errno.rs"]
mod plan_errno;

#[cfg(test)]
mod tests {
    use super::plan_errno::plan_errno;
    use crate::fs::blockfs_volume::PlanError;
    use crate::syscall::microkernel::errnos::{ERRNO_IO, ERRNO_NODEV};

    #[test]
    fn a_disk_with_no_plan_is_no_data_volume() {
        assert_eq!(plan_errno(PlanError::NoPlan), ERRNO_NODEV);
        assert_eq!(ERRNO_NODEV, -19);
    }

    #[test]
    fn a_plan_that_breaks_its_rules_is_a_failing_disk() {
        for e in [
            PlanError::BelowFloor,
            PlanError::VolumeTooSmall,
            PlanError::PastEnd,
            PlanError::Overlap,
            PlanError::BadImport,
        ] {
            assert_eq!(plan_errno(e), ERRNO_IO, "{e:?}");
        }
    }
}
