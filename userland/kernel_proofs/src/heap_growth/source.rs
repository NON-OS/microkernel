// NONOS Operating System (AGPL-3.0-or-later)
//! The kernel's use of the heap growth rules, by its source.

const MAP: &str = include_str!("../../../../src/memory/paging/manager/mapping/map.rs");
const VM: &str = include_str!("../../../../src/kernel_core/init/entry/init_vm_and_protection.rs");
const INIT: &str = include_str!("../../../../src/kernel_core/init/entry/microkernel_init.rs");
const STAGE: &str = include_str!("../../../../src/kernel_core/init/entry/init_heap_region.rs");
const ALLOC: &str = include_str!("../../../../src/memory/heap/types/alloc_impl.rs");
const DEALLOC: &str = include_str!("../../../../src/memory/heap/types/dealloc_impl.rs");
const ROUTE: &str = include_str!("../../../../src/memory/heap/types/region_pick.rs");
const FRONT: &str = include_str!("../../../../src/memory/heap/types/region_alloc.rs");
const PANIC: &str = include_str!("../../../../src/security/zerostate/panic_wipe.rs");
const SANITIZE: &str =
    include_str!("../../../../src/security/hardening/memory_sanitization/api.rs");
const CONSTANTS: &str = include_str!("../../../../src/memory/heap/constants.rs");

#[test]
fn map_page_asks_the_floor_with_the_user_bit_before_it_installs_anything() {
    let ask = MAP.find("user_page_refused(").expect("map_page asks the floor");
    let install = MAP.find("self.install_mapping(").expect("map_page installs");
    let record = MAP.find("self.mappings.insert(").expect("map_page records");
    assert!(ask < install && install < record, "refused before the leaf and the record");
    for needed in ["PagePermissions::USER", "USER_MAP_FLOOR", "FrameAllocationFailed"] {
        assert!(MAP[ask..install].contains(needed), "{needed}");
    }
    assert!(CONSTANTS.contains("pub const USER_MAP_FLOOR: usize = 16 * 1024 * 1024;"));
}

#[test]
fn boot_adds_the_region_after_paging_and_before_any_process() {
    let vm = VM.find("init_unified_vm()").expect("paging comes up");
    let heap = VM.find("init_heap_region::init_heap_region();").expect("then the region");
    assert!(vm < heap);
    assert!(STAGE.contains("crate::memory::heap::init()"));
    let stage = INIT.find("    init_vm_and_protection();").unwrap();
    let procs = INIT.find("    init_process_runtime();").expect("processes come after");
    assert!(stage < procs);
}

#[test]
fn a_block_goes_back_to_the_region_that_handed_it_out() {
    assert!(ALLOC.contains("allocator.raw_alloc(adjusted_layout)"));
    assert!(DEALLOC.contains("allocator.raw_dealloc(raw_ptr, adjusted_layout)"));
    let route = ROUTE.find("if self.in_second(ptr)").expect("a free is routed by address");
    let second = ROUTE.find("self.second.heap.dealloc(ptr, layout)").unwrap();
    let first = ROUTE.find("self.inner.dealloc(ptr, layout)").unwrap();
    assert!(route < second && second < first);
    // A small block goes to its class first; any other to its region.
    assert!(FRONT.contains("self.second.front.push(class, ptr)"));
    assert!(FRONT.contains("self.to_regions(ptr, layout)"));
}

#[test]
fn both_wipes_erase_both_regions() {
    for (name, src) in [("panic", PANIC), ("sanitize", SANITIZE)] {
        assert!(src.contains("get_allocator().extents().into_iter().flatten()"), "{name}");
    }
}
