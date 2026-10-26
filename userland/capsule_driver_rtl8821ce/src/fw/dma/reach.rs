// NONOS Operating System (AGPL-3.0-or-later)

//! Whether the card can reach a DMA region: its buffer descriptors carry
//! 32-bit addresses, so the whole region must end at or below 4 GiB.

/// The first byte a 32-bit descriptor address cannot name.
pub const DMA32_END: u64 = 1 << 32;

/// True when `[device_addr, device_addr + len)` lies wholly below 4 GiB.
pub fn reachable(device_addr: u64, len: usize) -> bool {
    device_addr.checked_add(len as u64).is_some_and(|end| end <= DMA32_END)
}
