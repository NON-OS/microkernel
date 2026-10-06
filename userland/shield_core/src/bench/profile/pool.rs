//! Where the prover runs: on every core, as a spend proves, or on a pool of a set size.

use crate::error::WalletError;

/// Run `work` on `threads` threads, 0 for the shared pool a spend proves on, which has one per
/// core. Returns what it gave and the threads it had.
pub fn on<T: Send>(threads: u32, work: impl FnOnce() -> T + Send) -> Result<(T, u32), WalletError> {
    if threads == 0 {
        return Ok((work(), count()));
    }
    let size = usize::try_from(threads).map_err(|_| WalletError::Unavailable)?;
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(size)
        .build()
        .map_err(|_| WalletError::Unavailable)?;
    Ok(pool.install(|| (work(), count())))
}

fn count() -> u32 {
    u32::try_from(rayon::current_num_threads()).unwrap_or(u32::MAX)
}
