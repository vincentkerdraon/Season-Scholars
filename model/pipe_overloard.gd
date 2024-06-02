class_name PipeOverlord extends Node

enum EventName{
	TEACH, GRADUATE, WELCOME, STATION_CHANGED, STUDENT_GRADUATED, DOOR_DESTROYED, OGRE_FED, GAME_OVER, SEASON_CHANGED, DOOR_CHANGED, WELCOME_AVAILABLE, WINDOW_CHANGED, WINDOW_HARVEST_CHANGED
}

var listenerDict={}
var idStudent = 0
var allStudents = {}
var allModels: Array[BaseModel]

var seasonTimer: Timer
var currentSeason :BaseParam.SEASON

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
	print_debug("New Event emitted: {event}, with params {param}".format({"event":EventName.find_key(event), "param":param.to_string()}))
	for callback in listenerDict[event]:
		callback.call(param)
	return

func CreateTeacher()->TeacherModel:
	return TeacherModel.new(Emit,AddListener)

func CreateStudent()->StudentModel:
	var stu= StudentModel.new(Emit, AddListener, idStudent)
	allStudents[""+idStudent]=stu
	idStudent+=1
	return stu

func CreateDoor()->DoorModel:
	return DoorModel.new(Emit, AddListener, 4, 4)

func CreateWorld()->WorldModel:
	return WorldModel.new(Emit, AddListener)

func DeleteStudent(id: String):
	allStudents[id].free()

func ListenStudentGraduated(param :BaseParam.KnowledgesParam):
	DeleteStudent(param.studentGuid)

func ListenGameOver(param: BaseParam.ScoreParam):
	print("GameOver. Your score is: {score}".format({"score":param.score}))
	CleanUp()
	return
	
func ListenWelcome(param: BaseParam.StudentColParam):
	CreateStudent()
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
	seasonTimer.start(5)
	Emit(EventName.SEASON_CHANGED, BaseParam.SeasonParam.new(currentSeason))
	Emit(EventName.DOOR_CHANGED, BaseParam.DoorEventParam.new(true))
	Emit(EventName.WELCOME_AVAILABLE, BaseParam.WelcomeAvailableParam.new(false))
	Emit(EventName.WINDOW_CHANGED, BaseParam.WindowChangedParam.new(false,1))
	Emit(EventName.WINDOW_CHANGED, BaseParam.WindowChangedParam.new(false,2))
	Emit(EventName.WINDOW_CHANGED, BaseParam.WindowChangedParam.new(false,3))
	Emit(EventName.WINDOW_CHANGED, BaseParam.WindowChangedParam.new(false,4))

func _on_seasonTimer_timeout():
	currentSeason = (currentSeason + 1) % BaseParam.SEASON.size() 
	Emit(EventName.SEASON_CHANGED, BaseParam.SeasonParam.new(currentSeason))


# Called when the node enters the scene tree for the first time.
func _ready():
	AddListener(EventName.SEASON_CHANGED,$Classroom.listenSeason)
	AddListener(EventName.DOOR_CHANGED, $Door.ListenDoorEvent)
	AddListener(EventName.WELCOME_AVAILABLE, $Welcome.ListenWelcomeAvailable)
	AddListener(EventName.WINDOW_CHANGED, $Windows.ListenWindowChanged)
	InitGame()


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta):
	pass
