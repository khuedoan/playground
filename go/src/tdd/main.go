package main

import (
	"fmt"
)

const greetingPrefix = "Hello, "

func Hello(name string) string {
	return greetingPrefix + name
}

func main() {
	fmt.Println("Go TDD")
}
