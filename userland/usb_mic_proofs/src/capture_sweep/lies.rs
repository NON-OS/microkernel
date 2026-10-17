// NONOS Operating System (AGPL-3.0-or-later)
//! The lies the capture sweep tells, one to each run.

macro_rules! lies {
    ($($lie:ident),* $(,)?) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum Lie {
            $($lie),*
        }

        pub const LIES: &[Lie] = &[$(Lie::$lie),*];
    };
}

lies!(Sound, PartFrame, OverShare, CountOver, CountUnder, LenPast, ShortHeader, Flood);
