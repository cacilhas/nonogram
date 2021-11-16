package ui

type Scene interface {
	Init() Scene
	Render() Scene
}
