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

//! The policy this machine attests against, as values.
//!
//! A path gate proves a context is a leaf of a tree, and the tree is named only
//! by its root. A verifier holding a proof, or a prover building one, needs that
//! root, its depth and the epoch that went into the context, or it cannot tell
//! which approved set was meant. This is that record: the kernel tree the boot
//! chain checked, and the capsule tree the spawn gate checks.
//!
//! Every value in it is the same on every machine running the same release, so
//! it names a release and never a machine. It carries no local enrolment: a root
//! a user enrolled on this machine alone would identify it.

pub mod record;

pub use record::{encode, Tree, CAPSULE_PATH_ROOT, KERNEL_CHECKED, RECORD_LEN, RECORD_VERSION};
