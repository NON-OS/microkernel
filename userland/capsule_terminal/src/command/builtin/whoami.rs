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

use alloc::vec::Vec;

use crate::command::output::Output;
use crate::term::identity::username;

/*
 * The name first and bare, as `whoami` prints it anywhere, then the capsule.
 * Who signed it is the kernel's record of this process, not a line the binary
 * carries about itself.
 */
pub fn run(out: &mut Output<'_>, _argv: &[&[u8]]) {
    out.writeln(username());
    out.writeln(b"  capsule: app.terminal");
    out.writeln(b"  namespace: systems.nonos.app.terminal");
    out.writeln(b"  cpl: 3 (user)");
    let mut line = Vec::from(&b"  signed by: "[..]);
    line.extend_from_slice(super::receipt::own_line());
    out.writeln(&line);
}
