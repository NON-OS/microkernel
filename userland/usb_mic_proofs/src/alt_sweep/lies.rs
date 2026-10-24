// NONOS Operating System (AGPL-3.0-or-later)
//! The lies the alternate sweep tells, one to each function.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cut {
    General,
    Format,
    RatesPast,
    Endpoint,
    Protocol,
    Interface,
    Class,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lie {
    Sound,
    Bytes(Cut),
    Bulk,
    Feedback,
}

pub const LIES: [Lie; 10] = [
    Lie::Sound,
    Lie::Bytes(Cut::General),
    Lie::Bytes(Cut::Format),
    Lie::Bytes(Cut::RatesPast),
    Lie::Bytes(Cut::Endpoint),
    Lie::Bytes(Cut::Protocol),
    Lie::Bytes(Cut::Interface),
    Lie::Bytes(Cut::Class),
    Lie::Bulk,
    Lie::Feedback,
];
