extends ColorRect

signal done(board)
signal tips(count)

var board := []
var tips := 3
var maze: Maze

onready var cell_size: float = 500.0 / Global.size


func _ready() -> void:
	for i in Global.size * Global.size:
		board.append(0)


func _draw() -> void:
	for y in Global.size:
		for x in Global.size:
			var sx: float = x * cell_size
			var sy: float = y * cell_size
			var bg := Color("#dddddd") if (int(x / 5) + int(y / 5)) % 2 == 1 else Color.white

			match board[y * Global.size + x]:
				1:
					draw_rect(Rect2(Vector2(sx, sy), Vector2.ONE * cell_size), Color.black, true)
				2:
					draw_rect(Rect2(Vector2(sx, sy), Vector2.ONE * cell_size), bg, true)
					draw_line(Vector2(sx, sy), Vector2(sx+cell_size, sy+cell_size), Color.black, 2)
					draw_line(Vector2(sx, sy+cell_size), Vector2(sx+cell_size, sy), Color.black, 2)
				_:
					draw_rect(Rect2(Vector2(sx, sy), Vector2.ONE * cell_size), bg, true)

	for y in Global.size:
		draw_line(Vector2(0, y) * cell_size, Vector2(Global.size, y) * cell_size, Color.gray, 2)
	for x in Global.size:
		draw_line(Vector2(x, 0) * cell_size, Vector2(x, Global.size) * cell_size, Color.gray, 2)


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

			elif Input.is_key_pressed(KEY_SHIFT) and tips > 0:
				tips -= 1
				board[index] = 1 if maze.cell(position) == 1 else 2
				_check(position)
				emit_signal("tips", tips)
				call_deferred("update")
				return

			elif button_index == BUTTON_LEFT and board[index] == 2:
				return

			board[index] = clamp(button_index, 1, 2) if board[index] == 0 else 0
			_check(position)
			call_deferred("update")

	for index in Global.size * Global.size:
		if board[index] == 0:
			return
	emit_signal("done", board)


func lock(position: Vector2) -> void:
	board[position.y * Global.size + position.x] = 2


func _check(cell: Vector2) -> void:
	var size := Global.size

	if maze.check_column(cell.x, board):
		for y in size:
			Utils.set_cell(board, Vector2(cell.x, y), 1 if maze.cell(Vector2(cell.x, y)) == 1 else 2)

	if maze.check_line(cell.y, board):
		for x in size:
			Utils.set_cell(board, Vector2(x, cell.y), 1 if maze.cell(Vector2(x, cell.y)) == 1 else 2)
