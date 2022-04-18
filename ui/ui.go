package ui

import (
	"time"

	raylib "github.com/gen2brain/raylib-go/raylib"
	"github.com/spf13/viper"
)

func update(dt time.Duration) {
	fullscreen := raylib.IsWindowFullscreen()
	if viper.GetBool("fullscreen") != fullscreen {
		raylib.ToggleFullscreen()
	}
}
