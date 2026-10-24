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

//! The USB host controllers' drivers, by the name each one's spawn
//! registers, fastest first.
//!
//! Every host serves the same wire, so a class driver reaches a device by
//! the host's name alone. The kernel holds each host to the class drivers
//! (held_table.rs), and the class drivers look the hosts up in this order,
//! keeping the first that answers: a device stays with the host that
//! enumerated it. The capsules mount this file by path, so a new host is a
//! line here and its spawn's name (held to this table in kernel_proofs).
//!
//! Only data, so no capsule carries code it never calls.

pub(crate) const USB_HOSTS: &[&str] =
    &["driver.xhci0", "driver.ehci0", "driver.ohci0", "driver.uhci0"];
