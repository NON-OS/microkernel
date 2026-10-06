extern crate alloc;
use crate::viewer::budget::{refuse_bytes, READ_LIMIT};
use crate::viewer::says::{decode_reason, read_again};
use crate::viewer::state::ViewerState;
use crate::viewer::viewport::View;
use crate::viewer::{decode, ext, flip, rotate};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use nonos_app_skeleton::clients::vfs::{list_paths, read_file};

/// Read and decode `path` into the single view, leaving the list that prev and
/// next walk as it is: the gallery's when opened from there, the folder's
/// when opened by path.
pub fn open_path(st: &mut ViewerState, path: &str) {
    st.view = View { zoom: 1.0, pan_x: 0.0, pan_y: 0.0 };
    // The image on screen goes before the next is read and decoded, so the two
    // are never in the heap at once.
    st.img = None;
    // Not the last file's size: a file that could not be read has none.
    st.file_size = 0;
    let bytes = match read_with_retry(st.owner_pid, path.as_bytes(), READ_LIMIT) {
        Ok(b) => b,
        Err(e) => {
            st.status = err_line(path, e);
            return;
        }
    };
    if let Some(why) = refuse_bytes(bytes.len() as u64) {
        st.status = err_line(path, why);
        return;
    }
    st.file_size = bytes.len() as u64;
    match decode::decode(&bytes, path.as_bytes()) {
        Ok(d) => {
            st.img = Some(d);
            st.status = String::new();
        }
        Err(e) => {
            st.img = None;
            st.status = err_line(path, e);
        }
    }
}

fn read_with_retry(owner_pid: u32, path: &[u8], max: u32) -> Result<Vec<u8>, &'static str> {
    let mut attempt = 0u32;
    let began = uptime();
    loop {
        match read_file(owner_pid, path, max) {
            Ok(b) => return Ok(b),
            Err(e) => {
                attempt += 1;
                if !read_again(attempt, uptime().saturating_sub(began), e) {
                    return Err(e);
                }
                for _ in 0..48 {
                    let _ = nonos_libc::mk_yield();
                }
            }
        }
    }
}

/// Open a path handed in from outside (the desktop shell), with the images in
/// its folder as the list prev and next walk, whether or not it decoded.
pub fn open_in_folder(st: &mut ViewerState, path: &str) {
    open_path(st, path);
    build_dir(st, path);
}

/// Step to the next or previous image. With one image or none there is
/// nothing to step to, and the one on screen is not read again.
pub fn step(st: &mut ViewerState, delta: i32) {
    if st.dir.len() <= 1 {
        return;
    }
    let n = st.dir.len() as i32;
    st.idx = (((st.idx as i32 + delta) % n + n) % n) as usize;
    let p = st.dir[st.idx].clone();
    open_path(st, &p);
}

pub fn rotate(st: &mut ViewerState) {
    if let Some(img) = st.img.as_mut() {
        let (px, w, h) = rotate::rotate_cw(&img.px, img.w, img.h);
        img.px = px;
        img.w = w;
        img.h = h;
    }
}

pub fn flip_h(st: &mut ViewerState) {
    if let Some(img) = st.img.as_mut() {
        img.px = flip::flip_h(&img.px, img.w, img.h);
    }
}

pub fn flip_v(st: &mut ViewerState) {
    if let Some(img) = st.img.as_mut() {
        img.px = flip::flip_v(&img.px, img.w, img.h);
    }
}

fn build_dir(st: &mut ViewerState, path: &str) {
    let (dir, _file) = split_parent(path);
    let paths = match list_paths(st.owner_pid, dir.as_bytes()) {
        Ok(paths) => paths,
        Err(e) => {
            // The image is still shown and still counted; only stepping is
            // lost, and the window says why rather than hiding the buttons.
            st.dir = alloc::vec![String::from(path)];
            st.idx = 0;
            if st.status.is_empty() {
                st.status = String::from("Previous and next are not available: ");
                st.status.push_str(decode_reason(e));
            }
            return;
        }
    };
    let mut imgs: Vec<String> = Vec::new();
    for p in paths {
        let Some(rest) = p.strip_prefix(dir.as_str()) else { continue };
        if rest.is_empty() || rest.contains('/') {
            continue;
        }
        if ext::is_codec_image(p.as_bytes()) {
            imgs.push(p);
        }
    }
    imgs.sort();
    // The listing can miss the file that was just opened (a name the filter
    // does not take as an image); it is still the one on screen.
    let idx = match imgs.iter().position(|p| p == path) {
        Some(i) => i,
        None => {
            imgs.push(String::from(path));
            imgs.len() - 1
        }
    };
    st.idx = idx;
    st.dir = imgs;
}

fn split_parent(path: &str) -> (String, String) {
    match path.rfind('/') {
        Some(i) => (path[..=i].to_string(), path[i + 1..].to_string()),
        None => ("/".to_string(), path.to_string()),
    }
}

fn err_line(path: &str, e: &'static str) -> String {
    let mut s = String::from("Can't display ");
    s.push_str(path);
    s.push_str(": ");
    s.push_str(decode_reason(e));
    s
}

fn uptime() -> u64 {
    nonos_libc::mk_uptime_ms().max(0) as u64
}
