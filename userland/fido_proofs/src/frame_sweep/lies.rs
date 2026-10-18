// NONOS Operating System (AGPL-3.0-or-later)
//! The lies the CTAPHID sweep tells, one to each stream.

macro_rules! lies {
    ($($lie:ident),* $(,)?) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum Lie {
            $($lie),*
        }

        pub const LIES: &[Lie] = &[$(Lie::$lie),*];
    };
}

lies!(SeqSkip, Reopen, LengthPast, ShortReport, CutOff, BadKeepalive, ErrorOk, ErrorBadLength);
