/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * AGPL-3.0-or-later
 *
 * nonos-darkfi-localnet, the program the DarkFi local dev chain listing
 * opens: one box holds DarkFi's darkfid on its localnet, the minerd that
 * mines its blocks, and the drk prompt (drk-prompt.c), joined on the box's
 * own loopback. The wallet is made first, so darkfid's config in the home
 * names its address as the recipient of what is mined. It starts minerd and
 * darkfid with the configs the content carries, waits for darkfid's RPC,
 * runs the prompt on localnet, and stops both when the prompt ends. The
 * chain is this box's alone: it reaches no network at all.
 */
#include <arpa/inet.h>
#include <fcntl.h>
#include <netinet/in.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <unistd.h>

#define SHARE "/usr/share/nonos-darkfi-localnet"
#define CONFIG "/root/.config/darkfi/darkfid_config.toml"
#define RPC 8240
#define WAIT_S 180

static pid_t start(char *const argv[], int quiet, int out) {
    pid_t pid = fork();
    if (pid == 0) {
        int null = open("/dev/null", O_RDWR);
        if (quiet)
            dup2(null, 0), dup2(null, 1), dup2(null, 2);
        if (out >= 0)
            dup2(out, 1);
        execv(argv[0], argv);
        _exit(127);
    }
    return pid;
}

static int up(void) {
    struct sockaddr_in at = {.sin_family = AF_INET, .sin_port = htons(RPC)};
    at.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    int s = socket(AF_INET, SOCK_STREAM, 0);
    int ok = s >= 0 && connect(s, (struct sockaddr *)&at, sizeof at) == 0;
    if (s >= 0)
        close(s);
    return ok;
}

/* The wallet's address, from `nonos-drk localnet --address`'s last line. */
static int address(char *out, size_t len) {
    int p[2];
    if (pipe(p) != 0)
        return 0;
    char *argv[] = {"/usr/bin/nonos-drk", "localnet", "--address", NULL};
    pid_t pid = start(argv, 0, p[1]);
    close(p[1]);
    FILE *f = fdopen(p[0], "r");
    char line[128];
    out[0] = 0;
    while (f && fgets(line, sizeof line, f))
        if (line[0] > ' ')
            snprintf(out, len, "%s", line);
    if (f)
        fclose(f);
    waitpid(pid, NULL, 0);
    out[strcspn(out, "\r\n")] = 0;
    return strlen(out) > 30 && !strpbrk(out, " \"\\");
}

/* darkfid's config, the content's with localnet's recipient set to `to`. */
static int config(const char *to) {
    FILE *in = fopen(SHARE "/darkfid_config.toml", "r"), *out;
    mkdir("/root/.config", 0700), mkdir("/root/.config/darkfi", 0700);
    if (!in || !(out = fopen(CONFIG, "w")))
        return 0;
    char line[512];
    while (fgets(line, sizeof line, in)) {
        if (!strncmp(line, "recipient = ", 12))
            fprintf(out, "recipient = \"%s\"\n", to);
        else
            fputs(line, out);
    }
    fclose(in), fclose(out);
    return 1;
}

int main(void) {
    setvbuf(stdout, NULL, _IOLBF, 0);
    char to[128];
    if (!address(to, sizeof to) || !config(to)) {
        puts("The wallet could not be made.");
        return 1;
    }
    char *miner[] = {"/usr/bin/minerd", "--config", SHARE "/minerd.toml", NULL};
    char *node[] = {"/usr/bin/darkfid", "--config", CONFIG, "-n", "localnet", NULL};
    pid_t m = start(miner, 1, -1), d = start(node, 1, -1);
    printf("Starting a local DarkFi chain; what it mines goes to %s.\n", to);
    for (int s = 0; !up(); s++) {
        if (s == WAIT_S || waitpid(d, NULL, WNOHANG) == d) {
            puts("darkfid did not start.");
            kill(m, SIGTERM), kill(d, SIGTERM);
            return 1;
        }
        sleep(1);
    }
    puts("The chain is up and mining. `scan` reads its blocks into the wallet.");
    char *prompt[] = {"/usr/bin/nonos-drk", "localnet", NULL};
    waitpid(start(prompt, 0, -1), NULL, 0);
    kill(d, SIGTERM), kill(m, SIGTERM);
    waitpid(d, NULL, 0), waitpid(m, NULL, 0);
    return 0;
}
