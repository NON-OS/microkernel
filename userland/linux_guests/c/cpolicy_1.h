/* cpolicy, part 1 of 1: included once, by cpolicy.c. */

static struct sockaddr_in at(unsigned ip, int port) {
    struct sockaddr_in sa;
    memset(&sa, 0, sizeof sa);
    sa.sin_family = AF_INET;
    sa.sin_port = htons(port);
    sa.sin_addr.s_addr = htonl(ip);
    return sa;
}

static int err(int rc) {
    return rc < 0 ? errno : 0;
}
