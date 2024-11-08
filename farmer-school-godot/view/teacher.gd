extends Node2D

var eventCall: Callable
var activeStationP1 = BaseParam.STATION.ST_COL_CENTER
var nextStationP1 = BaseParam.STATION.NONE
#var timerPathStarted: int

var previousDirP1: BaseParam.DIR =  BaseParam.DIR.NONE

var feedbackP1TimerBegin: int 
var feedbackP1TimerEnd: int  
var feedbackP1Enabled: bool = false

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	#if(feedbackP1TimerEnd < Time.get_ticks_msec()):
		#feedbackP1TimerEnd = Time.get_ticks_msec() +5000
		#feedbackP1TimerBegin = Time.get_ticks_msec() 
		#print("Restarting %d %d"% [feedbackP1TimerBegin,feedbackP1TimerEnd])
	#var timerNow =Time.get_ticks_msec() - feedbackP1TimerBegin
	#var timerEnd = feedbackP1TimerEnd - feedbackP1TimerBegin
	#$ST_COL_CENTER/TextureProgressBar.value = int(100*timerNow/timerEnd)
	if(feedbackP1Enabled):
		var timerNow =Time.get_ticks_msec() - feedbackP1TimerBegin
		var timerEnd = feedbackP1TimerEnd - feedbackP1TimerBegin
		GetStationNode(activeStationP1).find_child("TextureProgressBar").value = int(100*timerNow/timerEnd)
		if(feedbackP1TimerEnd <= Time.get_ticks_msec()):
			feedbackP1Enabled = false
			GetStationNode(activeStationP1).find_child("TextureProgressBar").visible = false
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
				BaseParam.DIR.UP:
					return BaseParam.STATION.WELCOME
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
				BaseParam.DIR.UP:
					return BaseParam.STATION.WELCOME
				_:
					return BaseParam.STATION.NONE
		BaseParam.STATION.WELCOME:
			match(dir):
				BaseParam.DIR.DOWNLEFT:
					return BaseParam.STATION.ST_COL_LEFT
				BaseParam.DIR.RIGHTDOWN:
					return BaseParam.STATION.ST_COL_RIGHT
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
	$ST_COL_LEFT_TO_WELCOME.visible =false
	$ST_COL_RIGHT_TO_WELCOME.visible =false
	
func CalculateAction():
	var velocity = Input.get_vector("pointLeftP1", "pointRightP1", "pointUpP1", "pointDownP1").snapped(Vector2.ONE)
	var direction : Vector2i = Vector2i(velocity.round())
	if(direction!=Vector2i(0,0)):
		#handled in CalculateDir
		return
	eventCall.call(PipeOverlord.EventName.PLAYER_ACTION, BaseParam.PlayerActionParam.new(Input.is_action_pressed("shortActionP1"),Input.is_action_pressed("longActionP1")))
	pass
	
func GetStationNode(station: BaseParam.STATION)->Node2D:
	return find_child(BaseParam.STATION.find_key(station))

func ListenStationChanged(ev: BaseParam.StationParam):
	activeStationP1 = ev.station
	nextStationP1 = BaseParam.STATION.NONE
	$ST_COL_CENTER.visible = false
	$ST_COL_LEFT.visible = false
	$ST_COL_RIGHT.visible = false
	$WELCOME.visible = false
	GetStationNode(ev.station).visible = true

func ListenFeedback(ev: BaseParam.FeedbackParam):
	feedbackP1TimerBegin =Time.get_ticks_msec()
	feedbackP1TimerEnd = ev.until
	feedbackP1Enabled = true
	GetStationNode(activeStationP1).find_child("TextureProgressBar").visible = true
	if(ev.shortAction):
		GetStationNode(activeStationP1).find_child("TextureProgressBar").texture_progress = get_meta("successShort")
	if (ev.longAction) :
		GetStationNode(activeStationP1).find_child("TextureProgressBar").texture_progress = get_meta("successLong")
	if (ev.fail): 
		GetStationNode(activeStationP1).find_child("TextureProgressBar").texture_progress = get_meta("fail")
		
