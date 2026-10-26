/* qwenchat's image frame and header checks. See qwenvision_frame.h. */
#include "qwenvision_frame.h"

#include <cstring>

ImgWhy img_frame(const unsigned char *head, size_t n, uint32_t &len) {
    len = 0;
    if (n < IMG_HEAD || memcmp(head, IMG_MAGIC, sizeof IMG_MAGIC) != 0) return ImgWhy::NoFrame;
    const uint32_t l = (uint32_t)head[8] | (uint32_t)head[9] << 8 | (uint32_t)head[10] << 16 | (uint32_t)head[11] << 24;
    if (l > IMG_MAX) return ImgWhy::TooLarge;
    len = l;
    return ImgWhy::Ok;
}

static uint32_t be32(const unsigned char *p) {
    return (uint32_t)p[0] << 24 | (uint32_t)p[1] << 16 | (uint32_t)p[2] << 8 | p[3];
}

static uint32_t be16(const unsigned char *p) { return (uint32_t)p[0] << 8 | p[1]; }

/* PNG: the signature, then IHDR first, its width and height big-endian. */
static ImgWhy png(const unsigned char *b, size_t n, uint32_t &w, uint32_t &h) {
    static const unsigned char SIG[8] = {0x89, 'P', 'N', 'G', '\r', '\n', 0x1a, '\n'};
    if (n < 8 || memcmp(b, SIG, 8) != 0) return ImgWhy::NotImage;
    if (n < 24 || be32(b + 8) != 13 || memcmp(b + 12, "IHDR", 4) != 0) return ImgWhy::NoSize;
    w = be32(b + 16), h = be32(b + 20);
    return ImgWhy::Ok;
}

/* A start-of-frame marker: C0 to CF but for DHT (C4), JPG (C8) and DAC (CC). */
static bool sof(unsigned char m) { return m >= 0xc0 && m <= 0xcf && m != 0xc4 && m != 0xc8 && m != 0xcc; }

/* JPEG: SOI, then segments until a start of frame, which carries the sides. */
static ImgWhy jpeg(const unsigned char *b, size_t n, uint32_t &w, uint32_t &h) {
    if (n < 3 || b[0] != 0xff || b[1] != 0xd8 || b[2] != 0xff) return ImgWhy::NotImage;
    size_t i = 2;
    while (i + 4 <= n) {
        if (b[i] != 0xff) return ImgWhy::NoSize;
        const unsigned char m = b[i + 1];
        if (m == 0xff) { i++; continue; }                 /* fill byte */
        if (m == 0xd8 || (m >= 0xd0 && m <= 0xd7) || m == 0x01) { i += 2; continue; } /* no length */
        if (m == 0xd9 || m == 0xda) return ImgWhy::NoSize; /* end, or scan data before any frame */
        const size_t len = be16(b + i + 2);
        if (len < 2 || i + 2 + len > n) return ImgWhy::NoSize;
        if (sof(m)) {
            if (len < 7) return ImgWhy::NoSize;
            h = be16(b + i + 5), w = be16(b + i + 7);
            return ImgWhy::Ok;
        }
        i += 2 + len;
    }
    return ImgWhy::NoSize;
}

ImgWhy img_header(const unsigned char *img, size_t n, ImgKind &kind, uint32_t &w, uint32_t &h) {
    kind = ImgKind::None, w = h = 0;
    if (n == 0) return ImgWhy::Short;
    ImgWhy why;
    if (n >= 1 && img[0] == 0x89) why = png(img, n, w, h), kind = ImgKind::Png;
    else if (n >= 1 && img[0] == 0xff) why = jpeg(img, n, w, h), kind = ImgKind::Jpeg;
    else return ImgWhy::NotImage;
    if (why != ImgWhy::Ok) return kind = ImgKind::None, why;
    if (w == 0 || h == 0) return kind = ImgKind::None, ImgWhy::NoSize;
    if (w > IMG_SIDE_MAX || h > IMG_SIDE_MAX) return kind = ImgKind::None, ImgWhy::TooWide;
    return ImgWhy::Ok;
}

std::string img_refusal(ImgWhy why) {
    switch (why) {
    case ImgWhy::Ok: return "";
    case ImgWhy::NoFrame: return "No image was handed over: give one with qwen TIER QUESTION < FILE.";
    case ImgWhy::TooLarge: return "The image is over 1 MiB, the most the Terminal hands a program; make it smaller.";
    case ImgWhy::Short: return "The image ended before it began; nothing was read.";
    case ImgWhy::NotImage: return "That file is not a PNG or a JPEG, the two kinds this model is shown.";
    case ImgWhy::NoSize: return "The image's header does not say its size, so it was not decoded.";
    case ImgWhy::TooWide: return "The image is over 4096 pixels a side; make it smaller.";
    }
    return "";
}
