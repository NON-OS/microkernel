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

//! The driver's scan report and scanner by `Default`, the proofs' alone: the
//! driver makes each with its `new` and wants no other way to.

use crate::scan::report::Report;
use crate::scan::Scanner;

impl Default for Report {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::scan::report::{Report, REPORT_LEN};
    use crate::scan::Scanner;

    #[test]
    fn the_defaults_are_what_new_makes() {
        let (mut a, mut b) = ([0u8; REPORT_LEN], [0u8; REPORT_LEN]);
        Report::default().encode(&mut a);
        Report::new().encode(&mut b);
        assert_eq!(a, b);
        assert_eq!(Scanner::default().report, Report::new(), "nothing found yet");
    }
}
