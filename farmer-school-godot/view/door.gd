extends Node2D

var currentMonster:int=1
var currentProtected:int=1

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func ListenDoorEvent(param: BaseParam.DoorEventParam):
	if(param.isProtected):
		currentProtected-=1
		if (currentProtected == 0):
			currentProtected=get_meta("maxprot")
		$DoorSprite.texture = get_meta("prot%d" % currentProtected)
	else:
		currentMonster-=1
		if (currentMonster == 0):
			currentMonster=get_meta("maxmon")
		$DoorSprite.texture = get_meta("mon%d" % currentMonster)
	return
