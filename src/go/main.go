package main

import (
	"fmt"
	gni "gni/src/go/cgo"
)

func main() {
	fmt.Println(gni.GetGPUNodeID())
}
