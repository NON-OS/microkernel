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

//! The gen3 radio as the serving loop runs it. At start the card is brought
//! up once (`bring`); on an SO platform with bundled firmware that ends with
//! the firmware up and a passive scan ready to run. The loop then calls
//! `tick` between requests: a background sweep is started, pumped and, when
//! the firmware completes it, the scan list is aged and the next sweep waits
//! a rest. Beacons and probe responses go into the shared scan list as they
//! are received. A join (`join`) stops the sweep, hunts the network and
//! keys a link; while the port is open the background scan rests and `tick`
//! services the link instead.
//!
//! The broker DMA grants (`grant`), the uptime clock (`clock`), the console
//! lines (`say`) and the kernel's randomness (`address`, `entropy`) are the
//! capsule's side of the `Region` and `Clock` traits and the entropy the host
//! proofs supply directly.

mod address;
mod bring;
mod clock;
mod entropy;
mod grant;
mod join;
mod say;
mod start;
mod state;
mod sweep_end;
mod tick;
mod view;

pub use state::Radio;
use state::{State, REST_MS};
