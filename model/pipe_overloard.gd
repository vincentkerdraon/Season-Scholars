class_name PipeOverlord extends Node

enum EventName{
	TEACH, GRADUATE, WELCOME, PLAYER_ACTION, CHANGE_STATION,
	STATION_CHANGED, STUDENT_GRADUATED, OGRE_FED, STUDENT_CHANGED, STUDENT_WELCOMED,
	GAME_OVER, ERROR_ACTION_STATION, INVALID_ACTION_STATION,
	SEASON_CHANGED, DOOR_CHANGED, WELCOME_AVAILABLE, WINDOW_CHANGED, WINDOW_HARVEST_CHANGED, DOOR_DESTROYED, 
}

var listenerDict={}
var idStudent = 0
var allStudents: Dictionary = {}
var studentToFree: Array[StudentModel]=[]
var allModels: Array[BaseModel]

var seasonTimer: Timer
var currentSeason :BaseParam.SEASON
const TIME_SEASON = 15

func _init():
	for ev in EventName.values():
		listenerDict[ev]=Array([])
	listenerDict[EventName.STUDENT_GRADUATED].append(ListenStudentGraduated)
	listenerDict[EventName.GAME_OVER].append(ListenGameOver)
	listenerDict[EventName.WELCOME].append(ListenWelcome)
	#CreateGame()

func CleanUp():
	seasonTimer.stop()
	for student in allStudents.keys():
		DeleteStudent(student)

func AddListener(event: EventName, callback: Callable):
	listenerDict[event].append(callback)
	return

func Emit(event: EventName, param: BaseParam):
	print_debug("New Event emitted: {event}, with params {param}".format({"event":EventName.find_key(event), "param":BaseModel.object_to_json(param)}))
	for callback in listenerDict[event]:
		callback.call(param)
	return

func CreateTeacher()->TeacherModel:
	return TeacherModel.new(Emit,AddListener)

func CreateStudent()->StudentModel:
	var stu= StudentModel.new(Emit, AddListener, "%d"%idStudent)
	allStudents["%d"%idStudent]=stu
	idStudent+=1
	return stu

func CreateAndPlaceStudent()->StudentModel:
	var cols = {}
	cols[BaseParam.STUDENT_COLS.COL_LEFT] = 0
	cols[BaseParam.STUDENT_COLS.COL_CENTER] = 0
	cols[BaseParam.STUDENT_COLS.COL_RIGHT] = 0
	for s in allStudents:
		cols[allStudents[s].position["col"]]+=1
	var mins = []
	if(cols[BaseParam.STUDENT_COLS.COL_LEFT] <= cols[BaseParam.STUDENT_COLS.COL_CENTER] && cols[BaseParam.STUDENT_COLS.COL_LEFT] <= cols[BaseParam.STUDENT_COLS.COL_RIGHT]):
		mins.append(BaseParam.STUDENT_COLS.COL_LEFT)
	if(cols[BaseParam.STUDENT_COLS.COL_CENTER] <= cols[BaseParam.STUDENT_COLS.COL_LEFT] && cols[BaseParam.STUDENT_COLS.COL_CENTER] <= cols[BaseParam.STUDENT_COLS.COL_RIGHT]):
		mins.append(BaseParam.STUDENT_COLS.COL_CENTER)
	if(cols[BaseParam.STUDENT_COLS.COL_RIGHT] <= cols[BaseParam.STUDENT_COLS.COL_CENTER] && cols[BaseParam.STUDENT_COLS.COL_RIGHT] <= cols[BaseParam.STUDENT_COLS.COL_LEFT]):
		mins.append(BaseParam.STUDENT_COLS.COL_RIGHT)
	
	var col = mins[randi() %mins.size()]
	var row = cols[col]
	var stu = CreateStudent()
	stu.SetPosition(col, row)
	
	return stu

func CreateDoor()->DoorModel:
	return DoorModel.new(Emit, AddListener, 4, 4)

func CreateWorld()->WorldModel:
	return WorldModel.new(Emit, AddListener)

func DeleteStudent(id: String):
	studentToFree.append(allStudents[id])

func ListenStudentGraduated(param :BaseParam.KnowledgesParam):
	DeleteStudent(param.studentGuid)

func ListenGameOver(param: BaseParam.ScoreParam):
	print("GameOver. Your score is: {score}".format({"score":param.score}))
	CleanUp()
	return
	
func ListenWelcome(param: BaseParam):
	var stu=CreateAndPlaceStudent()
	Emit(EventName.STUDENT_WELCOMED, BaseParam.NewStudentParam.new(stu.position["col"], stu.position["row"], stu.guid))
	return

func CreateSeasonTimer():
	seasonTimer = Timer.new()
	seasonTimer.one_shot = false
	seasonTimer.timeout.connect(_on_seasonTimer_timeout)

func InitGame():
	allModels = Array([], TYPE_OBJECT, &"Object", BaseModel)

	CreateSeasonTimer()
	allModels.append(CreateWorld())
	allModels.append(CreateDoor())
	allModels.append(CreateTeacher())
	add_child(seasonTimer)
	BeginGame()
	
func BeginGame():
	for model in allModels:
		model.Reset()
	currentSeason = BaseParam.SEASON.ASPARAGUS
	seasonTimer.start(TIME_SEASON)
	Emit(EventName.WINDOW_CHANGED, BaseParam.WindowChangedParam.new(false,1))
	Emit(EventName.WINDOW_CHANGED, BaseParam.WindowChangedParam.new(false,2))
	Emit(EventName.WINDOW_CHANGED, BaseParam.WindowChangedParam.new(false,3))
	Emit(EventName.WINDOW_CHANGED, BaseParam.WindowChangedParam.new(false,4))
	Emit(EventName.DOOR_CHANGED, BaseParam.DoorEventParam.new(true))
	Emit(EventName.WELCOME_AVAILABLE, BaseParam.WelcomeAvailableParam.new(false))
	Emit(EventName.SEASON_CHANGED, BaseParam.SeasonParam.new(currentSeason))
	ListenWelcome(BaseParam.new())
	ListenWelcome(BaseParam.new())
	ListenWelcome(BaseParam.new())
	ListenWelcome(BaseParam.new())
	ListenWelcome(BaseParam.new())
	ListenWelcome(BaseParam.new())

func _on_seasonTimer_timeout():
	currentSeason = (currentSeason + 1) % BaseParam.SEASON.size() 
	Emit(EventName.SEASON_CHANGED, BaseParam.SeasonParam.new(currentSeason))


# Called when the node enters the scene tree for the first time.
func _ready():
	AddListener(EventName.SEASON_CHANGED,$Classroom.listenSeason)
	AddListener(EventName.DOOR_CHANGED, $Door.ListenDoorEvent)
	AddListener(EventName.WELCOME_AVAILABLE, $Welcome.ListenWelcomeAvailable)
	AddListener(EventName.WINDOW_CHANGED, $Windows.ListenWindowChanged)
	AddListener(EventName.WINDOW_HARVEST_CHANGED, $Windows.ListenWindowHarvestChanged)
	$Teacher.LoadCallEvent(Emit)
	AddListener(EventName.STATION_CHANGED, $Teacher.ListenStationChanged)
	AddListener(EventName.STUDENT_WELCOMED, $Desks.ListenStudentWelcomed)
	AddListener(EventName.STUDENT_CHANGED, $Desks.ListenStudentChanged)
	InitGame()


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta):
	for st in studentToFree:
		st.free()
	pass
