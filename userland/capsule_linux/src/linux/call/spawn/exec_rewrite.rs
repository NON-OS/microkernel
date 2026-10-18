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

//! The argv a `#!` line makes.

use alloc::vec::Vec;

use super::exec_shebang::Interp;

/// The interpreter, its one optional argument, the script, then whatever the
/// caller passed after argv[0].
pub fn rewrite(interp: &Interp, script: &[u8], args: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut out = alloc::vec![interp.path.clone()];
    out.extend(interp.arg.clone());
    out.push(script.to_vec());
    out.extend(args.iter().skip(1).cloned());
    out
}
