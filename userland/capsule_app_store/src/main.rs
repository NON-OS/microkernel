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

#![no_std]
#![no_main]

extern crate alloc;

mod store;

/* What a Qwen tier needs in memory, and sizes, as the model fetcher says
 * them, for a tier's card. */
#[path = "../../capsule_model_fetch/src/need.rs"]
mod need;
#[path = "../../capsule_model_fetch/src/size.rs"]
mod size;
/* The fetcher's rule for the path a download takes, said on a tier's card. */
#[path = "../../capsule_model_fetch/src/path.rs"]
mod path;
/* Where a running download stands, as the fetcher answers it. */
#[path = "../../capsule_model_fetch/src/status_wire.rs"]
mod status_wire;
#[path = "../../capsule_model_fetch/src/status_read.rs"]
mod status_read;

/* The route type `path` reads, as every capsule decides it. */
mod net {
    pub use nonos_route_link::Route;
}

use nonos_app_skeleton::run;

/// # Safety The loader calls this once, on a fresh stack, as the process entry
/// point.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    run(store::Store::new)
}
