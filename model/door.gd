class_name DoorModel extends BaseModel
		#Need[][] Ogres
		#->Attack()
		#->SeasonChanged(Season)
		#->StudentGraduated(Knowledge[])
		#DoorDestroyed()->
		#OgreFed(Score)->
		
### MEMBERS

var monsters: Array[MonsterModel] = []
var difficulty:int

const MAX_NUMBER_NEED_PER_MONSTER = 4
const NB_MONSTER_CREATED_BEFURE_UPPING_DIFF = 10

var seasonBeforeAttack :int # Number of season before the current monster will begin to crush the door
var seasonCalmAdded :int # Number of calm season each time you just beat a monster(+ report of previous unused ones)

var seasonMonsterAdded: int # Number of season before adding a new monster
var currentSeasonForMonsterAdded:int

var DECOUNT :int
var MONSTER_ADD: int
### FUNC

func _init(arg, addListener: Callable, decount: int, monsterAdd: int):
	addListener.call(PipeOverlord.EventName.SEASON_CHANGED, ListenSeasonChanged)
	addListener.call(PipeOverlord.EventName.STUDENT_GRADUATED, ListenStudentGraduated)
	DECOUNT = decount
	MONSTER_ADD = monsterAdd
	super(arg)
	
func Reset():
	seasonBeforeAttack = DECOUNT
	seasonCalmAdded = DECOUNT
	seasonMonsterAdded = MONSTER_ADD
	difficulty=1
	CreateMonster()
	
func CreateMonster():
	print_debug("New Monster Creation")
	var newMonster = MonsterModel.new([],difficulty)
	for need in min(difficulty, MAX_NUMBER_NEED_PER_MONSTER):
		var season= BaseParam.SEASON.values()[randi() % BaseParam.SEASON.size()]
		if(!newMonster.Search(season)):
			newMonster.needs.append(BaseParam.KnowledgeParam.new(season,0))
		newMonster.DeltaValue(season, 1)
		print_debug("New Monster: Season: {season} / Value: {value}".format({"season":season, "value": newMonster.GetMonsterNeed(season).value}))
	monsters.append(newMonster)
	currentSeasonForMonsterAdded += seasonMonsterAdded
	if difficulty%NB_MONSTER_CREATED_BEFURE_UPPING_DIFF:
		seasonCalmAdded=min (1, seasonCalmAdded-1)
		seasonMonsterAdded=min (1, seasonMonsterAdded-1)
	difficulty+=1
	

func ListenSeasonChanged(_season :BaseParam.SeasonParam):
	seasonBeforeAttack -=1
	currentSeasonForMonsterAdded -=1
	if seasonBeforeAttack == 0:
		print_debug("Game Over - Door Destroyed by HUUUUNNGRY MONSTERS")
		emitCallback.call(PipeOverlord.EventName.DOOR_DESTROYED, BaseParam.new())
	if currentSeasonForMonsterAdded == 0:
		CreateMonster()
	
func ListenStudentGraduated(knowledge: BaseParam.KnowledgesParam):
	if monsters.size()>0 :
		var defeat = true;
		for need in monsters[0].needs:
			for cook in knowledge.knowledges:
				if(cook.season == need.season):
					need.value -= cook.value
			if(need.value>0):
				defeat = false
		if(defeat):
			var monster = monsters.pop_front()
			emitCallback.call(PipeOverlord.EventName.OGRE_FED, BaseParam.ScoreParam.new(monster.score))
			monster.queue_free()
			seasonBeforeAttack += seasonCalmAdded

class MonsterModel extends Object:
	var needs:Array[BaseParam.KnowledgeParam] 
	var score: int
	
	func _init(n: Array[BaseParam.KnowledgeParam], s:int):
		needs = n
		score = s
	
	func Search(s:BaseParam.SEASON)->bool:
		for n in needs:
			if(n.season ==s):
				return true
		return false
	
	func GetMonsterNeed(s:BaseParam.SEASON)->BaseParam.KnowledgeParam:
		for n in needs:
			if(n.season ==s):
				return n
		return;
	
	func DeltaValue(s: BaseParam.SEASON, d:int):
		for n in needs:
			if(n.season ==s):
				n.value+=d
		return;
