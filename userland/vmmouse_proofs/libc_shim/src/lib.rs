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

//! The things the included driver files take from `nonos_libc`, each
//! recorded per thread; every test runs on its own thread.

mod backdoor;
#[path = "../../../../src/arch/x86_64/hypervisor/backdoor_rule.rs"]
mod backdoor_rule;
mod input;
mod ipc;
#[path = "../../../../src/arch/x86_64/hypervisor/vendor.rs"]
mod vendor;

pub use backdoor::{backdoor_calls, backdoor_present, mk_hv_backdoor, set_host, Host};
pub use input::{
    input_abs_scale, mk_input_event_post, posted, InputEvent, INPUT_ABS_MAX,
    INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_POINTER_ABS, INPUT_KIND_POINTER_REL,
    INPUT_KIND_WHEEL,
};
pub use ipc::{idled, logged, mk_debug, mk_idle_ms};
pub use vendor::Hypervisor;
