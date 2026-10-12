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

pub(in crate::userspace::capsule_driver_usb_hid) const OP_HEALTHCHECK: u16 = 0x0001;
// 0x0002 probes a configuration descriptor, which nothing in the kernel
// asks. 0x0003 to 0x0006 fed reports in and read key and mouse events back
// out; the driver removed them and answers them as unknown ops: E_BAD_OP
// without a body, E_INVAL with one.
pub(in crate::userspace::capsule_driver_usb_hid) const OP_GET_STATE: u16 = 0x0007;
