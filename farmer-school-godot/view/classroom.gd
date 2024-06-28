extends Node

# Called when the node enters the scene tree for the first time.
func _ready():
	$ClassroomSprite.texture = get_meta("asparagus")
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func listenSeason(param: BaseParam.SeasonParam):
	match param.season:
		BaseParam.SEASON.ASPARAGUS:
			$ClassroomSprite.texture = get_meta("asparagus")
		BaseParam.SEASON.CHERRY:
			$ClassroomSprite.texture = get_meta("Cherry")
		BaseParam.SEASON.MUSHROOM:
			$ClassroomSprite.texture = get_meta("Mushroom")
		BaseParam.SEASON.LEMON:
			$ClassroomSprite.texture = get_meta("Lemon")
		_:
			printerr("No Season reconized")
