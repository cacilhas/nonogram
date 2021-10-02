extends Node2D

export var Maze: Script

var cell_size: float

onready var board: ColorRect = $Board
onready var help: WindowDialog = Global.help.duplicate()
onready var maze: Maze = Maze.new(Global.size)
onready var tips: Label = $Tips/Label
onready var victory: Label = $Victory/Label


func _ready() -> void:
	add_child(help)
	maze.build()
	board.maze = maze
	cell_size = 500 / Global.size

	for y in Global.size:
		var label: Label = Global.label_template.duplicate()
		var text: String = maze.get_line_strings(y).join(" ")
		if text == "":
			text = "0"
		label.text = text
		label.align = Label.ALIGN_RIGHT
		label.rect_position = Vector2(5, y * cell_size + 100)
		label.rect_size = Vector2(90, cell_size)
		var font: DynamicFont = label.get("custom_fonts/font").duplicate()
		font.size = min(cell_size - 2, 80 / text.length())
		label.set("custom_fonts/font", font)
		add_child(label)
		label.show()

	for x in Global.size:
		var label: Label = Global.label_template.duplicate()
		var text: String = maze.get_column_strings(x).join("\n")
		if text == "":
			text = "0"
		label.text = text
		label.align = Label.ALIGN_CENTER
		label.rect_position = Vector2(x * cell_size + 100, 5)
		label.rect_size = Vector2(cell_size, 90)
		var font: DynamicFont = label.get("custom_fonts/font").duplicate()
		font.size = min(cell_size - 2, 80 / text.length())
		label.set("custom_fonts/font", font)
		add_child(label)
		label.show()


func _input(_event: InputEvent) -> void:
	if Input.is_action_just_pressed("ui_help"):
		help.popup()
	if Input.is_action_just_pressed("ui_cancel"):
		if help.visible:
			help.hide()
		else:
			Global.save()
			get_tree().change_scene("res://Menu.tscn")


func _on_done(board: PoolByteArray) -> void:
	if maze.check(board):
		victory.show()


func _on_tips(left: int) -> void:
	tips.text = "Tips left:\n%d" % left
