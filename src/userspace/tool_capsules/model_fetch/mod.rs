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
 * The model fetcher, `tool.model-fetch`: `qwen get` and `qwen tiers` from the
 * Terminal. First-party, so it is not in `userland/apps.list`; it is spawned
 * on demand, parented to the Terminal that asked, and holds the network and
 * StreamImport, the right to feed a pinned file into the data volume. The
 * only other holder is `nonos.prove`, which leaves its proof on the volume.
 */

mod embed;
mod refusal;
mod spawn;

pub(super) use spawn::{run_for_caller, TOOL};
