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

//! A stored file mapped MAP_SHARED and served within the family
//! (file/held/shared_maps): the ranges a call reaches, a munmap's split, the
//! one writer rule, the 64-map bound, what a rename and an unlink do, which
//! pages a write-back takes, and a model of the whole protocol run on
//! random calls.

mod bound;
mod differ;
mod model;
mod model_back;
mod model_calls;
mod model_check;
mod model_guest;
mod model_run;
mod names;
mod protect;
mod ranges;
mod refused;
mod split;
mod support;
mod writers;
