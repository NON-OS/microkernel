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
//! The proofs, each in its own file beside this one.

#[path = "default_tests.rs"]
mod default_tests;
#[path = "download_tests.rs"]
mod download_tests;
#[path = "first_paint_tests.rs"]
mod first_paint_tests;
#[path = "pin_tests.rs"]
mod pin_tests;
#[path = "serve_tests.rs"]
mod serve_tests;
#[path = "wallpaper_backoff_tests.rs"]
mod wallpaper_backoff_tests;
#[path = "../../capsule_wallpaper/src/first_paint/embedded.rs"]
mod wallpaper_embedded;
#[path = "wallpaper_job_tests.rs"]
mod wallpaper_job_tests;
#[path = "../../policy_proto/src/wallpaper_labels.rs"]
mod wallpaper_labels;
#[path = "wallpaper_said_tests.rs"]
mod wallpaper_said_tests;
