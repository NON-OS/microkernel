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

//! The steps a close takes, logged.

use super::super::super::teardown_steps::Steps;

#[derive(Default)]
pub(super) struct Log {
    pub(super) fail_scene: bool,
    pub(super) fail_release: bool,
    pub(super) calls: Vec<&'static str>,
}

impl Steps for Log {
    fn scene_remove(&mut self) -> bool {
        self.calls.push("scene_remove");
        !self.fail_scene
    }
    fn unsubscribe_input(&mut self) {
        self.calls.push("unsubscribe");
    }
    fn release_surface(&mut self) -> bool {
        self.calls.push("release");
        !self.fail_release
    }
    fn unmap_backing(&mut self) -> bool {
        self.calls.push("unmap");
        true
    }
    fn wm_close(&mut self) -> bool {
        self.calls.push("wm_close");
        true
    }
}
