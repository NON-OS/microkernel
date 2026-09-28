// What a guest's sockets may reach, and what a descriptor number alone gets.
// Each part prints the errno it got; the NONOS answer is the capsule's policy
// and differs from an unconfined Linux by design, so the host's line records
// what Linux itself would allow. Parts:
//   bind-any       bind 0.0.0.0:0              NONOS EACCES, Linux 0
//   bind-out       bind 10.0.2.15:0            NONOS EACCES, Linux EADDRNOTAVAIL
//   listen-unbound listen with no bind         NONOS EACCES, Linux 0
//   udp-out        sendto 192.0.2.1:9          NONOS ENETUNREACH, Linux 1
//   raw            socket(SOCK_RAW)            NONOS EPERM, Linux EPERM unless root
//   forged         recv/close on numbers the guest never opened, and on a
//                  pipe: EBADF, EBADF, ENOTSOCK on both
#include <errno.h>
#include <netinet/in.h>
#include <stdio.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

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

int main(void) {
    int s = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in any = at(0, 0), out = at(0x0a00020f, 0), far = at(0xc0000201, 9);
    int bind_any = err(bind(s, (void *)&any, sizeof any));
    close(s);
    s = socket(AF_INET, SOCK_STREAM, 0);
    int bind_out = err(bind(s, (void *)&out, sizeof out));
    close(s);
    s = socket(AF_INET, SOCK_STREAM, 0);
    int listen_unbound = err(listen(s, 1));
    close(s);
    int u = socket(AF_INET, SOCK_DGRAM, 0);
    int udp_out = sendto(u, "x", 1, 0, (void *)&far, sizeof far) == 1 ? 0 : errno;
    close(u);
    int raw = err(socket(AF_INET, SOCK_RAW, IPPROTO_ICMP));
    char buf[4];
    int pipe_fds[2];
    pipe(pipe_fds);
    int forged_recv = err(recv(777, buf, 4, MSG_DONTWAIT));
    int forged_close = err(close(778));
    int pipe_recv = err(recv(pipe_fds[0], buf, 4, MSG_DONTWAIT));
    int pipe_name = err(getsockname(pipe_fds[0], (void *)&any, &(socklen_t){sizeof any}));
    printf("[C] cpolicy bind-any %d bind-out %d listen-unbound %d udp-out %d raw %d "
           "forged %d %d pipe %d %d\n",
           bind_any, bind_out, listen_unbound, udp_out, raw, forged_recv, forged_close,
           pipe_recv, pipe_name);
    int confined = bind_any == EACCES && bind_out == EACCES && listen_unbound == EACCES &&
                   udp_out == ENETUNREACH && raw == EPERM;
    int forged = forged_recv == EBADF && forged_close == EBADF && pipe_recv == ENOTSOCK &&
                 pipe_name == ENOTSOCK;
    printf("[C] cpolicy %s: confined %d, forged numbers refused %d\n",
           confined && forged ? "PASS" : "FAIL", confined, forged);
    fflush(stdout);
    return confined && forged ? 0 : 1;
}
