// NONOS Operating System (AGPL-3.0-or-later)
//! The lies the status sweep tells, one to each command.

macro_rules! lies {
    ($($lie:ident),* $(,)?) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum Lie {
            $($lie),*
        }

        pub const LIES: &[Lie] = &[$(Lie::$lie),*];
    };
}

lies!(Foreign, SensePast, Short, Unknown, Response, ShortResponse, WrongReady, SenseFirst);
