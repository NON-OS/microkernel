/*
 * qwenchat: a vision tier's image, read from its frame, held to the frame
 * and header rules (qwenvision_frame.cpp), and shown to the model ahead of
 * the person's first question. The image comes from the Terminal's
 * `< FILE`, never from a path the program opens, and is wiped with the
 * conversation. See qwenchat.h.
 */
#include "qwenchat.h"
#include "qwenvision.h"
#include "qwenvision_frame.h"

#include "mtmd-helper.h"
#include "mtmd.h"

#include <cerrno>
#include <cstdio>
#include <cstring>

/* Exactly n bytes from stdin, or false at its end. */
static bool read_all(unsigned char *to, size_t n) {
    while (n) {
        const size_t got = fread(to, 1, n, stdin);
        if (!got) return false;
        to += got, n -= got;
    }
    return true;
}

ImgWhy vision_frame(std::vector<unsigned char> &img) {
    img.clear();
    unsigned char head[IMG_HEAD];
    uint32_t len = 0;
    if (!read_all(head, sizeof head)) return ImgWhy::NoFrame;
    const ImgWhy framed = img_frame(head, sizeof head, len);
    if (framed != ImgWhy::Ok || len == 0) return framed;
    img.resize(len);
    if (!read_all(img.data(), len)) return img.clear(), ImgWhy::Short;
    ImgKind kind;
    uint32_t w, h;
    const ImgWhy why = img_header(img.data(), img.size(), kind, w, h);
    if (why != ImgWhy::Ok) std::fill(img.begin(), img.end(), 0), img.clear();
    return why;
}

bool vision_open(const ChatArgs &a, Chat &c) {
    /* The projector is a pinned file like the model: brought in, then read. */
    if (!chat_import(c, a.mmproj)) return false;
    mtmd_context_params p = mtmd_context_params_default();
    p.use_gpu = false, p.print_timings = false, p.warmup = false;
    p.n_threads = c.threads;
    mtmd_helper_log_set([](enum ggml_log_level, const char *, void *) {}, nullptr);
    errno = 0;
    c.vision = mtmd_init_from_file(a.mmproj.c_str(), c.model, p);
    if (!c.vision || !mtmd_support_vision(c.vision)) return c.err = errno ? errno : EINVAL, false;
    return true;
}

bool vision_turn(const ChatArgs &a, Chat &c, std::vector<unsigned char> &img, std::string &said, Put put,
                 void *to, int &made) {
    made = 0;
    c.stopped = false;
    while (!said.empty() && (said.back() == '\n' || said.back() == '\r')) said.back() = 0, said.pop_back();
    std::string asked = std::string(mtmd_default_marker()) + "\n" + said, turn;
    wipe(said);
    size_t skip = 0;
    const bool built = prompt_turn(c, asked, turn, skip);
    wipe(asked);
    mtmd_helper_bitmap_wrapper bm =
        mtmd_helper_bitmap_init_from_buf(c.vision, img.data(), img.size(), false, mtmd_helper_init_opt_default());
    std::fill(img.begin(), img.end(), 0), img.clear();
    if (!built || !bm.bitmap) {
        if (bm.bitmap) mtmd_bitmap_free(bm.bitmap);
        return wipe(turn), put(to, "(the image could not be read)", 29, false), built;
    }
    mtmd_input_text text = {turn.c_str() + (c.used ? skip : 0), turn.size() - (c.used ? skip : 0), true, true};
    mtmd_input_chunks *chunks = mtmd_input_chunks_init();
    const mtmd_bitmap *bitmaps[1] = {bm.bitmap};
    const int tokenized = mtmd_tokenize(c.vision, chunks, &text, bitmaps, 1);
    wipe(turn);
    mtmd_bitmap_free(bm.bitmap);
    const int start = c.used;
    llama_pos past = c.used;
    const size_t n = tokenized == 0 ? mtmd_helper_get_n_tokens(chunks) : 0;
    bool ok = tokenized == 0 && c.used + (int)n + a.n_reply + 8 <= c.n_ctx;
    if (tokenized == 0 && !ok) put(to, "(too long)", 10, false);
    ok = ok && mtmd_helper_eval_chunks(c.vision, c.ctx, chunks, c.used, 0, 512, true, &past) == 0;
    mtmd_input_chunks_free(chunks);
    if (ok) c.used = (int)past;
    return chat_reply(a, c, ok, start, put, to, made);
}

void vision_close(Chat &c) {
    if (c.vision) mtmd_free(c.vision);
    c.vision = nullptr;
}
