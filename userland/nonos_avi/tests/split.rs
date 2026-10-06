use nonos_avi::{movi_span, u32_at, AviError, AviFile};

const CLIP: &[u8] = include_bytes!("../../capsule_video_player/assets/clip.avi");

/// The clip's `idx1` data, found where `movi` ends, as the player reads it.
fn idx1_of(file: &[u8]) -> &[u8] {
    let end = movi_span(file).expect("movi").end as usize;
    assert_eq!(&file[end..end + 4], b"idx1", "the clip keeps idx1 straight after movi");
    let size = u32_at(file, end + 4).expect("size") as usize;
    &file[end + 8..end + 8 + size]
}

#[test]
fn movi_span_points_past_the_frames() {
    let span = movi_span(CLIP).expect("movi");
    assert_eq!(&CLIP[span.data_pos as usize..span.data_pos as usize + 4], b"movi");
    assert!(span.end as usize <= CLIP.len());
}

#[test]
fn a_head_without_the_index_parses_with_it_read_apart() {
    let whole = AviFile::parse(CLIP).expect("fixture must parse");
    let first = &whole.index[0];
    // Only the headers and the first frame, as a long film's head holds.
    let head = &CLIP[..(first.offset + first.len as u64) as usize];
    assert!(matches!(AviFile::parse(head), Err(AviError::NoIndex)));
    let parts = AviFile::parse_parts(head, idx1_of(CLIP)).expect("parts parse");
    assert_eq!(parts.index.len(), whole.index.len());
    for (a, b) in parts.index.iter().zip(whole.index.iter()) {
        assert_eq!((a.offset, a.len), (b.offset, b.len));
    }
    assert_eq!((parts.video.width, parts.video.height), (whole.video.width, whole.video.height));
    assert_eq!(parts.header.micro_sec_per_frame, whole.header.micro_sec_per_frame);
}

#[test]
fn parts_refuse_what_parse_refuses() {
    let idx1 = idx1_of(CLIP);
    assert!(matches!(AviFile::parse_parts(b"RIFX\0\0\0\0AVI ", idx1), Err(AviError::NotRiff)));
    assert!(matches!(AviFile::parse_parts(b"RIFF\0\0\0\0WAVE", idx1), Err(AviError::NotAvi)));
    assert!(AviFile::parse_parts(&CLIP[..40], idx1).is_err());
    assert!(AviFile::parse_parts(CLIP, &[]).is_err(), "an empty index has no frames");
}

#[test]
fn no_panic_on_any_truncated_head() {
    let idx1 = idx1_of(CLIP);
    for n in 0..CLIP.len().min(4096) {
        let _ = AviFile::parse_parts(&CLIP[..n], idx1);
        let _ = movi_span(&CLIP[..n]);
    }
}
