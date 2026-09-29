// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/crypto/util/constant_time/barriers.rs"]
pub mod barriers;

#[path = "../../../../../../../../src/crypto/util/constant_time/compare.rs"]
pub mod compare;

pub use barriers::{black_box, black_box_slice, compiler_fence, dummy_work, memory_fence, serialize_execution, time_constant_execute, volatile_read, volatile_write};
pub use compare::{ct_eq, ct_eq_16, ct_eq_32, ct_eq_64, ct_eq_u64, ct_gt_u64, ct_is_nonzero_u64, ct_is_zero_u64, ct_lt_u64};
