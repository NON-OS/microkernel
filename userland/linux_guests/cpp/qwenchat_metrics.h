/*
 * qwenchat's numbers on the serial log, through /dev/nonos-metrics: how
 * long the model took to open, and for each turn its tokens, the wait for
 * the first piece and the speed after it. The personality prints only the
 * names it lists and their integers (capsule_linux dev_metrics_parse.rs),
 * so nothing said either way reaches the log.
 */
#pragma once
#include "qwenchat.h"

/* A turn's pieces passed on to `put`, with the time of the first kept. */
struct Timed {
    Put put;
    void *to;
    double t0 = 0;    /* when the turn was handed to chat_turn */
    double first = 0; /* when its first piece came; 0 before then */
};
double metrics_now();
/* Put for chat_turn: keeps the first piece's time, then passes it on. */
void timed_put(void *to, const char *piece, size_t n, bool thought);
/* After chat_open, `seconds` after it began: what it opened, or its errno. */
void metrics_load(const Chat &c, bool opened, double seconds);
/* After chat_turn: `made` tokens, timed by t. */
void metrics_turn(const Chat &c, const Timed &t, int made);
