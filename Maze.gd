class_name Maze
extends Object

var size: int
var _data := []


func _init(size_: int).() -> void:
	size = size_


func build() -> void:
	for _i in size*size:
		# TODO: use a better maze algorithm
		_data.append(0 if randi() % 3 == 0 else 1)


func cell(position: Vector2) -> int:
	return Utils.get_cell(_data, position)


func check(board: Array) -> bool:
	for y in size:
		if not check_line(y, board):
			return false

	for x in size:
		if not check_column(x, board):
			return false

	return true


func check_column(x: int, board: Array) -> bool:
	return Utils.column_eq(x, board, _data)


func check_line(y: int, board: Array) -> bool:
	return Utils.line_eq(y, board, _data)


func get_column_string(x: int) -> String:
	var res := PoolStringArray()
	var cur := 0
	for value in Utils.get_column(x, _data):
		if value == 0 and cur > 0:
			res.append(String(cur))
			cur = 0
		else:
			cur += value
	if cur > 0:
		res.append(String(cur))
	var text := res.join("\n")
	return "0" if text == "" else text


func get_line_string(y: int) -> String:
	var res := PoolStringArray()
	var cur := 0
	for value in Utils.get_line(y, _data):
		if value == 0 and cur > 0:
			res.append(String(cur))
			cur = 0
		else:
			cur += value
	if cur > 0:
		res.append(String(cur))
	var text := res.join(" ")
	return "0" if text == "" else text
