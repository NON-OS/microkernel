//! View-only accounts, each from a view key somebody shared. They scan like any account, keep
//! their own sealed note log, and have no send: a view key holds no spend secret.

mod context;
mod edit;
mod file;
mod list;
mod watched;
mod watching;

pub(crate) use context::WATCH_CONTEXT;
pub(crate) use watched::Watched;
pub(crate) use watching::Watching;
