class_name BaseParam extends Object


### UTIL

enum STUDENT_COLS{
	COL0=0, COL1, COL2
}

enum STATION{
	NONE=-1, ST_COL0=0, ST_COL1= 1, ST_COL2=2
}

enum SEASON{
	ASPARAGUS, CHERRY, MUSHROOM, LEMON
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
	func _init(know, guid):
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
	func _init(s1:SEASON, s2:SEASON, s3:SEASON, i:int):
		windowId=i
		sea1=s1
		sea2=s2
		sea3=s3
