package ui

// #cgo darwin LDFLAGS: -lraylib -framework OpenGL -framework OpenAL -framework Cocoa
// #cgo linux LDFLAGS: -lraylib -lGL -lpthread -lm -lXrandr -lraylib -lGL -lpthread -lm -lXrandr
// #cgo windows LDFLAGS: -lraylib -lGL -lm -lpthread -ldl -lrt
// #include <raylib.h>
import "C"

import (
	"unsafe"

	raylib "github.com/gen2brain/raylib-go/raylib"
)

var initialised bool = false
var camera raylib.Camera = raylib.Camera{
	Position:   raylib.Vector3{X: 10.0, Y: 10.0, Z: 8.0},
	Target:     raylib.Vector3{},
	Up:         raylib.Vector3{X: 0.0, Y: 1.0, Z: 0.0},
	Fovy:       60,
	Projection: raylib.CameraPerspective,
}

func renderVictory() {
	if !initialised {
		raylib.SetCameraMode(camera, raylib.CameraOrbital)
		initialised = true
	}

	raylib.UpdateCamera(&camera)

	raylib.BeginMode3D(camera)
	base := raylib.Vector3{X: 0, Y: -5, Z: 0}
	vert1 := raylib.Vector3{X: 0, Y: 5, Z: -4}
	vert2 := raylib.Vector3{X: 0, Y: 5, Z: 4}
	C.DrawCylinderEx(
		*(*C.Vector3)(unsafe.Pointer(&base)),
		*(*C.Vector3)(unsafe.Pointer(&vert1)),
		(C.float)(0.25),
		(C.float)(0.25),
		(C.int)(8),
		*(*C.Color)(unsafe.Pointer(&raylib.Green)),
	)
	C.DrawCylinderEx(
		*(*C.Vector3)(unsafe.Pointer(&base)),
		*(*C.Vector3)(unsafe.Pointer(&vert2)),
		(C.float)(0.25),
		(C.float)(0.25),
		(C.int)(8),
		*(*C.Color)(unsafe.Pointer(&raylib.Green)),
	)
	raylib.EndMode3D()
}
