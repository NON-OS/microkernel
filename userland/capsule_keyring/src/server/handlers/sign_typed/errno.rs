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

//! The errno a refusal answers with.

use crate::protocol::{EACCES, EINVAL, ENOENT};
use crate::server::typed::refusal::Refusal;

/// A contract not deployed is ENOENT, so the caller can say which; anything
/// about who is asking is EACCES; everything else is EINVAL.
pub fn errno(r: Refusal) -> i32 {
    match r {
        Refusal::NotDeployed => ENOENT,
        Refusal::Caller | Refusal::NotSigner => EACCES,
        Refusal::Malformed | Refusal::UnknownEntry | Refusal::Chain | Refusal::Field(_) => EINVAL,
    }
}
