// NONOS Operating System (AGPL-3.0-or-later)
//! The lies the NTB sweep tells, one to each block, and the list of them.

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
    NthSign,
    NthLen,
    Ntb32NotOffered,
    BlockPastTransfer,
    BlockOverRx,
    FirstInHeader,
    NdpLenShort,
    NdpPastBlock,
    NdpSign,
    NdpLoop,
    NextPastBlock,
    ZeroIndex,
    ShortDatagram,
    DatagramPastBlock,
    DatagramWraps,
    TooMany,
    NextInHeader
);
