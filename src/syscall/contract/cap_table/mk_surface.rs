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

//! Surfaces, input, window instances, tools and terminals.

use crate::capabilities::CapabilityToken;
use crate::syscall::numbers::SyscallNumber;

pub(super) fn check(caps: &CapabilityToken, number: SyscallNumber) -> Option<bool> {
    Some(match number {
        SyscallNumber::MkSurfaceRegister
        | SyscallNumber::MkSurfaceShare
        | SyscallNumber::MkSurfaceRelease
        | SyscallNumber::MkBootLogo => caps.can_surface_create(),
        SyscallNumber::MkSurfaceAttach => caps.can_surface_map(),
        SyscallNumber::MkSurfacePresent => caps.can_present(),
        SyscallNumber::MkDisplayVsyncWait => caps.can_display_query(),
        /*
         * The raw input ring carries every keystroke: an input driver posts
         * under InputSource, input_router alone drains and waits under
         * InputConsumer, and neither Irq nor Admin stands in (K8).
         */
        SyscallNumber::MkInputEventPost => caps.can_input_source(),
        SyscallNumber::MkInputEventDrain => caps.can_input_consumer(),
        SyscallNumber::MkInputEventWait => caps.can_input_consumer(),

        /*
         * Only a SpawnWindow-trusted capsule (the desktop shell) may ask the
         * kernel to open another window instance of an embedded app capsule.
         */
        SyscallNumber::MkSpawnInstance => caps.can_spawn_window(),

        /*
         * Running a baked command-line tool needs only IPC: the tool is spawned
         * parented to the caller so the caller can drive its stdio, and only the
         * baked, attested set can be named.
         */
        SyscallNumber::MkToolRun => caps.can_ipc(),
        /*
         * Saying what a child's streams are on is part of driving its stdio,
         * so it needs what running the child needed. Asking about one's own
         * streams reveals nothing about anyone else.
         */
        SyscallNumber::MkTtySet => caps.can_ipc(),
        SyscallNumber::MkTtyQuery => true,

        _ => return None,
    })
}
