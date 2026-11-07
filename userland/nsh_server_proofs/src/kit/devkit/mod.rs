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

//! The devkit: a development machine's records (role table, anchor,
//! revocation list, and its enrolment once the machine has made its keys
//! and its request), and the client files that pair with them. Every key
//! comes from the proofs' fixed seeds and every signature from their
//! builders; only the validity windows move, to the wall clock the files
//! are made at, because the guest checks the grant and the revocation list
//! against its own clock and the client checks the enrolment against its
//! own. Throwaway keys for a development machine only: none of them is, or
//! signs for, a release key, and none is a host key.

pub mod files;
pub mod fresh;
pub mod machine;
pub mod make;
pub mod window;
pub mod write;

pub use files::{client, enrolment, records, Fixture, HOST_PUBLIC_LEN};
pub use machine::Machine;
pub use window::Window;
pub use write::write;
