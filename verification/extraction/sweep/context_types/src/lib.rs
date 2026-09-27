// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/context/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/context/types.rs"]
pub mod types;

pub fn processcontext_new(pid: u32, capabilities: u64, page_table_root: u64) -> types::ProcessContext {
    types::ProcessContext::new(pid, capabilities, page_table_root)
}

pub fn processcontext_has_capability(this: types::ProcessContext, cap: u64) -> bool {
    this.has_capability(cap)
}

pub fn executioncontext_is_kernel(this: types::ExecutionContext) -> bool {
    this.is_kernel()
}

pub fn executioncontext_is_process(this: types::ExecutionContext) -> bool {
    this.is_process()
}

