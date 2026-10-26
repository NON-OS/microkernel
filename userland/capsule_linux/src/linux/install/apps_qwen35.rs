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

//! The shipped tiers of Qwen3.5 and Qwen3.6: Qwen3.5 0.8B and Qwen3.6 27B,
//! both dense hybrids that attend in one layer in four.

use super::apps::{tier, App};

pub const QWEN35: &[App] = &[
    tier!("qwen-qwen3.5-0.8b", "/Qwen3.5-0.8B-Q8_0.gguf"),
    tier!("qwen-qwen3.6-27b", "/Qwen3.6-27B-Q4_K_M.gguf"),
];
