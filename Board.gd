extends ColorRect

signal done(board)

var board := PoolByteArray()

onready var cell_size: float = 500.0 / Global.size


func _ready() -> void:
	for i in Global.size * Global.size:
		board.append(0)


func _draw() -> void:
	for y in Global.size:
		draw_line(Vector2(0, y) * cell_size, Vector2(Global.size, y) * cell_size, Color.black, 2)
	for x in Global.size:
		draw_line(Vector2(x, 0) * cell_size, Vector2(x, Global.size) * cell_size, Color.black, 2)

	for y in Global.size:
		for x in Global.size:
			var sx: float = x * cell_size
			var sy: float = y * cell_size
			match board[y * Global.size + x]:
				1:
					draw_rect(Rect2(Vector2(sx+1, sy+1), Vector2.ONE * (cell_size - 2)), Color.black, true)
				2:
					draw_line(Vector2(sx, sy), Vector2(sx+cell_size, sy+cell_size), Color.black, 2)
					draw_line(Vector2(sx, sy+cell_size), Vector2(sx+cell_size, sy), Color.black, 2)


func _on_gui_input(event: InputEvent) -> void:
	var mouse := event as InputEventMouseButton
	if mouse and mouse.pressed:
		var position := (get_viewport().get_mouse_position() - Vector2.ONE * 100) / cell_size
		position = Vector2(int(position.x), int(position.y))
		if position.x >= 0 and position.x < Global.size and position.y >= 0 and position.y < Global.size:
			var index: int = position.y * Global.size + position.x
			var button_index := mouse.button_index
			if Input.is_key_pressed(KEY_CONTROL):
				button_index = KEY_RIGHT
			if button_index == BUTTON_LEFT and board[index] == 2:
				return
			board[index] = clamp(button_index, 1, 2) if board[index] == 0 else 0
			call_deferred("update")

	for index in Global.size * Global.size:
		if board[index] == 0:
			return
	emit_signal("done", board)
