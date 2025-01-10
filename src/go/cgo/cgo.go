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
