class_name StudentModel extends BaseModel
		#Knowledge
		#RC Position
		#->SeasonChanged(Season)
		#->Teach(Row)
		#->Graduate(Row)
		#StudentGraduated(Knowledge[])->

### MEMBERS

var knowledge: Array[BaseParam.SEASON]=[]
var position = {"col":BaseParam.STUDENT_COLS.COL_LEFT, "row":-1}
var currentSeason: BaseParam.SEASON
var guid: String

### FUNC

func _init(arg, addListener: Callable, i: String):
	#addListener.call(PipeOverlord.EventName.SEASON_CHANGED, Callable(self,"ListenSeasonChanged"))
	addListener.call(PipeOverlord.EventName.SEASON_CHANGED, ListenSeasonChanged)
	addListener.call(PipeOverlord.EventName.TEACH, ListenTeach)
	addListener.call(PipeOverlord.EventName.GRADUATE, ListenGraduate)
	guid = i
	super(arg)
	
func RemoveListeners(removeListener: Callable):
	removeListener.call(PipeOverlord.EventName.SEASON_CHANGED, ListenSeasonChanged)
	removeListener.call(PipeOverlord.EventName.TEACH, ListenTeach)
	removeListener.call(PipeOverlord.EventName.GRADUATE, ListenGraduate)

func ListenSeasonChanged(season :BaseParam.SeasonParam):
	currentSeason = season.season
	return

func ListenTeach(col: BaseParam.StationParam):
	if col.station == position["col"]:
		print("Student %s pos R%dC%d" % [guid, position["row"], position["col"]])
		knowledge.append(currentSeason)
		print_debug(knowledge)
		emitCallback.call(PipeOverlord.EventName.STUDENT_TAUGHT, BaseParam.StudentTaughtParam.new(guid, knowledge))
	return
	
func ListenGraduate(col:BaseParam.StationParam):
	if col.station == position["col"] && position["row"]==0:
		emitCallback.call(PipeOverlord.EventName.STUDENT_GRADUATED, BaseParam.KnowledgesParam.new(knowledge, guid))
	if col.station == position["col"] && position["row"]>0:
		position["row"]-=1
		emitCallback.call(PipeOverlord.EventName.STUDENT_CHANGED, BaseParam.ChangeStudentParam.new(position["col"], position["row"]+1, position["row"], guid))
	return	

func SetPosition(col: BaseParam.STUDENT_COLS, row: int):
	position["row"]=row
	position["col"]=col
