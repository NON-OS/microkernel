/*
 * qwenchat's image frame: how a vision tier is handed an image, and what it
 * holds the image to before any decoder sees it. The Terminal sends a frame
 * on a vision tier's stdin ahead of the question: the magic "NXIMG01\n", a
 * little-endian u32 length, then that many bytes, zero for no image. The
 * bytes are untrusted: the frame and the image's header are read here,
 * bounded, so the decoder is handed only a PNG or a JPEG of known size.
 */
#pragma once
#include <cstddef>
#include <cstdint>
#include <string>

static const char IMG_MAGIC[8] = {'N', 'X', 'I', 'M', 'G', '0', '1', '\n'};
static const size_t IMG_HEAD = 12;           /* the magic and the length */
static const uint32_t IMG_MAX = 1u << 20;    /* the Terminal's < cap, 1 MiB */
static const uint32_t IMG_SIDE_MAX = 4096;   /* pixels a side */

enum class ImgKind { None, Png, Jpeg };

/* Why an image is refused; Ok when it may be decoded. */
enum class ImgWhy { Ok, NoFrame, TooLarge, Short, NotImage, NoSize, TooWide };

/* The frame's length from its 12-byte head, or NoFrame / TooLarge. */
ImgWhy img_frame(const unsigned char *head, size_t n, uint32_t &len);
/* The image's kind and its sides, read from its header alone. */
ImgWhy img_header(const unsigned char *img, size_t n, ImgKind &kind, uint32_t &w, uint32_t &h);
/* One sentence for a person, naming the limit that refused it. */
std::string img_refusal(ImgWhy why);
