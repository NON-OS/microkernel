//! Peak memory, where the platform reports it.

/// Peak resident memory in kibibytes. Linux keeps a high water mark in `/proc`. iOS needs a Mach
/// call that belongs in the shell, so the core returns nothing there and never guesses.
pub fn peak_kib() -> Option<u64> {
    if cfg!(target_os = "android") || cfg!(target_os = "linux") {
        let status = std::fs::read_to_string("/proc/self/status").ok()?;
        for line in status.lines() {
            let Some(rest) = line.strip_prefix("VmHWM:") else {
                continue;
            };
            let kib = rest.trim().trim_end_matches(" kB").trim();
            return kib.parse().ok();
        }
    }
    None
}
