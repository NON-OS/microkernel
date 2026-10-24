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

//! The kinds of action a box refuses, as its report names them; each refusal is one kind.

pub const KINDS: usize = 9;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A file written, made, renamed or removed outside the writable folders.
    Write,
    /// A socket, bind, listen or stream the box gives no network for.
    Network,
    /// A program started by execve without EXEC.
    Exec,
    /// Executable memory made without JIT.
    Jit,
    /// The display reached without WAYLAND.
    Display,
    /// A model read without MODELS_RO.
    Models,
    /// A call outside the program's declared calls.
    Call,
    /// A call no guest is ever served (ptrace, mount, unshare, ...).
    Barred,
    /// A call the personality does not serve at all.
    Unserved,
}

pub const ALL: [Kind; KINDS] = [
    Kind::Write,
    Kind::Network,
    Kind::Exec,
    Kind::Jit,
    Kind::Display,
    Kind::Models,
    Kind::Call,
    Kind::Barred,
    Kind::Unserved,
];

impl Kind {
    pub fn index(self) -> usize {
        ALL.iter().position(|k| *k == self).unwrap_or(0)
    }

    pub fn tag(self) -> &'static str {
        ["write", "network", "exec", "jit", "display", "models", "call", "barred", "unserved"]
            [self.index()]
    }
}

/// The kind of a mask refusal, from the grant `app::refuse` says it lacks.
pub fn of_needs(needs: &str) -> Kind {
    match needs {
        "EXEC" => Kind::Exec,
        "JIT" => Kind::Jit,
        "WAYLAND" => Kind::Display,
        "MODELS_RO" => Kind::Models,
        _ => Kind::Network,
    }
}
