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
 * `qwen get TIER...`: every file of each tier fetched from its mirrors and
 * fed to the kernel, which verifies it against the signed pin as it seals
 * it. Ctrl-C at the Terminal ends this program; the kernel keeps how far
 * the file came, and the next `qwen get` goes on from there.
 */

mod anyone;
mod anyone_wait;
mod eta;
mod file;
mod memory_need;
mod mirrors;
mod offer;
mod pour;
mod progress;
mod put;
mod refusal;
mod retry;
mod run;

pub use run::run;
