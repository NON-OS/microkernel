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

//! What an install carries from the running system into the new disk's
//! store: first-boot setup's answers, so the installed system does not ask
//! again, and the signed programs the running system holds under
//! `/capsules/` and `/linux/`, so it runs what this one runs. The Linux
//! guests the Qwen tiers start, `/linux/bin/qwenchat` and its builds, are
//! among those when this boot's store carries them.

mod carried;
mod gather;
mod sets;
mod setup;
mod source;

pub use carried::Carried;
pub use gather::gather;
pub use source::CarrySource;
