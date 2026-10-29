/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * AGPL-3.0-or-later
 *
 * nonos-darkirc, the program the DarkIRC listing opens: one box holds
 * DarkFi's darkirc daemon and suckless sic, an IRC client, joined on the
 * box's own loopback. It asks for a nick on the first run and keeps it in
 * the home, starts darkirc with the config the content carries (so its
 * first run does not stop to write one), waits for its IRC port, runs sic
 * on it, and stops darkirc when sic ends. darkirc's own output goes
 * nowhere: the Terminal is sic's.
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

#define CONFIG "/usr/share/nonos-darkirc/darkirc_config.toml"
#define NICKS "/root/.config/nonos-darkirc"
#define NICK NICKS "/nick"
#define PORT 6667
#define WAIT_S 120

static void nick(char *out, size_t len) {
    FILE *f = fopen(NICK, "r");
    if (f && fgets(out, (int)len, f) && out[0] != '\n') {
        fclose(f);
        out[strcspn(out, "\r\n")] = 0;
        return;
    }
    if (f)
        fclose(f);
    do {
        fputs("Your DarkIRC nick (letters and digits): ", stdout);
        fflush(stdout);
        if (!fgets(out, (int)len, stdin))
            _exit(1);
        out[strcspn(out, "\r\n")] = 0;
    } while (!out[0] || strspn(out, "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-") != strlen(out));
    mkdir("/root/.config", 0700);
    mkdir(NICKS, 0700);
    if ((f = fopen(NICK, "w"))) {
        fprintf(f, "%s\n", out);
        fclose(f);
    }
}

static int irc_up(void) {
    struct sockaddr_in at = {.sin_family = AF_INET, .sin_port = htons(PORT)};
    at.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    int s = socket(AF_INET, SOCK_STREAM, 0);
    if (s < 0)
        return 0;
    int up = connect(s, (struct sockaddr *)&at, sizeof at) == 0;
    close(s);
    return up;
}

static pid_t start(char *const argv[], int quiet) {
    pid_t pid = fork();
    if (pid == 0) {
        if (quiet) {
            int null = open("/dev/null", O_RDWR);
            dup2(null, 0), dup2(null, 1), dup2(null, 2);
        }
        execv(argv[0], argv);
        _exit(127);
    }
    return pid;
}

int main(void) {
    setvbuf(stdout, NULL, _IOLBF, 0);
    char name[32];
    nick(name, sizeof name);
    char *daemon[] = {"/usr/bin/darkirc", "--config", CONFIG, NULL};
    pid_t irc = start(daemon, 1);
    if (irc < 0)
        return 1;
    puts("Starting DarkIRC. Its peers are reached over Anyone, never direct.");
    for (int s = 0; !irc_up(); s++) {
        if (s == WAIT_S || waitpid(irc, NULL, WNOHANG) == irc) {
            puts("DarkIRC did not start.");
            kill(irc, SIGTERM);
            return 1;
        }
        sleep(1);
    }
    puts("Connected. Channels sync from peers first, so the history fills in over a few minutes.");
    puts("  :j #dev        join a channel      :s #dev   where plain lines go");
    puts("  :m #dev text   say in a channel    :l #dev   leave      Ctrl-D   quit");
    char *client[] = {"/usr/bin/sic", "-h", "127.0.0.1", "-p", "6667", "-n", name, NULL};
    pid_t chat = start(client, 0);
    if (chat > 0)
        waitpid(chat, NULL, 0);
    kill(irc, SIGTERM);
    waitpid(irc, NULL, 0);
    return 0;
}
