// NONOS Operating System (AGPL-3.0-or-later)
//! The SMMUv3's register window is in the broker's forbidden list on
//! aarch64, as VT-d's and AMD-Vi's are on x86_64: the unit the tree
//! describes, in service or not, at least its two register pages long.
//! Read from the kernel's source.

fn src(path: &str) -> String {
    std::fs::read_to_string(format!("../../src/{path}")).unwrap()
}

#[test]
fn the_described_window_is_forbidden() {
    let window = src("arch/aarch64/iommu/smmu/window.rs");
    assert!(window.contains("const MIN_WINDOW: u64 = 0x2_0000;"));
    assert!(window.contains("let (info, _) = DESCRIBED.get()?;"));
    assert!(window.contains("Some((info.base, info.size.max(MIN_WINDOW)))"));
    let arch = src("arch/iommu_window.rs");
    assert!(arch.contains("return crate::arch::aarch64::iommu::smmu::described_window();"));
    let broker = src("hardware/broker/mmio/forbidden_iommu.rs");
    let off_x86 = broker.split("#[cfg(not(target_arch = \"x86_64\"))]").nth(1).unwrap();
    assert!(off_x86.contains("if let Some((base, len)) = crate::arch::iommu_register_window() {"));
    assert!(off_x86.contains("out.push((base, base.saturating_add(len)));"));
    let ranges = src("hardware/broker/mmio/forbidden_ranges.rs");
    assert!(ranges.contains("super::forbidden_iommu::push(&mut out);"));
}
