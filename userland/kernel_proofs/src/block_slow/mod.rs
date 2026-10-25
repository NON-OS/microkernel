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

//! Which block requests the kernel's log says: every failed one, and any
//! answer at or past SLOW_MS, so a stalled driver shows before it fails.

#[path = "../../../../src/hardware/block_device/wire/slow_rule.rs"]
mod slow_rule;

#[cfg(test)]
mod tests {
    use super::slow_rule::{said, SLOW_MS};

    // Said before a driver's own 30 s budget runs out.
    const _: () = assert!(SLOW_MS < 30_000);

    #[test]
    fn a_failed_request_is_always_said() {
        for status in [-110, -5, -1, 1, i32::MIN, i32::MAX] {
            for ms in [0, 1, SLOW_MS - 1, SLOW_MS, 35_000] {
                assert!(said(status, ms), "{status} {ms}");
            }
        }
    }

    #[test]
    fn a_good_answer_is_said_only_once_slow() {
        assert!(!said(0, 0) && !said(0, SLOW_MS - 1));
        assert!(said(0, SLOW_MS) && said(0, 33_000));
    }
}
