//! The pinned values of accounts 0, 1 and 2, for an independent implementation to reproduce.

/// Per account: `spend_pk`, `nk`, BLAKE3 of the X-Wing key, the public address, BLAKE3 of `nox1`.
pub(super) const VECTORS: [[&str; 5]; 3] = [
    [
        "96ad146ef260954fdde0675363037cdef506c817161ebe4b74cd18200b6a9a50",
        "ebd827039676a448e59a4d4417ceb082f4081dccd414c874d923c08e5d949436",
        "651465780e9e4fbb7e3e62272b6307b0c6f9cef3743c92cd596c93b3d7bf15b5",
        "0x9858EfFD232B4033E47d90003D41EC34EcaEda94",
        "6573b902ccfde6ee46196232af1b7cab75c13314e52144268db0aff48645fbaa",
    ],
    [
        "a863b3c231d5e0e76342d78f3acdce13aff36e52a3010b1d8677b39945b2771b",
        "9d0d8525b2a841b27392602ecfc1d45f60fece623a43496f57cecf99ac85d86a",
        "198b9bdadb5ce4e05ea7a87ea49d63f8a9046d378ba8f9c3d599ef1a194dfa25",
        "0x6Fac4D18c912343BF86fa7049364Dd4E424Ab9C0",
        "7fadd1693c5c8cca1bfbc00b851546e1d4a226e1342f6a3de724e0927c28320b",
    ],
    [
        "af5330da3404f004da4d525a53a816728ec1de8fb4f36625bfa729f415d73db9",
        "54ac90dc3be3cb3719cec59b5255ce47281b983fc8f1f74f647e3639ceff4f8b",
        "9b2678b4b08245477321b26cf2bbf1db2a8d14ebd62790330b08d51416c2a6de",
        "0xb6716976A3ebe8D39aCEB04372f22Ff8e6802D7A",
        "cebe75655808a55bc252b6d474d9e08b07c372dcb63edba34c9f6cb6abbda64c",
    ],
];
