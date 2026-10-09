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

//! The kernel's `MkEnroll` files but `call.rs`, which only copies to and from
//! user memory, with the tests beside them.

#[path = "../../../../../../src/syscall/microkernel/device_proof/enroll/answer.rs"]
mod answer;
#[path = "../../../../../../src/syscall/microkernel/device_proof/enroll/codec.rs"]
mod codec;
#[path = "../../../../../../src/syscall/microkernel/device_proof/enroll/errors.rs"]
mod errors;
#[path = "../../../../../../src/syscall/microkernel/device_proof/enroll/request.rs"]
mod request;

mod frame_tests;
mod live_tests;
mod request_tests;
