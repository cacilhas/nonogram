extends Node

const BUFFER_SIZE = 4096
const STORAGE = "user://settings.data"

var easy := false
var size := 5
var version := 3

onready var help: WindowDialog = $Help
onready var label_template: Label = $LabelTemplate


func _ready() -> void:
	randomize()
	load_settings()


func load_settings() -> void:
	var file := File.new()
	if file.file_exists(STORAGE):
		file.open(STORAGE, File.READ)
		var res: Dictionary = bytes2var(file.get_buffer(BUFFER_SIZE))
		file.close()
		if res and res.version == version:
			size = res.get("size", 5)
			easy = res.get("easy", false)


func save() -> void:
	var file := File.new()
	var res := inst2dict(self)
	file.open(STORAGE, File.WRITE)
	file.store_buffer(var2bytes(res))
	file.close()
