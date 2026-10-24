// NONOS Operating System (AGPL-3.0-or-later)
//! The lies the SCSI sweep tells, one to each device.

macro_rules! lies {
    ($($lie:ident),* $(,)?) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum Lie { $($lie),* }
        pub const LIES: &[Lie] = &[$(Lie::$lie),*];
    };
}

lies!(
    Sound,
    Short10,
    Bogus10,
    Past10,
    Fail10,
    Big,
    Short16Ok,
    Short16Bad,
    Last16Max,
    Fail16,
    Residue16,
    OddLen,
    FlushOk,
    FlushIllegal,
    FlushOther,
    SenseShort,
    SenseFormat,
    SenseFails,
);
