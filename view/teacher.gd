extends Node2D

var eventCall: Callable
var activeStationP1 = BaseParam.STATION.ST_COL_CENTER
var nextStationP1 = BaseParam.STATION.NONE
#var timerPathStarted: int

var previousDirP1: BaseParam.DIR =  BaseParam.DIR.NONE

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func LoadCallEvent(c: Callable):
	eventCall=c


func _input(event):
	if(Input.is_action_just_pressed("pointDownP1") || 
		Input.is_action_just_pressed("pointLeftP1") || 
		Input.is_action_just_pressed("pointRightP1") || 
		Input.is_action_just_pressed("pointUpP1") || 
		Input.is_action_just_released("pointDownP1") || 
		Input.is_action_just_released("pointLeftP1") || 
		Input.is_action_just_released("pointRightP1") || 
		Input.is_action_just_released("pointUpP1") ||
		Input.is_action_just_pressed("shortActionP1") # For moving to the direction
		):
		CalculateDir()
	if(Input.is_action_just_pressed("longActionP1") ||
		Input.is_action_just_pressed("shortActionP1") 
		):
		CalculateAction()

func CalculateDir():
	var velocity = Input.get_vector("pointLeftP1", "pointRightP1", "pointUpP1", "pointDownP1").snapped(Vector2.ONE)
	var direction : Vector2i = Vector2i(velocity.round())
	var dir: BaseParam.DIR = BaseParam.GetDirFromVector2Snapped(direction)
	
	ClearPathToNextStation()
	if(dir == BaseParam.DIR.NONE ):
		return
	nextStationP1=GetPointedStation(dir)
	if(nextStationP1 == BaseParam.STATION.NONE):
		return
	DrawPathToNextStation(nextStationP1)
	
	if(Input.is_action_pressed("shortActionP1")):
		eventCall.call(PipeOverlord.EventName.CHANGE_STATION, BaseParam.StationParam.new(nextStationP1))
	
func GetPointedStation(dir:BaseParam.DIR) -> BaseParam.STATION:
	match(activeStationP1):
		BaseParam.STATION.ST_COL_LEFT:
			match(dir):
				BaseParam.DIR.RIGHT:
					return BaseParam.STATION.ST_COL_CENTER
				_:
					return BaseParam.STATION.NONE
		BaseParam.STATION.ST_COL_CENTER:
			match(dir):
				BaseParam.DIR.RIGHT:
					return BaseParam.STATION.ST_COL_RIGHT
				BaseParam.DIR.LEFT:
					return BaseParam.STATION.ST_COL_LEFT
				_:
					return BaseParam.STATION.NONE
		BaseParam.STATION.ST_COL_RIGHT:
			match(dir):
				BaseParam.DIR.LEFT:
					return BaseParam.STATION.ST_COL_CENTER
				_:
					return BaseParam.STATION.NONE
		_:
			return BaseParam.STATION.NONE
			
func DrawPathToNextStation(nextStation):
	var pathNode
	if(activeStationP1>nextStation):
		pathNode = find_child("%s_TO_%s"%[BaseParam.STATION.find_key(nextStation), BaseParam.STATION.find_key(activeStationP1)])
	else: 
		pathNode = find_child("%s_TO_%s"%[BaseParam.STATION.find_key(activeStationP1), BaseParam.STATION.find_key(nextStation)])
	pathNode.visible = true

func ClearPathToNextStation():
	$ST_COL_CENTER_TO_ST_COL_RIGHT.visible =false
	$ST_COL_LEFT_TO_ST_COL_CENTER.visible =false
	
func CalculateAction():
	var velocity = Input.get_vector("pointLeftP1", "pointRightP1", "pointUpP1", "pointDownP1").snapped(Vector2.ONE)
	var direction : Vector2i = Vector2i(velocity.round())
	if(direction!=Vector2i(0,0)):
		#handled in CalculateDir
		return
	eventCall.call(PipeOverlord.EventName.PLAYER_ACTION, BaseParam.PlayerActionParam.new(Input.is_action_pressed("shortActionP1"),Input.is_action_pressed("longActionP1")))
	pass

func ListenStationChanged(ev: BaseParam.StationParam):
	activeStationP1 = ev.station
	nextStationP1 = BaseParam.STATION.NONE
	$ST_COL_CENTER.visible = false
	$ST_COL_LEFT.visible = false
	$ST_COL_RIGHT.visible = false
	find_child(BaseParam.STATION.find_key(ev.station)).visible = true
