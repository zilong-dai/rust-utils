package main

import "C"

import (
	"fmt"
)

//export add
func add(a, b int) int {
	return a + b
}

func main() {
	fmt.Println("Hello, World!")
}
