extends Node2D


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func ListenDoorEvent(param: BaseParam.DoorEventParam):
	if(param.isProtected):
		$DoorSprite.texture = get_meta("prot1")
	else:
		$DoorSprite.texture = get_meta("mon1")
