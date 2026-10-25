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

/// How a message a device writes becomes an interrupt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsiKind {
    /// A GICv3 ITS: the device writes an event id to `GITS_TRANSLATER`, and
    /// the ITS turns the pair of its device id and that event into an LPI.
    Its,
    /// A GICv2m frame: the device writes an SPI number to `MSI_SETSPI_NS`.
    /// `spi_count` zero means the node gives no range and the frame's own
    /// `MSI_TYPER` says which SPIs it raises.
    V2m { spi_base: u32, spi_count: u32 },
}

/// An MSI controller the device tree describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MsiController {
    pub kind: MsiKind,
    /// Register window: for an ITS the control frame, then the translation
    /// frame 64 KiB on; for a v2m frame one 4 KiB page.
    pub base: u64,
    pub size: u64,
    /// What `msi-map` and `msi-parent` name it by.
    pub phandle: u32,
}
