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

#![no_std]
#![no_main]
mod app;
mod audio_client;
mod decode;
mod fetch;
mod library;
#[cfg(feature = "nonos-audio-player-smoketest")]
mod loader;
mod loading;
mod mark;
mod model;
mod peaks;
mod resample;
#[cfg(feature = "nonos-audio-player-smoketest")]
mod selftest;
mod track;
mod track_fmt;
mod track_limit;
mod transport;
mod trouble;
mod ui;
mod volume;
pub mod waveform;
#[cfg(not(feature = "nonos-audio-player-smoketest"))]
use app::PlayerApp;
#[cfg(not(feature = "nonos-audio-player-smoketest"))]
use nonos_app_skeleton::run;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    mark::mark("[PLAYER] up\n");
    #[cfg(feature = "nonos-audio-player-smoketest")]
    selftest::run();
    // A track up to track_limit::MAX_FILE is held whole while it plays, and its
    // buffer doubles past that size while the read grows it, so the default
    // 16 MiB heap ran out on an ordinary song. Sized for the largest track plus
    // the window's caches, before the skeleton would take the default.
    #[cfg(not(feature = "nonos-audio-player-smoketest"))]
    {
        const PLAYER_HEAP: usize = 128 * 1024 * 1024;
        let _ = nonos_libc::heap_init_sized(PLAYER_HEAP);
        run(PlayerApp::new);
    }
}
