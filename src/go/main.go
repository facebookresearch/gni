// Copyright (c) Meta Platforms, Inc. and affiliates.
// All rights reserved.

// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.

package main

import (
	"fmt"
	gni "gni/src/go/cgo"
)

func main() {
	fmt.Println(gni.GetGPUNodeID())
}
