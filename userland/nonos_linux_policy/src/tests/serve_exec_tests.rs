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

//! An exec keeps SERVE_ONE only into the same field: the program exec'd
//! must serve the same port to the same capsule, or it runs without it.

use super::sample::served;
use crate::mask::{narrow, SERVE_ONE};
use crate::{unlisted, LinuxAppPolicy, ServeOne};

#[test]
fn an_exec_keeps_serve_one_only_into_the_same_field() {
    let core = served();
    let kept = |program: &LinuxAppPolicy| core.exec_keeps(program, narrow(core.mask, program.mask));
    assert_eq!(kept(&served()), core.mask);
    let port = LinuxAppPolicy { serve: ServeOne::new(5226, b"app.simplex"), ..served() };
    let name = LinuxAppPolicy { serve: ServeOne::new(5225, b"app.simplexx"), ..served() };
    let none = LinuxAppPolicy { serve: None, ..served() };
    for program in [port, name, none, unlisted([1; 32])] {
        assert_eq!(kept(&program) & SERVE_ONE, 0, "{:?}", program.serve);
        assert_eq!(kept(&program), narrow(core.mask, program.mask) & !SERVE_ONE);
    }
    let unserved = LinuxAppPolicy { mask: core.mask & !SERVE_ONE, ..served() };
    assert_eq!(unserved.exec_keeps(&core, core.mask) & SERVE_ONE, 0);
}
