package ui

import (
	"os/exec"
	"runtime"
	"strconv"
	"strings"

	raylib "github.com/gen2brain/raylib-go/raylib"
)

var fsWidth, fsHeight int32 = 0, 0

func GetResolution() (int32, int32) {
	width := int32(raylib.GetScreenWidth())
	height := int32(raylib.GetScreenHeight())

	if width == 0 || height == 0 {
		width, height = GetSysResolution()
	}

	return width, height
}

func GetSysResolution() (int32, int32) {
	if fsWidth == 0 || fsHeight == 0 {
		switch runtime.GOOS {
		case "darwin":
			fsWidth, fsHeight = getSysResolution_darwin()

		case "linux":
			fsWidth, fsHeight = getSysResolution_linux()

		case "windows":
			fsWidth, fsHeight = getSysResolution_windows()

		default:
			fsWidth, fsHeight = 1138, 640 // default to 0.73M9
		}
	}
	return fsWidth, fsHeight
}

func getSysResolution_darwin() (int32, int32) {
	cmd := exec.Command("system_profiler", "SPDisplaysDataType")
	out, _ := cmd.Output()
	for _, line := range strings.Split(string(out), "\n") {
		if strings.Contains(line, "Resolution") {
			res := strings.Split(strings.TrimSpace(line), ": ")[1]
			resSplit := strings.Split(res, " x ")
			width, _ := strconv.Atoi(resSplit[0])
			height, _ := strconv.Atoi(resSplit[1])
			return int32(width), int32(height)
		}
	}
	return 0, 0
}

func getSysResolution_linux() (int32, int32) {
	cmd := exec.Command("xrandr")
	if out, err := cmd.Output(); err == nil {
		for _, line := range strings.Split(string(out), "\n") {
			if strings.Contains(line, "*") {
				res := strings.Split(strings.TrimSpace(line), " ")[0]
				resSplit := strings.Split(res, "x")
				width, _ := strconv.Atoi(resSplit[0])
				height, _ := strconv.Atoi(resSplit[1])
				return int32(width), int32(height)
			}
		}
	}
	return 0, 0
}

func getSysResolution_windows() (int32, int32) {
	// XXX: untested!!
	cmd := exec.Command("wmic", "desktopmonitor", "get", "screenwidth", "screenheight")
	if res, err := cmd.Output(); err == nil {
		data := strings.Split(string(res), " ")
		width, _ := strconv.Atoi(data[0])
		height, _ := strconv.Atoi(data[1])
		return int32(width), int32(height)
	}
	return 0, 0
}
