extends Node2D

var currentTraveller:int=1

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func ListenWelcomeAvailable(param: BaseParam.WelcomeAvailableParam):
	if(param.isAvailable):
		currentTraveller-=1
		if (currentTraveller == 0):
			currentTraveller=get_meta("maxtrav")
		$WelcomeSprite.texture = get_meta("t%d" % currentTraveller)
	else:
		$WelcomeSprite.texture = get_meta("closed")
