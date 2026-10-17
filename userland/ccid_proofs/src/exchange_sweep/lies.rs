// NONOS Operating System (AGPL-3.0-or-later)
//! The lies the CCID exchange sweep tells, one to each stream.

macro_rules! lies {
    ($($lie:ident),* $(,)?) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum Lie {
            $($lie),*
        }

        pub const LIES: &[Lie] = &[$(Lie::$lie),*];
    };
}

lies!(
    Cut,
    TooLong,
    BadStatus,
    Overtime,
    LateFlood,
    Halt,
    OutFails,
    Silence,
    Trailing,
    ShortWrite,
    PastBuffer,
    ShortHeader
);
