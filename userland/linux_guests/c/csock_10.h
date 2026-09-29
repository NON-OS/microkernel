/* csock, part 10 of 11: included once, by csock.c. */

/*
 * The options Linux keeps that a loopback connection cannot tell apart,
 * read back as Linux reads them, and the ones a socket's kind refuses.
 */
static int quiet_options(void) {
    int t = socket(AF_INET, SOCK_STREAM, 0), u = socket(AF_INET, SOCK_DGRAM, 0);
    int x = socket(AF_UNIX, SOCK_STREAM, 0);
    int tos = 0x13, ttl = 32, zero = 0, ut = 5000, prio = 6;
    setsockopt(t, IPPROTO_IP, IP_TOS, &tos, sizeof tos);
    setsockopt(t, IPPROTO_IP, IP_TTL, &ttl, sizeof ttl);
    int bad_ttl = setsockopt(t, IPPROTO_IP, IP_TTL, &zero, sizeof zero) ? errno : 0;
    setsockopt(t, IPPROTO_TCP, TCP_USER_TIMEOUT, &ut, sizeof ut);
    setsockopt(t, SOL_SOCKET, SO_PRIORITY, &prio, sizeof prio);
    int got_tos = get_int(t, IPPROTO_IP, IP_TOS), got_ttl = get_int(t, IPPROTO_IP, IP_TTL);
    int got_ut = get_int(t, IPPROTO_TCP, TCP_USER_TIMEOUT);
    int got_qa = get_int(t, IPPROTO_TCP, TCP_QUICKACK), got_prio = get_int(t, SOL_SOCKET, SO_PRIORITY);
    int udp_tcp = setsockopt(u, IPPROTO_TCP, TCP_USER_TIMEOUT, &ut, sizeof ut) ? errno : 0;
    int unix_tcp = setsockopt(x, IPPROTO_TCP, TCP_NODELAY, &prio, sizeof prio) ? errno : 0;
    close(t);
    close(u);
    close(x);
    /* A TCP socket drops the two ECN bits of the type of service. */
    if (got_tos != 0x10 || got_ttl != 32 || bad_ttl != EINVAL || got_ut != 5000 || got_qa != 1 ||
        got_prio != 6 || udp_tcp != ENOPROTOOPT || unix_tcp != EOPNOTSUPP) {
        printf("[C] csock quiet_options got %d %d %d %d %d %d %d %d\n", got_tos, got_ttl, bad_ttl,
               got_ut, got_qa, got_prio, udp_tcp, unix_tcp);
        return fail("quiet_options: kept and read back as Linux does", got_tos, got_ttl);
    }
    ok("quiet_options", "6 kept and read back; refusals", unix_tcp);
    return 0;
}
