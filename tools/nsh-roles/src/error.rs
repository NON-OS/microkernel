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

//! Why the tool refused. Nothing is written after a refusal, so an operator
//! never hands a host or a user a file the host would then refuse.

use core::fmt;

#[derive(Debug)]
pub enum Error {
    /// A key file, the randomness, or a file the tool reads or writes.
    Enrol(nsh_enrol::Error),
    /// The table or list made, or the one given, does not open.
    Policy(nsh_roles::PolicyError),
    /// The grant made does not pass the check a host runs on it.
    Grant(nsh_roles::Refusal),
    /// A value given is not one the format admits.
    Value(String),
    Usage(String),
}

impl From<nsh_enrol::Error> for Error {
    fn from(e: nsh_enrol::Error) -> Self {
        Error::Enrol(e)
    }
}

impl From<nsh_roles::PolicyError> for Error {
    fn from(e: nsh_roles::PolicyError) -> Self {
        Error::Policy(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Enrol(e) => write!(f, "{e}"),
            Error::Policy(e) => write!(f, "the table or list is refused: {e:?}"),
            Error::Grant(r) => write!(f, "the grant is refused: {r:?} ({:?})", r.reason()),
            Error::Value(v) => write!(f, "invalid {v}"),
            Error::Usage(u) => write!(f, "{u}"),
        }
    }
}

impl std::error::Error for Error {}

pub fn value(what: &str) -> Error {
    Error::Value(what.to_string())
}
