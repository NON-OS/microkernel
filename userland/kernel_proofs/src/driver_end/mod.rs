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
 * Which capsule ends the kernel names on the serial line. The real rule is
 * included by path; its exit codes must stay the ones the userland bring-up
 * policy leaves with (userland/libc/src/bringup/policy.rs).
 */

#[allow(dead_code)]
#[path = "../../../../src/process/exit/end_rule.rs"]
mod end_rule;

#[cfg(test)]
mod tests {
    use super::end_rule::{told, words, EXIT_ABSENT, EXIT_GAVE_UP};

    #[test]
    fn a_driver_that_found_no_device_is_named() {
        assert!(told("driver.e1000_0", EXIT_ABSENT, false));
        assert_eq!(words(EXIT_ABSENT), "no device present, not started");
    }

    #[test]
    fn a_driver_that_gave_up_is_named() {
        assert!(told("driver.nvme0", EXIT_GAVE_UP, false));
        assert!(words(EXIT_GAVE_UP).contains("given up"));
    }

    #[test]
    fn a_clean_exit_or_a_fault_gets_no_second_line() {
        assert!(!told("driver.ahci0", 0, false));
        assert!(!told("driver.ahci0", -11, true));
    }

    #[test]
    fn only_driver_names_are_told() {
        // An app or a guest's name says what the person runs.
        assert!(!told("app.terminal", 1, false));
        assert!(!told("foreign:qwen", 2, false));
        assert!(!told("drivers", 2, false));
    }

    #[test]
    fn the_codes_match_the_bring_up_policy() {
        let policy = include_str!("../../../libc/src/bringup/policy.rs");
        assert!(policy.contains("pub const EXIT_ABSENT: i32 = 2;"));
        assert!(policy.contains("pub const EXIT_GAVE_UP: i32 = 6;"));
        assert_eq!((EXIT_ABSENT, EXIT_GAVE_UP), (2, 6));
    }
}
