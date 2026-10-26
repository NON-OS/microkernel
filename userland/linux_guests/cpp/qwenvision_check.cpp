/*
 * qwenvision_check: a host check of the image frame and header rules
 * (qwenvision_frame.cpp), then a seeded fuzz sweep over both. Each refusal
 * has its own sentence; a frame over 1 MiB, a file that is neither PNG nor
 * JPEG, a header with no size and a side over 4096 are refused before any
 * decoder sees the bytes. The sweep feeds random and mutated inputs and
 * holds every answer to the rules; built with AddressSanitizer by
 * `make nonos-mk-qwenvision-check` (QwenChat.mk), so a read past the input
 * fails it. NONOS_FUZZ_ITERS and NONOS_FUZZ_SEED widen or move the sweep.
 * Exits 0 when every check holds, else names the first that does not.
 */
#include "qwenvision_frame.h"

#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <set>
#include <vector>

static int failed(const char *what) {
    fprintf(stderr, "qwenvision_check: %s\n", what);
    return 1;
}

typedef std::vector<unsigned char> Bytes;

static Bytes frame(uint32_t len) {
    Bytes b(IMG_MAGIC, IMG_MAGIC + 8);
    for (int i = 0; i < 4; i++) b.push_back((unsigned char)(len >> (8 * i)));
    return b;
}

static Bytes png(uint32_t w, uint32_t h) {
    Bytes b = {0x89, 'P', 'N', 'G', '\r', '\n', 0x1a, '\n', 0, 0, 0, 13, 'I', 'H', 'D', 'R'};
    for (uint32_t v : {w, h})
        for (int i = 3; i >= 0; i--) b.push_back((unsigned char)(v >> (8 * i)));
    b.insert(b.end(), {8, 2, 0, 0, 0});
    return b;
}

/* SOI, an APP0 segment, then SOF0 with the sides; `progressive` is SOF2. */
static Bytes jpeg(uint32_t w, uint32_t h, bool progressive = false) {
    Bytes b = {0xff, 0xd8, 0xff, 0xe0, 0, 16, 'J', 'F', 'I', 'F', 0, 1, 1, 0, 0, 1, 0, 1, 0, 0};
    b.insert(b.end(), {0xff, (unsigned char)(progressive ? 0xc2 : 0xc0), 0, 17, 8,
                       (unsigned char)(h >> 8), (unsigned char)h, (unsigned char)(w >> 8), (unsigned char)w, 3});
    b.insert(b.end(), 9, 0);
    return b;
}

static ImgWhy head(const Bytes &b, uint32_t *w = nullptr, uint32_t *h = nullptr) {
    ImgKind k;
    uint32_t ww, hh;
    const ImgWhy why = img_header(b.data(), b.size(), k, ww, hh);
    if (w) *w = ww;
    if (h) *h = hh;
    return why;
}

