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

//! A refusal: the command fails, says why, and writes nothing.

use super::common::{run, said};

pub fn refused(d: &std::path::Path, args: &[&str], why: &str, out: &str) {
    let o = run(d, args);
    assert!(!o.status.success(), "{args:?}");
    assert!(
        said(&o).starts_with("nsh-roles: ") && said(&o).contains(why),
        "{args:?}: {}",
        said(&o)
    );
    assert!(!d.join(out).exists(), "{out} was written");
}
