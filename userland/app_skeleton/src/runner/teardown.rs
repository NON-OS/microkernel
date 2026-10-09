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

use nonos_libc::{mk_munmap, mk_surface_release};

use crate::clients::{compositor, input_router, wm};
use crate::discover::Peers;
use crate::setup::WindowBinding;

use super::request_id::next;
use super::teardown_steps::{take_down, Steps};

struct Calls<'a> {
    peers: &'a Peers,
    window_id: u32,
    binding: &'a WindowBinding,
    request_id: &'a mut u32,
}

impl Steps for Calls<'_> {
    fn scene_remove(&mut self) -> bool {
        compositor::scene_remove(self.peers.compositor, next(self.request_id), 0).is_ok()
    }

    fn unsubscribe_input(&mut self) {
        let _ = input_router::subscribe(self.peers.input_router, next(self.request_id), 0);
    }

    fn release_surface(&mut self) -> bool {
        mk_surface_release(self.binding.surface_handle) >= 0
    }

    fn unmap_backing(&mut self) -> bool {
        mk_munmap(self.binding.backing_va as *mut u8, self.binding.byte_len as usize) >= 0
    }

    fn wm_close(&mut self) -> bool {
        wm::window_close(self.peers.wm, next(self.request_id), self.window_id).is_ok()
    }
}

/// Take a closed window down, in the order teardown_steps.rs gives.
pub(super) fn close(
    peers: &Peers,
    window_id: u32,
    binding: &WindowBinding,
    request_id: &mut u32,
) -> bool {
    take_down(&mut Calls { peers, window_id, binding, request_id })
}
