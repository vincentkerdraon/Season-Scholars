class_name WorldModel extends BaseModel

		#Score
		#->Welcome(Row)
		#->OgreFed(Score)
		#->DoorDestroed()
		#SeasonChanged(Season)->
		#GameOver(Score)->

### MEMBERS

var score:int

### FUNC

func _init(arg, addListener: Callable):
	addListener.call(PipeOverlord.EventName.OGRE_FED, ListenOgreFed)
	addListener.call(PipeOverlord.EventName.DOOR_DESTROYED, ListenDoorDestroyed)
	super(arg)

func Reset():
	score = 0
	

#func ListenWelcome(Student):
#what does it does?

func ListenOgreFed(param: BaseParam.ScoreParam):
	score += param.score

func ListenDoorDestroyed(_param: BaseParam):
	emitCallback.call(PipeOverlord.EventName.GAME_OVER, BaseParam.ScoreParam.new(score))
