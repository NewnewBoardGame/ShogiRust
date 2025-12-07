package main

import (
	"fmt"
	"testing"
)

func Test(t *testing.T) {
	fmt.Println("Testing `shogi_go.go`...")
	t.Error("Some error occurred...")
}
