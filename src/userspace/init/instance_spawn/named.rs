/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

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
            _ => return None,
        })
    }
}
