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

//! Every number a guest can ask for, one table a family of calls.

pub(super) mod all;
pub(super) mod data;
pub(super) mod file;
pub(super) mod link;
pub(super) mod mem;
pub(super) mod meta;
pub(super) mod net;
pub(super) mod proc;
pub(super) mod sig;
pub(super) mod sys;
