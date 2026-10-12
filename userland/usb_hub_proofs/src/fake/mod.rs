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
//! A modelled hub and the devices on its ports, answering as USB 2.0
//! chapter 11 and USB 3.2 chapter 10 say a hub answers.

mod classes;
mod control;
mod host;
mod model;
mod standard;
mod state;
mod status;

pub use classes::{root_hub, Bind};
pub use model::{Device, HubModel, Kind};
pub use state::Fake;
