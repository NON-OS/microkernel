// NONOS Operating System (AGPL-3.0-or-later)
//! The lies the PACKET_MSG sweep tells, one to each transfer.

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
    WrongType,
    LenShort,
    LenPastTransfer,
    OffsetLow,
    FrameShort,
    FrameLong,
    FramePastMessage,
    OffsetWraps,
    TrailingJunk
);
