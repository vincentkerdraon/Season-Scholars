class_name BaseParam extends Object


### UTIL

enum STUDENT_COLS{
	COL_LEFT=0, COL_CENTER, COL_RIGHT
}

enum STATION{
	NONE=-1, ST_COL_LEFT=0, ST_COL_CENTER= 1, ST_COL_RIGHT=2, WELCOME
}

enum SEASON{
	ASPARAGUS, CHERRY, MUSHROOM, LEMON
}

enum DIR{
	UP, UPRIGHT, RIGHT, RIGHTDOWN, DOWN, DOWNLEFT, LEFT, LEFTUP, NONE
}

static func IsValidStationToStudentCols(station) -> bool: 
	if(station<0):
		printerr("Trying to cast Station to Student Cols in a incorrect Station ({station})".format({"station":station}))
		return false
	if(station>STUDENT_COLS.size()):
		printerr("Trying to cast Station to Student Cols in a incorrect Station ({station})".format({"station":station}))
		return false
	return true

static func StationToStudentCols(station) -> STUDENT_COLS:	
	return STUDENT_COLS.find_key(station)

static func GetDirFromVector2Snapped(v :Vector2i)->DIR:
	match (v):
		Vector2i(0,1):
			return DIR.DOWN
		Vector2i(1,1):
			return DIR.RIGHTDOWN
		Vector2i(1,0):
			return DIR.RIGHT
		Vector2i(1,-1):
			return DIR.UPRIGHT
		Vector2i(0,-1):
			return DIR.UP
		Vector2i(-1,-1):
			return DIR.LEFTUP
		Vector2i(-1,0):
			return DIR.LEFT
		Vector2i(-1,1):
			return DIR.DOWNLEFT
		Vector2i(0,0):
			return DIR.NONE
		_: 
			printerr("Direction not reconized: %v" % v)
			return DIR.NONE

### SUBCLASS

class StudentColParam extends BaseParam:
	var studentCol:STUDENT_COLS
	func _init(col :STUDENT_COLS):
		studentCol=col

class StationParam extends BaseParam:
	var station: STATION
	func _init(sta :STATION):
		station=sta
	
class SeasonParam extends BaseParam:
	var season: SEASON
	func _init(sea: SEASON):
		season=sea

class KnowledgeParam extends BaseParam:
	var season: SEASON
	var value: int = 1
	func _init(sea: SEASON, val: int):
		season=sea
		value=val

class KnowledgesParam extends BaseParam:
	var knowledges: Array[KnowledgeParam]
	var studentGuid: String
	func _init(know:Array[KnowledgeParam], guid:String):
		knowledges = know
		studentGuid = guid

class ScoreParam extends BaseParam:
	var score: int
	func _init(s:int):
		score = s
		
class DoorEventParam extends BaseParam:
	var isProtected: bool
	func _init(p:bool):
		isProtected = p

class WelcomeAvailableParam extends BaseParam:
	var isAvailable: bool
	func _init(a:bool):
		isAvailable = a

class WindowChangedParam extends BaseParam:
	var isOpen: bool
	var windowId: int
	func _init(o:bool, i:int):
		isOpen = o
		windowId=i

class WindowHarvestChangedParam extends BaseParam:
	var sea1: SEASON
	var sea2: SEASON
	var sea3: SEASON
	var windowId: int
	func _init(i:int, s1:SEASON, s2:SEASON, s3:SEASON):
		windowId=i
		sea1=s1
		sea2=s2
		sea3=s3

class PlayerActionParam extends BaseParam:
	var longAction: bool
	var shortAction: bool
	func _init(s: bool, l: bool):
		longAction = l
		shortAction = s

class StudentTeachParam extends BaseParam:
	var studentPosCol: STUDENT_COLS
	var studentGuid: String
	var acquiredKnowledge: Array[SEASON]

class NewStudentParam extends BaseParam:
	var studentPosCol: STUDENT_COLS
	var studentGuid: String
	var studentPosRow: int
	func _init(col: STUDENT_COLS, row: int, guid: String):
		studentGuid = guid
		studentPosCol = col
		studentPosRow = row
