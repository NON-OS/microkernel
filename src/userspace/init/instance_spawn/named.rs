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

/* The handle a shell names an app by, as the app it queues. */

use super::queue::PendingApp;

impl PendingApp {
    /* `None` for a handle that names no app with instance windows. */
    pub fn named(name: &str) -> Option<PendingApp> {
        Some(match name {
            "app.terminal" => PendingApp::Terminal,
            "app.browser" => PendingApp::Browser,
            "app.text_editor" => PendingApp::TextEditor,
            "app.settings" => PendingApp::Settings,
            "app.calculator" => PendingApp::Calculator,
            "app.clock" => PendingApp::Clock,
            "app.about" => PendingApp::About,
            "app.snake" => PendingApp::Snake,
            "app.nonos_wallet" => PendingApp::WalletNonos,
            "app.file_manager" => PendingApp::FileManager,
            "app.process_manager" => PendingApp::ProcessManager,
            "app.audio_player" => PendingApp::AudioPlayer,
            "app.video_player" => PendingApp::VideoPlayer,
            "app.install" => PendingApp::Install,
            "app.prove" => PendingApp::Prove,
            _ => return None,
        })
    }
}
