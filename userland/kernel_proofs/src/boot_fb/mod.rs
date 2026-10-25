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

//! The boot framebuffer's holder (broker/table/boot_fb_rule.rs): while a
//! claimed device's memory window contains the frame the loader handed over,
//! the GOP present answers EBUSY; once the claim goes, GOP is writable again.

#[path = "../../../../src/hardware/broker/table/boot_fb_rule.rs"]
mod boot_fb_rule;

#[cfg(test)]
mod tests;
