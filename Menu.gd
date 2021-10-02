extends Control

onready var help: WindowDialog = Global.help.duplicate()
onready var small_size_box = $Settings/SmallSizeBox
onready var medium_size_box = $Settings/MediumSizeBox
onready var large_size_box = $Settings/LargeSizeBox


func _ready() -> void:
	add_child(help)
	match Global.size:
		10:
			medium_size_box.pressed = true

		15:
			large_size_box.pressed = true

		_:
			small_size_box.pressed = true


func _input(_event: InputEvent) -> void:
	if Input.is_action_just_pressed("ui_help"):
		help.popup()
	if Input.is_action_just_pressed("ui_cancel"):
		if help.visible:
			help.hide()
		else:
			Global.save()
			get_tree().quit()


func _on_SmallSizeBox_toggled(button_pressed: bool) -> void:
	if button_pressed:
		Global.size = 5


func _on_MediumSizeBox_toggled(button_pressed: bool) -> void:
	if button_pressed:
		Global.size = 10


func _on_LargeSizeBox_toggled(button_pressed: bool) -> void:
	if button_pressed:
		Global.size = 15


func _on_StartButton_pressed() -> void:
	Global.save()
	get_tree().change_scene("res://Main.tscn")
