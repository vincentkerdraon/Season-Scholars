extends Object
class_name BaseModel

var emitCallback = null

func _init(eC: Callable):
	emitCallback=eC

func Reset():
	print_debug("Reset")

static func object_to_dict(obj: Object) -> Dictionary:
	var result = {}
	var properties = obj.get_property_list()
	for prop in properties:
		var name = prop.name
		result[name] = obj.get(name)
	return result

static func object_to_json(obj) -> String:
	return JSON.stringify(object_to_dict(obj))
