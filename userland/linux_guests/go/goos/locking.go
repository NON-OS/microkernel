package main

import (
	"os"
	"syscall"
)

func locking() {
	p := "flock"
	path := root + "/lock"
	os.WriteFile(path, nil, 0o644)
	a, _ := os.Open(path)
	b, _ := os.Open(path)
	defer a.Close()
	defer b.Close()
	if !check(p, syscall.Flock(int(a.Fd()), syscall.LOCK_EX) == nil, "LOCK_EX") {
		return
	}
	err := syscall.Flock(int(b.Fd()), syscall.LOCK_EX|syscall.LOCK_NB)
	if !check(p, err == syscall.EWOULDBLOCK, "second LOCK_NB", err) {
		return
	}
	syscall.Flock(int(a.Fd()), syscall.LOCK_UN)
	if !check(p, syscall.Flock(int(b.Fd()), syscall.LOCK_EX|syscall.LOCK_NB) == nil, "after unlock") {
		return
	}
	done(p, "an exclusive lock refuses a second open until it is released")
}
