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
//! The market's length-prefixed field readers and writers, as it ships them:
//! the writers for the release and commerce encoders, the readers for the
//! request tests. `lp` names them as the market does.

#[cfg(test)]
#[path = "../../../capsule_market/src/server/lp/pair.rs"]
mod pair;
#[cfg(test)]
#[path = "../../../capsule_market/src/server/lp/take.rs"]
mod take;
#[path = "../../../capsule_market/src/server/lp/write.rs"]
mod write;

pub(crate) mod lp {
    pub(crate) use super::write::{write_count, write_lp};
    #[cfg(test)]
    pub(crate) use super::{pair::parse_pair, take::read_lp_str, take::take_lp};
}
