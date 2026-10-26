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

//! The shipped vision tier, Qwen3-VL 2B Instruct: the model and its
//! projector, shown an image the Terminal hands it with `< FILE`.

use super::apps::{vision_tier, App};

pub const VL: &[App] = &[vision_tier!("qwen-vl-2b", "/Qwen3VL-2B-Instruct-Q4_K_M.gguf", "/mmproj-Qwen3VL-2B-Instruct-Q8_0.gguf")];
