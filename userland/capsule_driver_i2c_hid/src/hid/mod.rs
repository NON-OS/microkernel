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

mod descriptor;
mod input_len;
pub(crate) mod input_mode;
mod input_register;
mod power;
mod probe;
mod report_desc;

pub use descriptor::{valid_descriptor, HID_DESC_LEN};
pub use input_len::input_len;
pub use input_mode::{configure_reporting, Configured, I2cFeatures};
pub use input_register::input_register;
pub use power::{sleep, wake};
pub use probe::probe_addr;
pub use report_desc::{
    decode_mouse, decode_touch, parse as parse_report_descriptor,
    parse_mouse as parse_mouse_layout, Field, MouseLayout, TouchLayout, TouchSample,
};
pub use report_desc::{primary, Multitouch, ScreenEvent, Surface};
