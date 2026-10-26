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

//! The display drivers, best first (PLAN-coverage section 12, Selection):
//! the compositor tries them in this order and drives the first that
//! answers, the kernel holds each to the compositor, and its traces and
//! bench marks name them. Data only: the kernel uses it, and capsules
//! mount this file by path, so every reader sees one list. Whether a name
//! is a display driver is whether this list contains it. The hint senders
//! are display_hints.rs, the compositor's alone.
//!
//! Native engines go before the boot framebuffer, which comes last: it is
//! the fallback for any machine whose GPU no other driver takes.

pub(crate) const DISPLAYS: &[&str] = &[
    "driver.virtio_gpu0",
    // 15ad:0405, and 80ee:beef with an I/O BAR (VBoxSVGA).
    "driver.vmware_svga0",
    "driver.hv_synthvid0",
    "driver.bga0",
    // Intel's display engines, taken over at firmware's mode.
    "driver.intel_display0",
    // The boot framebuffer served on the wire: last, for any GPU no other
    // driver here takes.
    "driver.bootfb0",
];
