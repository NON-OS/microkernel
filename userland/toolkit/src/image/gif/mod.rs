mod decoder;
mod frame;
mod header;
mod lzw;
mod lzw_dict;
mod lzw_expand;
mod sub_blocks;
mod to_argb;

pub use decoder::decode_gif_argb8888;
pub use header::gif_dimensions;
