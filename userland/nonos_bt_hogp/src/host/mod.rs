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
mod chars;
mod descs;
mod dispatch;
mod enable;
mod leds;
mod phase;
mod ready;
mod services;
mod sources;
mod start;
mod state;
mod step;
mod timer;
mod types;

pub use phase::Phase;
pub use state::Host;
pub use step::{Outcome, Step};
pub use types::{Kind, Source, MAX_DECLS, MAX_REQUESTS, MAX_SERVICES, TIMEOUT_MS};
