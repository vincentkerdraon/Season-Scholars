extends Node2D

var activeStationP1
# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func ListenStationChanged(param: BaseParam.StationParam):
	match(activeStationP1):
		BaseParam.STATION.ST_COL_LEFT:
			$TeachingLeft.visible=true
		BaseParam.STATION.ST_COL_CENTER:
			$TeachingCentral.visible=true
		BaseParam.STATION.ST_COL_RIGHT:
			$TeachingRight.visible=true
		_:
			return
	var teachingCol: Node2D
	match(param.station):
		BaseParam.STATION.ST_COL_LEFT:
			teachingCol = $TeachingLeft/TeacherSprite
			activeStationP1 = BaseParam.STATION.ST_COL_LEFT
		BaseParam.STATION.ST_COL_CENTER:
			teachingCol = $TeachingCentral/TeacherSprite
			activeStationP1 = BaseParam.STATION.ST_COL_CENTER
		BaseParam.STATION.ST_COL_RIGHT:
			teachingCol = $TeachingRight/TeacherSprite
			activeStationP1 = BaseParam.STATION.ST_COL_RIGHT
		_:
			return
	teachingCol.texture = get_meta("TeacherP1")
	teachingCol.get_parent().visible = true
	
	
