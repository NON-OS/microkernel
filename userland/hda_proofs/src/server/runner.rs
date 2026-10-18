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

//! The serving loop's work between requests, from the shipping files: the
//! jack poll and its schedule, the completion check and the period refill.
//! Their items are visible to this module alone, so their tests live here,
//! with the handlers' (`serving` brings up the driver they all run on).

#[path = "../../../capsule_driver_hda/src/server/runner/jack_poll.rs"]
mod jack_poll;
#[path = "../../../capsule_driver_hda/src/server/runner/jack_schedule.rs"]
pub mod jack_schedule;
#[path = "../../../capsule_driver_hda/src/server/runner/max_tx_body.rs"]
mod max_tx_body;
#[path = "../../../capsule_driver_hda/src/server/runner/poll_irq.rs"]
mod poll_irq;
#[path = "../../../capsule_driver_hda/src/server/runner/refill.rs"]
mod refill;
#[path = "../../../capsule_driver_hda/src/server/runner/refill_log.rs"]
mod refill_log;

mod jack_lost_tests;
mod jack_poll_tests;
mod pcm_tests;
mod poll_irq_tests;
mod queue_tests;
mod refill_count_tests;
mod refill_tests;
mod serve_tests;
mod serving;
mod stream_serve_tests;
