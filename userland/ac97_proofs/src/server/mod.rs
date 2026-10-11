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

//! The serve path without the receive loop: the request dispatch, the
//! refill and the stream, each file by path.

#[path = "../../../capsule_driver_ac97/src/server/dispatch.rs"]
mod dispatch;
#[path = "../../../capsule_driver_ac97/src/server/refill.rs"]
mod refill;
#[path = "../../../capsule_driver_ac97/src/server/respond.rs"]
mod respond;
#[path = "../../../capsule_driver_ac97/src/server/stream.rs"]
mod stream;

pub use dispatch::serve;
pub use refill::refill;
pub use stream::Stream;
