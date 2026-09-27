// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/boot/types_context.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/boot/types_context.rs"]
pub mod types_context;

pub fn exceptioncontext_instruction_pointer(this: types_context::ExceptionContext) -> u64 {
    this.instruction_pointer()
}

pub fn exceptioncontext_stack_pointer(this: types_context::ExceptionContext) -> u64 {
    this.stack_pointer()
}

pub fn exceptioncontext_code_segment(this: types_context::ExceptionContext) -> u64 {
    this.code_segment()
}

pub fn exceptioncontext_is_user_mode(this: types_context::ExceptionContext) -> bool {
    this.is_user_mode()
}

pub fn exceptioncontext_is_kernel_mode(this: types_context::ExceptionContext) -> bool {
    this.is_kernel_mode()
}

pub fn exceptioncontext_has_error_code(this: types_context::ExceptionContext) -> bool {
    this.has_error_code()
}

