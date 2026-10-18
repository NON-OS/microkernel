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

use super::apps_desk::{
    spawn_about, spawn_clock, spawn_file_manager, spawn_snake, spawn_wallet_nonos,
};
use super::apps_media::{
    spawn_audio_player, spawn_install, spawn_process_manager, spawn_video_player,
};
use super::apps_tools::{
    spawn_browser, spawn_calculator, spawn_settings, spawn_terminal, spawn_text_editor,
};
use super::error_name::spawn_error_name;
use super::queue::{self, PendingApp};

/// Perform every queued window-instance spawn. Called from `init_loop`, so
/// this runs in init's context, the same environment the boot spawns use.
/// Each spawn still goes through the full verified, attested pipeline; a
/// rejection (every declared slot already live, or a load error) is logged
/// with its cause and skipped so one bad request cannot wedge the drain.
pub(crate) fn service() {
    for app in queue::take() {
        let result = match app {
            PendingApp::Terminal => spawn_terminal(),
            PendingApp::Browser => spawn_browser(),
            PendingApp::TextEditor => spawn_text_editor(),
            PendingApp::Settings => spawn_settings(),
            PendingApp::Calculator => spawn_calculator(),
            PendingApp::Clock => spawn_clock(),
            PendingApp::About => spawn_about(),
            PendingApp::Snake => spawn_snake(),
            PendingApp::WalletNonos => spawn_wallet_nonos(),
            PendingApp::FileManager => spawn_file_manager(),
            PendingApp::ProcessManager => spawn_process_manager(),
            PendingApp::AudioPlayer => spawn_audio_player(),
            PendingApp::VideoPlayer => spawn_video_player(),
            PendingApp::Install => spawn_install(),
            PendingApp::Prove => crate::userspace::capsule_prove::spawn_prove_instance(),
        };
        match result {
            /*
             * Deliver the focus frame the app skeleton waits for. A freshly
             * spawned instance builds its window on it instead of sitting idle;
             * an app that was already at its window cap restores, raises and
             * focuses the window the user asked for. The spawn decides which of
             * those the pid is, because only it knows whether a slot was free.
             */
            Ok(pid) => super::boot_frame::boot(pid),
            Err(e) => {
                crate::sys::serial::print(b"[SPAWN-INSTANCE] rejected ");
                crate::sys::serial::print(app.name());
                crate::sys::serial::print(b" err=");
                crate::sys::serial::println(spawn_error_name(e));
            }
        }
    }
    queue::settle();
}
