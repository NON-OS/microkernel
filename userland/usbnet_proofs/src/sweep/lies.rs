// NONOS Operating System (AGPL-3.0-or-later)
//! The lies a sweep tells, one to each configuration.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lie {
    ZeroLen,
    OneLen,
    PastEnd,
    Overlong,
    ShortForType,
    ShortTwin,
    EndpointCount,
    InterfaceCount,
    IadCount,
    EndpointField,
}

use Lie::*;
pub const LIES: [Lie; 10] = [
    ZeroLen,
    OneLen,
    PastEnd,
    Overlong,
    ShortForType,
    ShortTwin,
    EndpointCount,
    InterfaceCount,
    IadCount,
    EndpointField,
];
