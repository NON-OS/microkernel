/*
 * net/http as Linux runs it, inside one guest: a server on 127.0.0.1:0 and a
 * client of it. Twenty GETs, each answered 200 with the path it asked for,
 * over one kept-alive connection, which the server's own count of new
 * connections shows.
 */
package main

import (
	"fmt"
	"io"
	"net"
	"net/http"
	"os"
	"sync/atomic"
)

func main() {
	ln, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		fmt.Println("[GO] gohttp FAIL: listen:", err)
		os.Exit(1)
	}
	var conns atomic.Int32
	srv := &http.Server{
		Handler: http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			io.WriteString(w, "hello "+r.URL.Path)
		}),
		ConnState: func(_ net.Conn, s http.ConnState) {
			if s == http.StateNew {
				conns.Add(1)
			}
		},
	}
	go srv.Serve(ln)
	client := &http.Client{}
	const gets = 20
	for i := 0; i < gets; i++ {
		r, err := client.Get(fmt.Sprintf("http://%s/%d", ln.Addr(), i))
		if err != nil {
			fmt.Println("[GO] gohttp FAIL: get", i, err)
			os.Exit(1)
		}
		body, err := io.ReadAll(r.Body)
		r.Body.Close()
		if err != nil || r.StatusCode != 200 || string(body) != fmt.Sprintf("hello /%d", i) {
			fmt.Println("[GO] gohttp FAIL: reply", i, r.StatusCode, string(body), err)
			os.Exit(1)
		}
	}
	if n := conns.Load(); n != 1 {
		fmt.Println("[GO] gohttp FAIL: keep-alive: connections", n)
		os.Exit(1)
	}
	fmt.Printf("[GO] gohttp PASS: %d GETs answered 200 over %d connection\n", gets, conns.Load())
}
