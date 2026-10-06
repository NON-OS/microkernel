package main

import (
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"syscall"
)

func system() {
	p := "system"
	exe, err := os.Executable()
	if !check(p, err == nil && filepath.Base(exe) == "goos", "Executable", exe, err) {
		return
	}
	host, err := os.Hostname()
	var u syscall.Utsname
	syscall.Uname(&u)
	var node []byte
	for _, c := range u.Nodename {
		if c == 0 {
			break
		}
		node = append(node, byte(c))
	}
	if !check(p, err == nil && host == string(node), "Hostname", host, string(node)) {
		return
	}
	info, _ := os.ReadFile("/proc/cpuinfo")
	cpus := strings.Count(string(info), "processor\t:")
	if !check(p, runtime.NumCPU() == cpus && cpus >= 1, "NumCPU", runtime.NumCPU(), cpus) {
		return
	}
	wd, err := os.Getwd()
	if !check(p, err == nil && wd == "/", "Getwd", wd, err) {
		return
	}
	done(p, "Executable, Hostname as uname says it, NumCPU as cpuinfo lists them")
}
