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

//! E-63: one model over the whole store op sequence. Keep, forget and reopen
//! in every seeded order, a keyring fault injected at each keep, checked
//! against a reference of each identity's intended level: the recorded level
//! never exceeds the last written (`sweep`), a forgotten identity never comes
//! back on reopen (`sweep`), and the two downgrades a bug would bring — a
//! stale record preferred on reopen, two identities keyed alike — are caught
//! (`mutants`).

mod kit;
mod mutants;
mod run;
mod sweep;
