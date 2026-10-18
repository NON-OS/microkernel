// NONOS Operating System (AGPL-3.0-or-later)
//! The order the kernel keeps, read from its source: the uncached block is
//! written out before its entry breaks, the new entry is Non-cacheable, the
//! identity alias goes; an uncached grant for a device that does not snoop
//! comes from the uncached pool with no fallback.

const BLOCK: &str = include_str!("../../../src/arch/aarch64/mmu/uncached_block.rs");
const ALLOC: &str = include_str!("../../../src/hardware/broker/dma/map/alloc.rs");
const UNCACHED: &str = include_str!("../../../src/hardware/broker/dma/map/uncached.rs");

fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("{needle} missing"))
}

#[test]
fn the_block_is_broken_before_it_is_made_and_made_uncached() {
    let flush = at(BLOCK, "flush_and_fence(direct, BLOCK_2M);");
    let clear = at(BLOCK, "high.clear_entry(l2);");
    let tlbi = at(BLOCK, "tlb::flush_tlb_page(direct);");
    let make = at(BLOCK, "high.set_block(l2, pa, &UNCACHED);");
    assert!(flush < clear && clear < tlbi && tlbi < make);
    assert!(BLOCK.contains("PageAttributes::new(MemoryType::NormalNC, true, false, false,"));
    let identity = at(BLOCK, "low.clear_entry(l2);");
    assert!(make < identity && !BLOCK[identity..].contains("set_block"));
}

#[test]
fn an_uncached_grant_for_a_non_snooping_device_has_one_source() {
    let body: String = ALLOC.split_whitespace().collect();
    assert!(body.contains("ifuncached_frames{pool::uncached_alloc(pagesasusize)}else{take("));
    assert!(!body.contains("uncached_alloc(pagesasusize).or"), "a fallback to cacheable frames");
    let rule: String = UNCACHED.split_whitespace().collect();
    assert!(rule.contains("flags&(DMA_MAP_COHERENT|DMA_MAP_WC)!=0"));
    assert!(rule.contains("&&!crate::hardware::broker::table::dma_coherent(device_id)"));
}
