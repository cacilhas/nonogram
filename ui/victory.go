package ui

// #cgo darwin LDFLAGS: -lraylib -framework OpenGL -framework OpenAL -framework Cocoa
// #cgo linux LDFLAGS: -lraylib -lGL -lm -lpthread -lXrandr
// #cgo windows LDFLAGS: -lraylib -lGL -lm -lpthread -ldl -lrt
// #include <raylib.h>
import "C"

import (
	"unsafe"

	raylib "github.com/gen2brain/raylib-go/raylib"
)

var cameraModeSet = false

func renderVictory(camera *raylib.Camera) {
	if !cameraModeSet {
		raylib.SetCameraMode(*camera, raylib.CameraOrbital)
		cameraModeSet = true
	}
	raylib.UpdateCamera(camera)
	base := raylib.Vector3{X: 0, Y: -5, Z: 0}
	vert1 := raylib.Vector3{X: 0, Y: 5, Z: -4}
	vert2 := raylib.Vector3{X: 0, Y: 5, Z: 4}
	drawCylindexEx(base, vert1, 0.25, 0.25, 8, raylib.Green)
	drawCylindexEx(base, vert2, 0.25, 0.25, 8, raylib.Green)
}

func drawCylindexEx(startPos, endPos raylib.Vector3, radiusTop, radiusBottom float32, slices int32, color raylib.Color) {
	C.DrawCylinderEx(
		*(*C.Vector3)(unsafe.Pointer(&startPos)),
		*(*C.Vector3)(unsafe.Pointer(&endPos)),
		(C.float)(radiusTop),
		(C.float)(radiusBottom),
		(C.int)(slices),
		*(*C.Color)(unsafe.Pointer(&color)),
	)
}
