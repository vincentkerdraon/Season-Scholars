class_name TeacherModel extends BaseModel
#Station
#Controller
#Teach(Row)->
#Graduate(Row)->
#Welcome(Row)->
#StationChanged(Station)->

### MEMBERS

var station: BaseParam.STATION = BaseParam.STATION.ST_COL_CENTER
var endCurrentAction: int=0
var isWelcomeAvailable: bool = false

const TIME_MOVE = 2000
const TIME_TEACH_SHORT = 5000
const TIME_GRAD_LONG = 10000
const TIME_WELCOME_SHORT = 500
const TIME_RECRUIT_LONG = 1000
#const TIME_WATCH_SHORT = 5000
const TIME_ERROR = 1000

### FUNC

func _init(arg, addListener: Callable):
	addListener.call(PipeOverlord.EventName.CHANGE_STATION, ListenChangeStation)
	addListener.call(PipeOverlord.EventName.PLAYER_ACTION, ListenPlayerAction)
	addListener.call(PipeOverlord.EventName.WELCOME_AVAILABLE, ListenWelcomeAvailable)
	super(arg)

func Reset():
	station = BaseParam.STATION.NONE
	isWelcomeAvailable = false

func PlayerActionTeach(to: int):
	print_debug("Teach {station}".format({"station":station}))
	emitCallback.call(PipeOverlord.EventName.TEACH, BaseParam.StationParam.new(station))
	return

func PlayerActionGraduate(to: int):
	print_debug("Graduate {station}".format({"station":station}))
	emitCallback.call(PipeOverlord.EventName.GRADUATE, BaseParam.StationParam.new(station))
	return

func PlayerActionWelcome(to:int):
	print_debug("Welcome {station}".format({"station":station}))
	emitCallback.call(PipeOverlord.EventName.WELCOME_AVAILABLE, BaseParam.WelcomeAvailableParam.new(false))
	emitCallback.call(PipeOverlord.EventName.WELCOME, BaseParam.StationParam.new(station))
	return

func PlayerActionRecruit(to:int):
	print_debug("Recruit {station}".format({"station":station}))
	emitCallback.call(PipeOverlord.EventName.WELCOME_AVAILABLE, BaseParam.WelcomeAvailableParam.new(true))
	return

func ListenChangeStation(sta :BaseParam.StationParam):
	var now=Time.get_ticks_msec()
	if(now < endCurrentAction):
		print_debug("Current move not completed. Should finish at %d and it is %d" % [endCurrentAction, now])
		emitCallback.call(PipeOverlord.EventName.INVALID_ACTION_STATION, BaseParam.StationParam.new(station))
		return
	endCurrentAction= now + TIME_MOVE
	station = sta.station
	emitCallback.call(PipeOverlord.EventName.STATION_CHANGED, sta)
	emitCallback.call(PipeOverlord.EventName.FEEDBACK, BaseParam.FeedbackParam.new(true, false, false, endCurrentAction, true))
	return
	
func ListenWelcomeAvailable(param: BaseParam.WelcomeAvailableParam):
	isWelcomeAvailable=param.isAvailable
	
func ListenPlayerAction(act: BaseParam.PlayerActionParam):
	var now=Time.get_ticks_msec()
	if(now < endCurrentAction):
		print_debug("Current action not completed. Should finish at %d and it is %d" % [endCurrentAction, now])
		emitCallback.call(PipeOverlord.EventName.INVALID_ACTION_STATION, BaseParam.StationParam.new(station))
		return
	if(act.longAction && act.shortAction):
		printerr("Not implemented: Short and long action at the same time in station %s" % BaseParam.STATION.find_key(station))
		emitCallback.call(PipeOverlord.EventName.ERROR_ACTION_STATION, BaseParam.StationParam.new(station))
		emitCallback.call(PipeOverlord.EventName.FEEDBACK, BaseParam.FeedbackParam.new(false, false, true, now + TIME_ERROR, true))
		return
	if(!act.longAction && !act.shortAction):
		printerr("Error call: no short action, no long action, in station %s" % BaseParam.STATION.find_key(station))
		emitCallback.call(PipeOverlord.EventName.ERROR_ACTION_STATION, BaseParam.StationParam.new(station))
		emitCallback.call(PipeOverlord.EventName.FEEDBACK, BaseParam.FeedbackParam.new(false, false, true, now + TIME_ERROR, true))
		return
	match(station):
		BaseParam.STATION.ST_COL_LEFT, BaseParam.STATION.ST_COL_CENTER, BaseParam.STATION.ST_COL_RIGHT:
			if(act.shortAction):
				endCurrentAction = now + TIME_TEACH_SHORT 
				PlayerActionTeach(endCurrentAction)
				emitCallback.call(PipeOverlord.EventName.FEEDBACK, BaseParam.FeedbackParam.new(false, true, false, endCurrentAction, true))
			else:
				endCurrentAction = now + TIME_GRAD_LONG
				PlayerActionGraduate(endCurrentAction)
				emitCallback.call(PipeOverlord.EventName.FEEDBACK, BaseParam.FeedbackParam.new(true, false, false, endCurrentAction, true))
		BaseParam.STATION.WELCOME:
			if(act.shortAction):
				endCurrentAction = now + TIME_WELCOME_SHORT
				PlayerActionWelcome(endCurrentAction)
				emitCallback.call(PipeOverlord.EventName.FEEDBACK, BaseParam.FeedbackParam.new(false, true, false, endCurrentAction, true))
			else:
				endCurrentAction = now + TIME_RECRUIT_LONG
				PlayerActionRecruit(endCurrentAction)
				emitCallback.call(PipeOverlord.EventName.FEEDBACK, BaseParam.FeedbackParam.new(true, false, false, endCurrentAction, true))
		_:
			printerr("Action not yet implemented in station %s" % BaseParam.STATION.find_key(station))
			emitCallback.call(PipeOverlord.EventName.ERROR_ACTION_STATION, BaseParam.StationParam.new(station))
			emitCallback.call(PipeOverlord.EventName.FEEDBACK, BaseParam.FeedbackParam.new(false, false, true, now + TIME_ERROR, true))
			
			
