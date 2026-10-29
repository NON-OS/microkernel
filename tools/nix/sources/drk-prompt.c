/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * AGPL-3.0-or-later
 *
 * nonos-drk, the program the DarkFi wallet listing opens: a prompt in the
 * Terminal that runs DarkFi's drk once per line, so a wallet needs no
 * shell. The first run asks for the wallet's passphrase and writes drk's
 * config into the home from the one the content carries (so drk never sees
 * its default "changeme", which it refuses), then makes the wallet and its
 * first key. The node is a darkfid elsewhere, named with `:node`, reached
 * over the box's network; `nonos-drk localnet` is the local dev chain's
 * prompt (darkfi-localnet.c), whose node is in the same box. One `|`
 * joins two drk commands, as `transfer ... | broadcast` needs. With
 * `--address` it only makes the wallet if need be and prints its address.
 * drk's `fun`, a sound for each payment, is off: a box has no audio.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <termios.h>
#include <unistd.h>

#define DRK "/usr/bin/drk"
#define TEMPLATE "/usr/share/nonos-drk/drk_config.toml"
#define CONFIG "/root/.config/darkfi/drk_config.toml"
#define ARGS 48

static const char *net = "testnet";

static int run(char *const argv[], int in, int out) {
    pid_t pid = fork();
    if (pid == 0) {
        if (in >= 0)
            dup2(in, 0);
        if (out >= 0)
            dup2(out, 1);
        execv(argv[0], argv);
        _exit(127);
    }
    int status = 1;
    if (pid > 0)
        waitpid(pid, &status, 0);
    return status;
}

/* `line` as drk's argv after "-n NET", split at spaces; "..." keeps one. */
static int words(char *line, char *argv[]) {
    int n = 0;
    argv[n++] = DRK, argv[n++] = "-n", argv[n++] = (char *)net;
    for (char *p = line; *p && n < ARGS - 1;) {
        while (*p == ' ' || *p == '\t')
            p++;
        if (!*p)
            break;
        char end = *p == '"' ? '"' : ' ';
        if (end == '"')
            p++;
        argv[n++] = p;
        while (*p && *p != end && !(end == ' ' && *p == '\t'))
            p++;
        if (*p)
            *p++ = 0;
    }
    argv[n] = NULL;
    return n;
}

static int drk(const char *line) {
    char copy[1024], *argv[ARGS];
    snprintf(copy, sizeof copy, "%s", line);
    char *bar = strchr(copy, '|');
    if (!bar) {
        words(copy, argv);
        return run(argv, -1, -1);
    }
    *bar = 0;
    char *argv2[ARGS];
    words(copy, argv), words(bar + 1, argv2);
    int p[2];
    if (pipe(p) != 0)
        return 1;
    pid_t first = fork();
    if (first == 0) {
        close(p[0]);
        dup2(p[1], 1);
        execv(argv[0], argv);
        _exit(127);
    }
    close(p[1]);
    int status = run(argv2, p[0], -1);
    close(p[0]);
    waitpid(first, NULL, 0);
    return status;
}

/* The line `key = ...` in `[network_config."NET"]`, set to `value`. */
static void set(const char *key, const char *value) {
    FILE *f = fopen(CONFIG, "r");
    if (!f)
        return;
    char out[65536] = "", line[512], head[64];
    snprintf(head, sizeof head, "[network_config.\"%s\"]", net);
    int here = 0;
    while (fgets(line, sizeof line, f)) {
        if (line[0] == '[')
            here = strncmp(line, head, strlen(head)) == 0;
        if (here && strncmp(line, key, strlen(key)) == 0 && line[strlen(key)] == ' ')
            snprintf(line, sizeof line, "%s = \"%s\"\n", key, value);
        strncat(out, line, sizeof out - strlen(out) - 1);
    }
    fclose(f);
    if ((f = fopen(CONFIG, "w"))) {
        fputs(out, f);
        fclose(f);
    }
}

