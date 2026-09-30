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
 * The steps in the order setup shows them; theme::STEP_LABELS names each
 * one, in the same order. DONE is one past the last.
 */

pub const KEYBOARD: u8 = 0;
pub const NAME: u8 = 1;
pub const TIME_ZONE: u8 = 2;
pub const MODE: u8 = 3;
pub const NETWORK: u8 = 4;
pub const PRIVACY: u8 = 5;
pub const APPEARANCE: u8 = 6;
pub const QWEN: u8 = 7;
pub const APPS: u8 = 8;
pub const LOCAL_SOFTWARE: u8 = 9;
pub const HOST: u8 = 10;
pub const REVIEW: u8 = 11;
pub const DONE: u8 = 12;

const _: () = assert!(crate::render::theme::STEP_LABELS.len() == DONE as usize);
