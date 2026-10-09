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
 * The install stage table makes room by dropping a finished install, the
 * one recorded longest ago, and never one still queued or running.
 *
 * The kernel's choice is included by path. The table it serves used to
 * stop recording once 32 listings had been asked for, so a later install
 * ran with no stage the store could read; the checks below fail against
 * a choice that drops an install still moving, or a newer finished one.
 */

#[path = "../../../../src/userspace/init/linux_jobs/evict.rs"]
mod evict;
/*
 * And the order of jobs that waited: an install asked for while another was
 * downloading stays queued and runs after it, in the order asked, where it
 * was once started at once and refused with EndpointCollision.
 */
#[path = "../../../../src/userspace/init/linux_jobs/order.rs"]
mod order;
mod order_tests;
mod tests;
