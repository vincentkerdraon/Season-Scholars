extends Object
class_name BaseModel

var emitCallback = null

func _init(eC: Callable):
	emitCallback=eC

func Reset():
	print_debug("Reset")
