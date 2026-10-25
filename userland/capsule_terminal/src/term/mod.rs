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

pub mod banner;
pub mod block;
pub mod context;
pub mod complete_list;
pub mod cwd;
pub mod dimensions;
pub mod dur;
pub mod git_probe;
pub mod handed;
pub mod history;
pub mod identity;
pub mod line;
pub mod manifest;
pub mod prefs;
pub mod prompt;
pub mod prompt_bits;
pub mod rtc;
pub mod scrollback;
pub mod search;
pub mod search_step;
pub mod select;
pub mod state;
pub mod terminal;
pub mod theme;
pub mod theme_pick;
pub mod util;

#[cfg(not(feature = "agentproof-headless"))]
pub use terminal::Terminal;
