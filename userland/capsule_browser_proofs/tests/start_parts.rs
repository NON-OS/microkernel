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

//! The start page's session badge and cards, drawn from shared widgets,
//! at 1x and 2x: the badge sits in the hero's corner clear of the greeting
//! and is left out when they would meet, and every card holds its lines
//! under its head and inside its body, however many the session shows.

#[path = "start_parts/badge_tests.rs"]
mod badge_tests;
#[path = "start_parts/card_tests.rs"]
mod card_tests;
#[path = "start_parts/kit.rs"]
mod kit;
