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
 * The Linux personality's install of a shipped tier, as it ships: the tier
 * tables, the reasons an install stops with, the model dependency's rules,
 * and the fetcher's exit statuses those rules read, mounted at the paths
 * the personality's own install module gives them.
 */

#[path = "../../../capsule_linux/src/linux/install/apps.rs"]
pub mod apps;
#[path = "../../../capsule_linux/src/linux/install/apps_coder.rs"]
pub mod apps_coder;
#[path = "../../../capsule_linux/src/linux/install/apps_qwen25.rs"]
pub mod apps_qwen25;
#[path = "../../../capsule_linux/src/linux/install/apps_qwen3.rs"]
pub mod apps_qwen3;
#[path = "../../../capsule_model_fetch/src/exit.rs"]
pub mod fetch_exit;
#[path = "../../../capsule_linux/src/linux/install/model_dep.rs"]
pub mod model_dep;
#[path = "../../../capsule_linux/src/linux/install/model_remove.rs"]
pub mod model_remove;
#[path = "../../../capsule_linux/src/linux/install/why.rs"]
pub mod why;

/* How the personality splits a package name, and how the kernel names one for a listing. */
#[path = "../../../capsule_linux/src/linux/file/family.rs"]
pub mod family;
#[path = "../../../../src/userspace/capsule_linux/family.rs"]
pub mod listing;
