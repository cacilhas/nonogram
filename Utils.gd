class_name Utils
extends Object


static func array_hash(array: Array) -> String:
	return String(array)
	#return PoolByteArray(array).get_string_from_ascii()


static func column_eq(x: int, a: Array, b: Array) -> bool:
	return array_hash(get_column(x, a)) == array_hash(get_column(x, b))


static func line_eq(y: int, a: Array, b: Array) -> bool:
	return array_hash(get_line(y, a)) == array_hash(get_line(y, b))


static func get_cell(board: Array, position: Vector2) -> int:
	var size := int(sqrt(board.size()))
	var index := (int(position.y) % size) * size + (int(position.x) % size)
	return board[index]


static func set_cell(board: Array, position: Vector2, value: int) -> void:
	var size := int(sqrt(board.size()))
	var index := (int(position.y) % size) * size + (int(position.x) % size)
	board[index] = value


static func get_column(x: int, board: Array) -> Array:
	var size := int(sqrt(board.size()))
	var res := []
	for value in board.slice(x, x - size, size):
		res.append(value % 2)
	return res


static func get_line(y: int, board: Array) -> Array:
	var size := int(sqrt(board.size()))
	var res := []
	for value in board.slice(y*size, y*size + size - 1, 1):
		res.append(value % 2)
	return res
