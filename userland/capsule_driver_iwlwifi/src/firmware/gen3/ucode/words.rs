// NONOS Operating System (AGPL-3.0-or-later)

//! The capability and API bitmaps' words, and little-endian reads.

/// `NUM_IWL_UCODE_TLV_CAPA` and `NUM_IWL_UCODE_TLV_API`: 128 bits each.
pub(super) const BITMAP_WORDS: usize = 4;

// A capability or API record: `[word index u32][bits u32]`. A word past the
// 128 bits this driver knows is ignored.
pub(super) fn set_word(words: &mut [u32; BITMAP_WORDS], data: &[u8]) -> Option<()> {
    let index = le32(data, 0)? as usize;
    let bits = le32(data, 4)?;
    if let Some(w) = words.get_mut(index) {
        *w = bits;
    }
    Some(())
}

pub(super) fn test(words: &[u32; BITMAP_WORDS], bit: u32) -> bool {
    words.get((bit / 32) as usize).is_some_and(|w| w & (1 << (bit % 32)) != 0)
}

pub(super) fn le32(d: &[u8], o: usize) -> Option<u32> {
    let b = d.get(o..o.checked_add(4)?)?;
    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}
