// NONOS Operating System (AGPL-3.0-or-later)
//! The block's `Default`, the proofs' alone: the kernel's statics need its
//! const `new`, and nothing in the kernel wants another way to make one.

use super::types::PerCpuData;

impl Default for PerCpuData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::PerCpuData;
    use core::sync::atomic::Ordering::Relaxed;

    #[test]
    fn the_default_block_is_the_one_new_makes() {
        let (d, n) = (PerCpuData::default(), PerCpuData::new());
        assert_eq!((d.self_ptr, d.cpu_id, d.apic_id), (n.self_ptr, n.cpu_id, n.apic_id));
        assert_eq!(d.active_asid.load(Relaxed), n.active_asid.load(Relaxed));
    }
}
