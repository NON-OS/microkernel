// NONOS Operating System (AGPL-3.0-or-later)
//! The HBA's reach into memory, through the IOMMU: a bus address names host
//! memory only inside a DMA grant the broker still holds. Anything else, a
//! user VA written where a bus address belongs, a dead driver's frame, a
//! grant already given back, is a fault the test sees: the access panics.

use core::mem::size_of;

/// The host address of `len` bytes at bus address `bus`.
pub fn host(bus: u64, len: u64) -> u64 {
    let end = bus.checked_add(len);
    let inside =
        |&(b, _, n): &(u64, usize, usize)| bus >= b && end.is_some_and(|e| e <= b + n as u64);
    match nonos_libc::live_dma().into_iter().find(inside) {
        Some((b, h, _)) => h as u64 + (bus - b),
        None => panic!("the HBA reached {len} bytes at {bus:#x}, outside every live DMA grant"),
    }
}

/// The host address of a `T` at bus address `bus`.
pub fn host_of<T>(bus: u64) -> u64 {
    host(bus, size_of::<T>() as u64)
}
