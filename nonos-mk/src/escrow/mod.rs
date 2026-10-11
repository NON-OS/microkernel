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
mod answer_types;
mod body;
mod bucket;
mod http;
mod keys;
mod offer;
mod seen;
mod serve;
mod x;

#[cfg(test)]
mod tests;

pub use answer_types::Served;
pub use bucket::Bucket;
pub use http::{read_request, write_answer};
pub use keys::prune;
pub use seen::Seen;
pub use serve::Agent;
