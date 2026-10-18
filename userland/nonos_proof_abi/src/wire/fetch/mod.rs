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

mod bodies;
mod ops;
mod status;

pub use bodies::{anchor_of_body, get_body, get_of, put_bodies};
pub use ops::{Put, CHUNK_MAX, FETCH_MAGIC, GET, GET_BODY, PUT, PUT_HEAD, STATUS};
pub use status::{status_body, Stage, ANCHOR, NOT_CONTAINER, NO_RELAY, PENDING, RELAY, ROUTE};
