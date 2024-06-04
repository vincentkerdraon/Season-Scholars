class_name TeacherModel extends BaseModel
#Station
#Controller
#Teach(Row)->
#Graduate(Row)->
#Welcome(Row)->
#StationChanged(Station)->

### MEMBERS

var station: BaseParam.STATION = BaseParam.STATION.NONE
var endCurrentAction: int=0

const TIME_TEACH_SHORT = 5

### FUNC

func _init(arg, addListener: Callable):
	addListener.call(PipeOverlord.EventName.CHANGE_STATION, ListenChangeStation)
	addListener.call(PipeOverlord.EventName.PLAYER_ACTION, ListenPlayerAction)
	super(arg)

func PlayerActionTeach():
	print_debug("Teach {station}".format({"station":station}))
	if BaseParam.IsValidStationToStudentCols(station):
		emitCallback.call(PipeOverlord.EventName.TEACH, BaseParam.StationToStudentCols(station))
	return

func PlayerActionGraduate():
	print_debug("Graduate {station}".format({"station":station}))
	if BaseParam.IsValidStationToStudentCols(station):
		emitCallback.call(PipeOverlord.EventName.GRADUATE, BaseParam.StationToStudentCols(station))
	return

func PlayerActionWelcome():
	print_debug("Welcome {station}".format({"station":station}))
	if BaseParam.IsValidStationToStudentCols(station):
		emitCallback.call(PipeOverlord.EventName.WELCOME, BaseParam.StationToStudentCols(station))
		emitCallback.call(PipeOverlord.EventName.WELCOME_AVAILABLE, BaseParam.WelcomeAvailableParam.new(false))
	return

func ListenChangeStation(sta :BaseParam.StationParam):
	station = sta.station
	emitCallback.call(PipeOverlord.EventName.STATION_CHANGED, sta)
	return
	
func ListenPlayerAction(act: BaseParam.PlayerActionParam):
	var now=Time.get_ticks_msec()
	if(now < endCurrentAction):
		print_debug("Current action not completed. Should finish at %d and it is %d" % [endCurrentAction, now])
		emitCallback.call(PipeOverlord.EventName.INVALID_ACTION_STATION, BaseParam.StationParam.new(station))
		return
	if(act.longAction && act.shortAction):
		printerr("Not implemented: Short and long action at the same time in station %s" % BaseParam.STATION.find_key(station))
		emitCallback.call(PipeOverlord.EventName.ERROR_ACTION_STATION, BaseParam.StationParam.new(station))
		return
	if(!act.longAction && !act.shortAction):
		printerr("Error call: no short action, no long action, in station %s" % BaseParam.STATION.find_key(station))
		emitCallback.call(PipeOverlord.EventName.ERROR_ACTION_STATION, BaseParam.StationParam.new(station))
		return
	match(station):
		BaseParam.STATION.ST_COL_LEFT, BaseParam.STATION.ST_COL_CENTER, BaseParam.STATION.ST_COL_RIGHT:
			if(act.shortAction):
				endCurrentAction = now + TIME_TEACH_SHORT * 1000
				PlayerActionTeach()
		_:
			printerr("Action not yet implemented in station %s" % BaseParam.STATION.find_key(station))
			emitCallback.call(PipeOverlord.EventName.ERROR_ACTION_STATION, BaseParam.StationParam.new(station))
			
			
