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

/*
 * Init's prove-job rule, queue and DELIVER rate, and the argv init
 * hands the headless prover, all included by path: who may ask, PROVE first,
 * one delivery per supervisor measurement per ten minutes, and a job told
 * its op and the one pid it serves.
 */

#[path = "../../../../src/userspace/init/prove_job/ask.rs"]
mod ask;

mod ask_tests;
#[path = "../../../../src/userspace/capsule_prove/job_argv.rs"]
mod job_argv;
mod receipt_tests;
#[path = "../../../../src/userspace/init/prove_job/rate.rs"]
mod rate;
#[path = "../../../../src/userspace/init/prove_job/slots.rs"]
mod slots;
mod tests;
