// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the aarch64 speculation decision. `spectre` mirrors the
//! kernel's module so the `super::` paths inside the mounted files resolve.

#[cfg(test)]
mod spectre;

#[cfg(test)]
mod cases;
#[cfg(test)]
mod decide_rest_tests;
#[cfg(test)]
mod decide_tests;
#[cfg(test)]
mod named_more_tests;
#[cfg(test)]
mod named_tests;
#[cfg(test)]
mod word_tests;
