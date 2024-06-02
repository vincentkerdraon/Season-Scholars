class_name StudentModel extends BaseModel
		#Knowledge
		#RC Position
		#->SeasonChanged(Season)
		#->Teach(Row)
		#->Graduate(Row)
		#StudentGraduated(Knowledge[])->

### MEMBERS

var knowledge = []
var position = {"col":-1, "row":-1}
var currentSeason: BaseParam.SEASON
var guid: String

### FUNC

func _init(arg, addListener: Callable, i: String):
	#addListener.call(PipeOverlord.EventName.SEASON_CHANGED, Callable(self,"ListenSeasonChanged"))
	addListener.call(PipeOverlord.EventName.SEASON_CHANGED, ListenSeasonChanged)
	addListener.call(PipeOverlord.EventName.TEACH, ListenTeach)
	guid = i
	super(arg)

func ListenSeasonChanged(season :BaseParam.SeasonParam):
	currentSeason = season.season
	return

func ListenTeach(col: BaseParam.StudentColParam):
	if col == position["col"]:
		knowledge.append(BaseParam.KnowledgeParam.new(currentSeason,1))
	return
	
func ListenGraduate(col:BaseParam.StudentColParam):
	if col == position["col"] && position["row"]==0:
		emitCallback.call(PipeOverlord.EventName.STUDENT_GRADUATED, BaseParam.KnowledgesParam.new(knowledge, guid))
	if col == position["col"] && position["row"]>0:
		position["row"]-=1
	return	
