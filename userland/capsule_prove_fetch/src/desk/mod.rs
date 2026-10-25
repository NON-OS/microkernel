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

mod answer;
mod errno;
mod finish;
mod head;
mod held;
mod http;
mod put;
mod relay;
mod reply;
mod transfer;
mod verdict;

pub use answer::{answer, Caller};
pub use head::Refused;
pub use http::{get_request, put_request};
pub use relay::{OP_MS, READ_ANYONE_MS, READ_NYM_MS, RELAY, ROTATIONS};
pub use reply::{Read, Reply};
pub use transfer::Desk;
pub use verdict::verdict;
