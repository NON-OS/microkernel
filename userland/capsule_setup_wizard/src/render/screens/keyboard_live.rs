/*
 * The layout is written to the policy store the moment the keyboard step
 * is confirmed, not at the end of setup: the name and the Wi-Fi passphrase
 * come later and are typed through the PS/2 and USB drivers, which follow
 * the store's layout. Written only at commit, a passphrase with symbols was
 * typed in the US layout and the join failed. The commit writes it again,
 * so a refusal here is still named on the review screen.
 */

/// Write `layout` through `set` when the person leaves the keyboard step
/// forward and there is a store to write to. `None` when nothing was
/// written, else the store's answer.
pub fn write_on_advance(
    advancing: bool,
    port: u32,
    layout: u8,
    set: impl FnOnce(u32, u8) -> Result<(), i32>,
) -> Option<Result<(), i32>> {
    if !advancing || port == 0 {
        return None;
    }
    Some(set(port, layout))
}
