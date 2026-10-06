/* qwenchat: the model's parts brought onto the volume before it loads. See qwenchat.h. */
#include "qwenchat.h"
#include "qwenmem_file.h"

#include <cerrno>
#include <fcntl.h>
#include <sys/stat.h>
#include <unistd.h>

/*
 * The personality brings a pinned model onto the data volume the first
 * time a part is opened: sealed, hashed and held to its signed pin, which
 * takes minutes for a large part, and nothing else in this program runs
 * meanwhile. Each part is opened here once, first to last, so the person
 * is told which part and how large before each wait (c.step), and Esc or
 * a closed window between parts stops there.
 *
 * A refusal ends it with the personality's errno. The plan and the loader
 * open the model only after every part was opened here, so a refused
 * import is never asked for a second time: before this, a refused first
 * part was opened again by the memory plan and again by the loader.
 */
bool chat_import(Chat &c, const std::string &model) {
    const std::vector<std::string> parts = model_parts(model.c_str());
    for (size_t i = 0; i < parts.size(); i++) {
        /* A pinned part not brought in yet reports its pinned size. */
        struct stat st;
        const long long bytes = stat(parts[i].c_str(), &st) == 0 ? (long long)st.st_size : 0;
        if (c.step) c.step(c.step_to, (int)i + 1, (int)parts.size(), bytes);
        if (c.stop && !c.stopped) c.stopped = c.stop(c.stop_to);
        if (c.stopped) return c.err = ECANCELED, false;
        const int fd = open(parts[i].c_str(), O_RDONLY | O_CLOEXEC);
        if (fd < 0) return c.err = errno, false;
        close(fd);
    }
    return true;
}
