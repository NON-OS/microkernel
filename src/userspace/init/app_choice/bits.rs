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

/*
 * One bit per app setup may turn off; a set bit is off. The same bits as
 * nonos_policy_proto::apps, which setup and the desktop shell read.
 */

pub(crate) const BROWSER: u32 = 1 << 0;
pub(crate) const WALLET: u32 = 1 << 1;
/* The store window. The package service behind it is not covered. */
pub(crate) const STORE: u32 = 1 << 2;
pub(crate) const FILES: u32 = 1 << 3;
pub(crate) const EDITOR: u32 = 1 << 4;
pub(crate) const CALCULATOR: u32 = 1 << 5;
/* The music player, the video player and the image viewer together. */
pub(crate) const MEDIA: u32 = 1 << 6;
/* The Linux personality, and so Qwen and every Linux package it runs. */
pub(crate) const LINUX: u32 = 1 << 7;