static void ask(const char *what, char *out, size_t len, int hide) {
    struct termios was, now;
    int tty = tcgetattr(0, &was) == 0;
    if (hide && tty) {
        now = was, now.c_lflag &= ~ECHO;
        tcsetattr(0, TCSANOW, &now);
    }
    fputs(what, stderr);
    if (!fgets(out, (int)len, stdin))
        exit(1);
    out[strcspn(out, "\r\n")] = 0;
    if (hide && tty) {
        tcsetattr(0, TCSANOW, &was);
        fputc('\n', stderr);
    }
}

static void first_run(void) {
    char pass[128], again[128], line[512];
    do {
        ask("A passphrase for your DarkFi wallet (8 or more, no quotes): ", pass, sizeof pass, 1);
        ask("Once more: ", again, sizeof again, 1);
    } while (strlen(pass) < 8 || strcmp(pass, again) || strpbrk(pass, "\"\\") || !strcmp(pass, "changeme"));
    FILE *in = fopen(TEMPLATE, "r");
    mkdir("/root/.config", 0700), mkdir("/root/.config/darkfi", 0700);
    FILE *out = fopen(CONFIG, "w");
    if (!in || !out)
        exit(1);
    chmod(CONFIG, 0600);
    while (fgets(line, sizeof line, in))
        fputs(!strncmp(line, "wallet_pass = ", 14) ? "wallet_pass = \"\"\n"
              : !strncmp(line, "fun = true", 10) ? "fun = false\n" : line, out);
    fclose(in), fclose(out);
    const char *nets[] = {"localnet", "testnet", "mainnet"}, *mine = net;
    for (int i = 0; i < 3; i++)
        net = nets[i], set("wallet_pass", pass);
    net = mine;
    memset(pass, 0, sizeof pass), memset(again, 0, sizeof again);
}

static void wallet(void) {
    char db[256];
    snprintf(db, sizeof db, "/root/.local/share/darkfi/drk/%s/wallet.db", net);
    if (access(db, F_OK) == 0)
        return;
    mkdir("/root/.local", 0700), mkdir("/root/.local/share", 0700);
    mkdir("/root/.local/share/darkfi", 0700), mkdir("/root/.local/share/darkfi/drk", 0700);
    snprintf(db, sizeof db, "/root/.local/share/darkfi/drk/%s", net);
    mkdir(db, 0700);
    drk("wallet --initialize");
    drk("wallet --keygen");
    drk("wallet --default-address 1");
}

int main(int argc, char **argv) {
    setvbuf(stdout, NULL, _IOLBF, 0);
    int address = argc > 1 && !strcmp(argv[argc - 1], "--address");
    if (argc > 1 && !strcmp(argv[1], "localnet"))
        net = "localnet";
    if (access(CONFIG, F_OK) != 0)
        first_run();
    wallet();
    if (address)
        return drk("wallet --address");
    char line[1024];
    if (strcmp(net, "localnet")) {
        puts("Your node: a darkfid you run elsewhere, as :node tcp://host:port, reached over Anyone.");
    }
    puts("Each line runs drk. Try: wallet --address, wallet --balance, ping, transfer ..., :help, :quit");
    for (;;) {
        printf("drk %s> ", net);
        fflush(stdout);
        if (!fgets(line, sizeof line, stdin))
            break;
        line[strcspn(line, "\r\n")] = 0;
        if (!line[0])
            continue;
        if (!strcmp(line, ":quit"))
            break;
        if (!strcmp(line, ":help")) {
            drk("help");
            puts(":node tcp://host:port  the darkfid to ask     :quit  leave");
            puts("Two commands join with |, as in: transfer 1 <token> <address> | broadcast");
        } else if (!strncmp(line, ":node ", 6) && (!strncmp(line + 6, "tcp://", 6) || !strncmp(line + 6, "tcp+tls://", 10))) {
            set("endpoint", line + 6);
            printf("Node set to %s\n", line + 6);
        } else if (line[0] == ':') {
            puts("Not a prompt command: :help, :node, :quit");
        } else {
            drk(line);
        }
    }
    return 0;
}
