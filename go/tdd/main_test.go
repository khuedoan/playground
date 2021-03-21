package main

import (
	"testing"
)

func TestHello(t *testing.T) {
	got := Hello("Khue Doan")
	want := "Hello, Khue Doan"

	if got != want {
		t.Errorf("got %q want %q", got, want)
	}
}
