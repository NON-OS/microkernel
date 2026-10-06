/* qwenchat: its numbers on the serial log. See qwenchat_metrics.h. */
#include "qwenchat_metrics.h"

#include <chrono>
#include <cstdio>
#include <fcntl.h>
#include <sys/resource.h>
#include <unistd.h>

/* The device takes at most this much, one line in one write (MAX_LINE). */
static const int LINE = 512;

double metrics_now() {
    using namespace std::chrono;
    return duration<double>(steady_clock::now().time_since_epoch()).count();
}

void timed_put(void *to, const char *piece, size_t n, bool thought) {
    Timed &t = *(Timed *)to;
    if (t.first == 0) t.first = metrics_now();
    t.put(t.to, piece, n, thought);
}

/* The most memory this process has held, as the personality counts it. */
static long peak_kb() {
    struct rusage ru {};
    return getrusage(RUSAGE_SELF, &ru) == 0 ? ru.ru_maxrss : 0;
}

/* A guest started without the device, or on a host, says nothing. */
static void say(const char *line, int n) {
    if (n <= 0 || n >= LINE) return;
    const int fd = open("/dev/nonos-metrics", O_WRONLY | O_CLOEXEC);
    if (fd < 0) return;
    (void)!write(fd, line, (size_t)n);
    close(fd);
}

void metrics_load(const Chat &c, bool opened, double seconds) {
    char line[LINE];
    const long ms = (long)(seconds * 1000);
    /* fail_stage 1 is the open, as qwencheck numbers its own first stage. */
    const int n = opened ? snprintf(line, sizeof line, "threads=%d load_ms=%ld model_bytes=%lld peak_rss_kb=%ld\n",
                                    c.threads, ms, c.mem.weights, peak_kb())
                         : snprintf(line, sizeof line, "fail_stage=1 errno=%d load_ms=%ld model_bytes=%lld\n",
                                    c.err, ms, c.mem.weights);
    say(line, n);
}

/*
 * The speed is the decode alone: from the first piece on, one token at a
 * time. The wait before it is the prompt going in, said as ttft_ms, which
 * grows with the turn's length, not with the machine's decode speed.
 */
void metrics_turn(const Chat &c, const Timed &t, int made) {
    const double end = metrics_now(), first = t.first > 0 ? t.first : end;
    const double decode = end - first;
    const long tps_x100 = made > 1 && decode > 0 ? (long)((made - 1) / decode * 100) : 0;
    char line[LINE];
    const int n = snprintf(line, sizeof line,
                           "tokens=%d threads=%d ttft_ms=%ld decode_ms=%ld tok_per_s_x100=%ld peak_rss_kb=%ld\n",
                           made, c.threads, (long)((first - t.t0) * 1000), (long)(decode * 1000), tps_x100,
                           peak_kb());
    say(line, n);
}
