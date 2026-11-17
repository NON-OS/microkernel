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

//! The pointer and the hit boxes of one surface, so an app lays out its
//! controls once and gets back each control's `State` and the action a click
//! fires. The toolkit cannot see input events; the app, or the skeleton's
//! `feed`, passes them in as `moved`, `pressed`, `released` and `left`.

mod build;
mod input;
mod look;
mod place;
mod react;
mod step;
mod targets;

pub use build::Controls;
pub use step::Step;
