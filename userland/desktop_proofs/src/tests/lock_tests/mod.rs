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

//! The shell's lock screen and passphrase prompts (state/lock/), and where
//! the shell hooks them in: what each key does, what each keyring answer
//! does, the verifier's standing, the cover over every output, and that
//! nothing else of the shell sees input or draws while it is up.

mod cover_tests;
mod hooks_tests;
mod prompt_tests;
mod released_tests;
mod screen_tests;
mod tick_band_tests;
mod verifier_tests;
