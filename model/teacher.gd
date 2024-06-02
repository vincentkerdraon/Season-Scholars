class_name TeacherModel extends BaseModel
#Station
#Controller
#Teach(Row)->
#Graduate(Row)->
#Welcome(Row)->
#StationChanged(Station)->

### MEMBERS

var station: BaseParam.STATION = BaseParam.STATION.NONE
var controller = null

### FUNC

func _init(arg, addListener: Callable):
	addListener.call(PipeOverlord.EventName.STATION_CHANGED, ListenStationChanged)
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
	return

func ListenStationChanged(sta :BaseParam.StationParam):
	station = sta.station
	return
