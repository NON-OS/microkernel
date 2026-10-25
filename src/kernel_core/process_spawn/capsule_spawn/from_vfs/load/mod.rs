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

mod args;
mod asked;
mod baked_apps;
mod baked_names;
mod bound;
mod endpoint;
mod name_rule;
mod names;
mod spawn;
mod trust;

pub(crate) use bound::{installed as child_caps, missing as beyond_parent};
pub(crate) use spawn::load_capsule_from_vfs;
