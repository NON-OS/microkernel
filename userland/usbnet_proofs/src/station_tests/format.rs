// NONOS Operating System (AGPL-3.0-or-later)
//! A station address has no `Debug`, so no `{:?}` puts it in a log line or
//! a panic message (C5): only `bytes()` reaches the six bytes.

use core::fmt::Debug;
use core::marker::PhantomData;

use crate::station::Station;

struct Probe<T>(PhantomData<T>);

/// What a type without `Debug` answers.
trait Fallback {
    fn has_debug(&self) -> bool {
        false
    }
}

impl<T> Fallback for Probe<T> {}

impl<T: Debug> Probe<T> {
    /// Taken over the trait's answer whenever `T` has `Debug`.
    fn has_debug(&self) -> bool {
        true
    }
}

#[test]
fn a_station_address_cannot_be_formatted() {
    assert!(Probe::<[u8; 6]>(PhantomData).has_debug(), "the probe sees Debug where it is");
    assert!(!Probe::<Station>(PhantomData).has_debug());
}
