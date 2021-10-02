class_name Maze
extends Object

var size: int
var _data: PoolByteArray
var _hash: String


func _init(size_: int).() -> void:
	size = size_


func build() -> void:
	_data = PoolByteArray()
	for _i in size*size:
		_data.append(randi() % 2)  # TODO: use a maze algorithm
	_hash = array_hash(_data)


func cell(x: int, y: int) -> int:
	x %= size
	y %= size
	return _data[y * size + x]


func check(board: PoolByteArray) -> bool:
	return array_hash(board) == _hash


func get_column(x: int) -> PoolByteArray:
	var res := PoolByteArray()
	var sum := 0
	for y in size:
		var cur := cell(x, y)
		if cur == 0 and sum > 0:
			res.append(sum)
			sum = 0
		elif cur == 1:
			sum += 1
	if sum > 0:
		res.append(sum)
	return res


func get_column_strings(x: int) -> PoolStringArray:
	var res := PoolStringArray()
	for value in get_column(x):
		res.append(String(value))
	return res


func get_line(y: int) -> PoolByteArray:
	var res := PoolByteArray()
	var sum := 0
	for x in size:
		var cur := cell(x, y)
		if cur == 0 and sum > 0:
			res.append(sum)
			sum = 0
		elif cur == 1:
			sum += 1
	if sum > 0:
		res.append(sum)
	return res


func get_line_strings(x: int) -> PoolStringArray:
	var res := PoolStringArray()
	for value in get_line(x):
		res.append(String(value))
	return res


static func array_hash(array: PoolByteArray) -> String:
	var res := PoolByteArray()
	var sum := 0
	var index := 0
	for value in array:
		sum |= (value % 2) << index
		index += 1
		if index == 8:
			res.append(sum)
			sum = 0
			index = 0
	if sum > 0:
		res.append(sum)
	return res.get_string_from_ascii()
