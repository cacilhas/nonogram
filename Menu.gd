extends Control

onready var help: WindowDialog = Global.help.duplicate()


func _ready() -> void:
	add_child(help)
	$Settings/EasyBox.pressed = Global.easy
	match Global.size:
		10:
			$Settings/MediumSizeBox.pressed = true

		15:
			$Settings/LargeSizeBox.pressed = true

		_:
			$Settings/SmallSizeBox.pressed = true


func _input(_event: InputEvent) -> void:
	if Input.is_action_just_pressed("ui_help"):
		help.popup()
	if Input.is_action_just_pressed("ui_cancel"):
		if help.visible:
			help.hide()
		else:
			Global.save()
			get_tree().quit()


func _on_EasyBox_toggled(button_pressed: bool) -> void:
	Global.easy = button_pressed


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
