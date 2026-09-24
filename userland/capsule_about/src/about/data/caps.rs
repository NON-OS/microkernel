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

pub struct CapDescriptor {
    pub bit: u64,
    pub name: &'static [u8],
    pub role: &'static [u8],
}

// The bit each capability occupies in the mask the kernel records for a process.
// Named once here so no reader anywhere has to count shifts to find out which
// bit is Debug, and so a check on one of them names the same bit the pill list
// draws.

pub const CORE_EXEC: u64 = 1 << 0;
pub const IO: u64 = 1 << 1;
pub const NETWORK: u64 = 1 << 2;
pub const IPC: u64 = 1 << 3;
pub const MEMORY: u64 = 1 << 4;
pub const CRYPTO: u64 = 1 << 5;
pub const FILESYSTEM: u64 = 1 << 6;
pub const HARDWARE: u64 = 1 << 7;
pub const DEBUG: u64 = 1 << 8;
pub const ADMIN: u64 = 1 << 9;
pub const REGISTER_SERVICE: u64 = 1 << 10;
pub const GFX_DISPLAY_QUERY: u64 = 1 << 11;
pub const GFX_SURFACE_CREATE: u64 = 1 << 12;
pub const GFX_SURFACE_MAP: u64 = 1 << 13;
pub const GFX_PRESENT: u64 = 1 << 14;
pub const DEVICE_ENUM: u64 = 1 << 15;
pub const DRIVER: u64 = 1 << 16;
pub const MMIO: u64 = 1 << 17;
pub const IRQ: u64 = 1 << 18;
pub const DMA: u64 = 1 << 19;
pub const PIO: u64 = 1 << 20;

// Reach that lets a capsule drive physical devices without going through a
// broker. Grouped here because it is asked for as one question far more often
// than one bit at a time.
pub const RAW_HARDWARE: u64 = HARDWARE | DRIVER | MMIO | IRQ | DMA | PIO;
pub const ALL_CAPS: &[CapDescriptor] = &[
    CapDescriptor { bit: CORE_EXEC, name: b"CoreExec", role: b"run user code" },
    CapDescriptor { bit: IO, name: b"IO", role: b"raw port + console io" },
    CapDescriptor { bit: NETWORK, name: b"Network", role: b"open sockets" },
    CapDescriptor { bit: IPC, name: b"IPC", role: b"toolkit calls + event recv" },
    CapDescriptor { bit: MEMORY, name: b"Memory", role: b"mmap the paint buffer" },
    CapDescriptor { bit: CRYPTO, name: b"Crypto", role: b"call crypto primitives" },
    CapDescriptor { bit: FILESYSTEM, name: b"FileSystem", role: b"vfs read/write" },
    CapDescriptor { bit: HARDWARE, name: b"Hardware", role: b"raw hardware register access" },
    CapDescriptor { bit: DEBUG, name: b"Debug", role: b"proof markers via MkDebug" },
    CapDescriptor { bit: ADMIN, name: b"Admin", role: b"reboot/shutdown" },
    CapDescriptor {
        bit: REGISTER_SERVICE,
        name: b"RegisterService",
        role: b"publish an IPC endpoint",
    },
    CapDescriptor {
        bit: GFX_DISPLAY_QUERY,
        name: b"GraphicsDisplayQuery",
        role: b"learn display dimensions",
    },
    CapDescriptor {
        bit: GFX_SURFACE_CREATE,
        name: b"GraphicsSurfaceCreate",
        role: b"register the paint surface",
    },
    CapDescriptor {
        bit: GFX_SURFACE_MAP,
        name: b"GraphicsSurfaceMap",
        role: b"map a registered surface",
    },
    CapDescriptor {
        bit: GFX_PRESENT,
        name: b"GraphicsPresent",
        role: b"flush a surface for scanout",
    },
    CapDescriptor { bit: DEVICE_ENUM, name: b"DeviceEnum", role: b"enumerate broker devices" },
    CapDescriptor { bit: DRIVER, name: b"Driver", role: b"claim a broker device" },
    CapDescriptor { bit: MMIO, name: b"Mmio", role: b"map device mmio" },
    CapDescriptor { bit: IRQ, name: b"Irq", role: b"bind a device irq" },
    CapDescriptor { bit: DMA, name: b"Dma", role: b"grant a dma window" },
    CapDescriptor { bit: PIO, name: b"Pio", role: b"raw port io" },
];

// What this capsule's own manifest asks for. It is a declaration, not a
// measurement: the kernel is the only party that knows what was actually
// granted, and the Verify screen holds this constant up against the mask the
// kernel recorded for our pid. If the two ever part, the check fails and says
// so, which is the only way a hardcoded mask earns its place in a build.
pub const MASK: u64 = CORE_EXEC | IPC | MEMORY | GFX_DISPLAY_QUERY | GFX_SURFACE_CREATE;

pub fn is_granted(bit: u64) -> bool {
    MASK & bit != 0
}
