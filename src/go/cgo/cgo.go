// Copyright (c) Meta Platforms, Inc. and affiliates.
// All rights reserved.

// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.

package cgo

/*
#cgo CFLAGS: -I../../c
#cgo LDFLAGS: -L../../c -lGNI -L../../../target/debug -lgni_lib
#include "../../c/GNI.h"
#include <stdlib.h> // for free()
*/
import "C"
import (
	"unsafe"
)

func GetGPUNodeID() string {
	cString := C.c_get_gpu_node_id()
	goGPUNodeID := C.GoString(cString)
	C.free(unsafe.Pointer(cString))
	return goGPUNodeID
}