static int fixed() {
    uint32_t len = 7;
    Bytes f = frame(1234);
    if (img_frame(f.data(), f.size(), len) != ImgWhy::Ok || len != 1234) return failed("a frame's length");
    if (img_frame(frame(0).data(), 12, len) != ImgWhy::Ok || len != 0) return failed("a frame with no image");
    if (img_frame(frame(IMG_MAX).data(), 12, len) != ImgWhy::Ok) return failed("a frame of exactly 1 MiB");
    if (img_frame(frame(IMG_MAX + 1).data(), 12, len) != ImgWhy::TooLarge || len != 0) return failed("over 1 MiB");
    f[3] = 'X';
    if (img_frame(f.data(), f.size(), len) != ImgWhy::NoFrame) return failed("a wrong magic");
    if (img_frame(frame(1).data(), 11, len) != ImgWhy::NoFrame) return failed("a short head");
    uint32_t w, h;
    if (head(png(640, 480), &w, &h) != ImgWhy::Ok || w != 640 || h != 480) return failed("a PNG's sides");
    if (head(jpeg(800, 600), &w, &h) != ImgWhy::Ok || w != 800 || h != 600) return failed("a JPEG's sides");
    if (head(jpeg(32, 16, true), &w, &h) != ImgWhy::Ok || w != 32) return failed("a progressive JPEG");
    if (head(png(4096, 4096)) != ImgWhy::Ok) return failed("4096 a side");
    if (head(png(4097, 1)) != ImgWhy::TooWide || head(jpeg(1, 4097)) != ImgWhy::TooWide) return failed("a side over 4096");
    if (head(png(0, 5)) != ImgWhy::NoSize || head(jpeg(5, 0)) != ImgWhy::NoSize) return failed("a zero side");
    if (head(Bytes{'G', 'I', 'F', '8'}) != ImgWhy::NotImage) return failed("a GIF");
    if (head(Bytes{}) != ImgWhy::Short) return failed("nothing");
    Bytes cut = png(10, 10);
    cut.resize(20);
    if (head(cut) != ImgWhy::NoSize) return failed("a PNG cut before its sides");
    Bytes scan = {0xff, 0xd8, 0xff, 0xda, 0, 2};
    if (head(scan) != ImgWhy::NoSize) return failed("a JPEG scan before any frame");
    Bytes overrun = {0xff, 0xd8, 0xff, 0xe0, 0x7f, 0xff, 0};
    if (head(overrun) != ImgWhy::NoSize) return failed("a segment longer than the file");
    std::set<std::string> said;
    for (ImgWhy why : {ImgWhy::NoFrame, ImgWhy::TooLarge, ImgWhy::Short, ImgWhy::NotImage, ImgWhy::NoSize, ImgWhy::TooWide}) {
        const std::string s = img_refusal(why);
        if (s.empty() || s.back() != '.' || !said.insert(s).second) return failed("a refusal sentence");
    }
    if (!img_refusal(ImgWhy::Ok).empty()) return failed("a sentence for Ok");
    return 0;
}

/* xorshift64*: the sweep is the same for the same seed. */
static uint64_t next(uint64_t &s) {
    s ^= s >> 12, s ^= s << 25, s ^= s >> 27;
    return s * 2685821657736338717ull;
}

static int sweep(uint64_t seed, long iters) {
    const Bytes seeds[] = {png(64, 48), jpeg(320, 200), jpeg(9, 9, true), frame(77)};
    for (long i = 0; i < iters; i++) {
        Bytes b;
        if (next(seed) % 4 == 0) {
            b.resize(next(seed) % 64);
            for (auto &c : b) c = (unsigned char)next(seed);
        } else {
            b = seeds[next(seed) % 4];
            for (int k = (int)(next(seed) % 4); k >= 0 && !b.empty(); k--) b[next(seed) % b.size()] = (unsigned char)next(seed);
            b.resize(next(seed) % (b.size() + 1));
        }
        /* An exact-size copy, so a read past the end is a fault under ASan. */
        unsigned char *p = (unsigned char *)malloc(b.size() ? b.size() : 1);
        if (!b.empty()) memcpy(p, b.data(), b.size());
        ImgKind k;
        uint32_t w, h, len;
        const ImgWhy why = img_header(p, b.size(), k, w, h);
        const ImgWhy fw = img_frame(p, b.size(), len);
        free(p);
        if (why == ImgWhy::Ok && (k == ImgKind::None || !w || !h || w > IMG_SIDE_MAX || h > IMG_SIDE_MAX))
            return failed("the sweep: an Ok image outside the rules");
        if (why != ImgWhy::Ok && k != ImgKind::None) return failed("the sweep: a refused image with a kind");
        if (fw == ImgWhy::Ok && len > IMG_MAX) return failed("the sweep: a frame over 1 MiB taken");
        if (fw != ImgWhy::Ok && len != 0) return failed("the sweep: a refused frame with a length");
    }
    return 0;
}

int main() {
    if (int r = fixed()) return r;
    const char *it = getenv("NONOS_FUZZ_ITERS"), *sd = getenv("NONOS_FUZZ_SEED");
    const long iters = it ? atol(it) : 200000;
    const uint64_t seed = sd ? strtoull(sd, nullptr, 10) | 1 : 0x9e3779b97f4a7c15ull;
    if (int r = sweep(seed, iters)) return r;
    printf("qwenvision_check: frame and header rules hold, %ld fuzzed inputs\n", iters);
    return 0;
}
