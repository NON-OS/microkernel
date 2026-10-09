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

//! The catalog's `server` module, file for file as
//! capsule_wallpaper_catalog/src/server/mod.rs lists it, less `recv` and
//! `runner`: the loop that receives frames and hands each to `serve`.

#[path = "../../capsule_wallpaper_catalog/src/server/handlers/mod.rs"]
pub mod handlers;
#[path = "../../capsule_wallpaper_catalog/src/server/reply.rs"]
pub mod reply;
#[path = "../../capsule_wallpaper_catalog/src/server/respond/mod.rs"]
pub mod respond;
#[path = "../../capsule_wallpaper_catalog/src/server/serve.rs"]
pub mod serve;
