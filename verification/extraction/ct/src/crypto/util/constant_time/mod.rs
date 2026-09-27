// NONOS Operating System (AGPL-3.0-or-later)
// The comparison, selection, lookup and arithmetic primitives from
// src/crypto/util/constant_time, and the barriers module they read their fences
// from. The re-exports mirror the real module's, because each of these files
// reaches for the others through `super`.
#[path = "../../../../../../../src/crypto/util/constant_time/barriers.rs"]
pub mod barriers;

#[path = "../../../../../../../src/crypto/util/constant_time/compare.rs"]
pub mod compare;

#[path = "../../../../../../../src/crypto/util/constant_time/select.rs"]
pub mod select;

#[path = "../../../../../../../src/crypto/util/constant_time/lookup.rs"]
pub mod lookup;

#[path = "../../../../../../../src/crypto/util/constant_time/math.rs"]
pub mod math;

pub use barriers::{compiler_fence, volatile_read};
pub use compare::{ct_eq_u64, ct_gt_u64, ct_is_nonzero_u64, ct_is_zero_u64, ct_lt_u64};
pub use select::{ct_select_u32, ct_select_u64};
