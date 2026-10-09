/* qwencheck: the numbers, to /dev/nonos-metrics. See qwencheck.h. */
#include "qwencheck.h"

#include <cstdio>
#include <fcntl.h>
#include <sys/resource.h>
#include <unistd.h>

/*
 * One line, one write: the device takes a whole line of listed names and
 * integers and refuses anything else.
 */
void report(const Args &a, const Run &r, bool match) {
    struct rusage ru {};
    getrusage(RUSAGE_SELF, &ru);
    const double tps = r.decode_s > 0 ? r.decodes / r.decode_s : 0;
    char line[512];
    int n = snprintf(line, sizeof line,
                     "match=%d tokens=%zu prompt_tokens=%d threads=%d load_ms=%ld ttft_ms=%ld "
                     "decode_ms=%ld tok_per_s_x100=%ld peak_rss_kb=%ld\n",
                     match ? 1 : 0, r.ids.size(), r.prompt_tokens, a.threads,
                     (long)(r.load_s * 1000), (long)(r.ttft_s * 1000), (long)(r.decode_s * 1000),
                     (long)(tps * 100), ru.ru_maxrss);
    /* A failed run says which step failed and errno then, numbers only. */
    if (r.stage) n = snprintf(line, sizeof line, "match=0 fail_stage=%d errno=%d load_ms=%ld model_bytes=%lld\n",
                              r.stage, r.err, (long)(r.load_s * 1000), r.model_bytes);
    int fd = open("/dev/nonos-metrics", O_WRONLY);
    if (fd < 0 || n <= 0 || n >= (int)sizeof line) return;
    (void)!write(fd, line, (size_t)n);
    close(fd);
}
