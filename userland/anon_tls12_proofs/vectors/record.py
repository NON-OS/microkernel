#!/usr/bin/env python3
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# Records net.anon's TLS 1.2 client against a real OpenSSL server, one case
# at a time: starts `openssl s_server`, runs the ignored record_openssl test
# against it, and talks to the server's console (a line of data, then "q" to
# close or "r" to ask for renegotiation). Writes <case>.server, <case>.client
# and <case>.outcome next to this file.
#
# The server's RSA key is a throwaway made in a temporary directory for this
# run and removed with it. It is never printed and never written here; only
# its certificate reaches the recording, inside the server's bytes, which is
# what a relay sends anyone.
#
# Run from the crate: python3 vectors/record.py   (OpenSSL 3.0 was used.)

import os, subprocess, sys, tempfile, time

HERE = os.path.dirname(os.path.abspath(__file__))
CRATE = os.path.dirname(HERE)

NO_EMS = """openssl_conf = conf
[conf]
ssl_conf = ssl
[ssl]
system_default = sys
[sys]
Options = -ExtendedMasterSecret
"""

# name: (s_server arguments, console lines after the client's ping, config)
CASES = {
    "chacha_certreq": (["-tls1_2", "-cipher", "ECDHE-RSA-CHACHA20-POLY1305", "-verify", "1"], ["hello from openssl", "Q"], None),
    "aes_gcm_renegotiate": (["-tls1_2", "-cipher", "ECDHE-RSA-AES256-GCM-SHA384"], ["hello from openssl", "r"], None),
    "pkcs1_sha384": (["-tls1_2", "-cipher", "ECDHE-RSA-AES256-GCM-SHA384", "-sigalgs", "RSA+SHA384", "-verify", "1"], ["hello from openssl", "q"], None),
    "pkcs1_sha256": (["-tls1_2", "-cipher", "ECDHE-RSA-CHACHA20-POLY1305", "-sigalgs", "RSA+SHA256"], ["hello from openssl", "q"], None),
    "fragmented": (["-tls1_2", "-cipher", "ECDHE-RSA-AES256-GCM-SHA384", "-max_send_frag", "512", "-verify", "1"], ["hello from openssl", "q"], None),
    "no_ems": (["-tls1_2", "-cipher", "ECDHE-RSA-CHACHA20-POLY1305"], ["hello from openssl", "q"], NO_EMS),
    "downgrade": (["-cipher", "ECDHE-RSA-CHACHA20-POLY1305"], [], None),
    "no_shared_suite": (["-tls1_2", "-cipher", "ECDHE-RSA-AES128-GCM-SHA256"], [], None),
    # s_server drops the socket without close_notify; Python's ssl, OpenSSL
    # underneath, sends one on unwrap().
    "close_notify": (None, [], None),
}

PY_SERVER = """
import socket, ssl, sys
ctx = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
ctx.maximum_version = ssl.TLSVersion.TLSv1_2
ctx.set_ciphers("ECDHE-RSA-AES256-GCM-SHA384")
ctx.options |= ssl.OP_NO_TICKET
ctx.load_cert_chain(sys.argv[2], sys.argv[3])
s = socket.socket(); s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
s.bind(("127.0.0.1", int(sys.argv[1]))); s.listen(1)
c, _ = s.accept()
t = ctx.wrap_socket(c, server_side=True)
t.recv(100)
t.sendall(b"hello from python ssl\\n")
t.unwrap()
"""

def record(name, args, lines, conf, tmp, port):
    env = dict(os.environ)
    if conf:
        path = os.path.join(tmp, "no_ems.cnf")
        open(path, "w").write(conf)
        env["OPENSSL_CONF"] = path
    if args is None:
        command = [sys.executable, "-c", PY_SERVER, str(port), os.path.join(tmp, "cert.pem"), os.path.join(tmp, "key.pem")]
    else:
        command = ["openssl", "s_server", "-accept", str(port), "-cert", os.path.join(tmp, "cert.pem"),
         "-key", os.path.join(tmp, "key.pem"), "-no_ticket"] + args
    server = subprocess.Popen(
        command, stdin=subprocess.PIPE, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
    time.sleep(0.8)
    client = subprocess.Popen(
        ["cargo", "test", "--offline", "--release", "record_openssl", "--", "--ignored", "--nocapture"],
        cwd=CRATE, env=dict(os.environ, NONOS_TLS12_RECORD="%s:%d" % (name, port)),
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    time.sleep(1.5)
    for line in lines:
        server.stdin.write((line + "\n").encode())
        server.stdin.flush()
        time.sleep(1.0)
    out = client.communicate(timeout=60)[0].decode()
    server.kill()
    server.wait()
    said = [l for l in out.splitlines() if l.startswith(name + ":")]
    print(said[0] if said else name + ": no outcome\n" + out)

def main():
    subprocess.run(["cargo", "test", "--offline", "--release", "--no-run"], cwd=CRATE, check=True,
                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    with tempfile.TemporaryDirectory() as tmp:
        subprocess.run(["openssl", "req", "-x509", "-newkey", "rsa:2048", "-nodes", "-keyout",
                        os.path.join(tmp, "key.pem"), "-out", os.path.join(tmp, "cert.pem"),
                        "-subj", "/CN=www.relay.example", "-days", "7"],
                       check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        for i, (name, (args, lines, conf)) in enumerate(CASES.items()):
            if len(sys.argv) > 1 and name not in sys.argv[1:]:
                continue
            record(name, args, lines, conf, tmp, 44330 + i)

main()
