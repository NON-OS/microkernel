// NONOS Operating System (AGPL-3.0-or-later)
//! The lies the clock sweep tells, one to each function.

macro_rules! lies {
    ($($lie:ident),* $(,)?) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum Lie {
            $($lie),*
        }

        pub const LIES: &[Lie] = &[$(Lie::$lie),*];
    };
}

lies!(Sound, DupId, ZeroId, Cut, NoPins, Chain, Multiplier, Missing, Foreign);
