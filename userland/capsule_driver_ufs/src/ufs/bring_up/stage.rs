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

//! The bring-up's stages, in the order each needs the one before: a host
//! that has initialised, DMA it can reach, a link with a device on it, the
//! queues running (requests travel through them), the device answering at
//! the UTP layer, its own initialisation through, then its logical units.
//! Every stage names itself in the line a failure logs.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Host,
    Dma,
    Link,
    Queues,
    Answers,
    Initialised,
    Luns,
    Units,
}

impl Stage {
    pub fn name(self) -> &'static [u8] {
        match self {
            Stage::Host => b"host enable",
            Stage::Dma => b"DMA",
            Stage::Link => b"link startup",
            Stage::Queues => b"transfer lists",
            Stage::Answers => b"NOP OUT",
            Stage::Initialised => b"fDeviceInit",
            Stage::Luns => b"REPORT LUNS",
            Stage::Units => b"logical units",
        }
    }
}
