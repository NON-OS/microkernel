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

//! Host stand-in for the capsule's baseline.rs: an image built with no catalogue.
//
// The shipped static embeds target/market/index.bin, which is empty when the
// build had no operator seed. Empty here too, so load hands ingest only the
// operator's file and every blob a test sees is that file.

pub(crate) static BASELINE: &[u8] = &[];
