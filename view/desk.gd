extends Node2D

#contains the map between studentGuid (from model), and the position and so the ref to the student sprite
var students: Dictionary = {}

var currentStudentSide = 1
var currentStudentCenter = 1

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func ListenStudentGraduated(sta: BaseParam.StationParam):
	#students.erase()
	pass

func ListenStudentTeached(stu: BaseParam.StudentTeachParam):
	pass

func ListenStudentWelcomed(stu: BaseParam.NewStudentParam):
	if(students.has(stu.studentGuid)):
		printerr("Another key with same guid already exists: %s" % stu.studentGuid)
		return
	DrawNewStudent(stu.studentPosCol, stu.studentPosRow, stu.studentGuid)
	
func ListenStudentChanged(stu: BaseParam.NewStudentParam):
	if(!students.has(stu.studentGuid)):
		printerr("No key with same guid already exists: %s" % stu.studentGuid)
		return
	students[stu.studentGuid].col = stu.studentPosCol
	students[stu.studentGuid].row = stu.studentPosRow
	DrawStudent(stu.studentGuid)

func DrawNewStudent(col: int, row: int, guid: String):
	if(col==1):
		currentStudentCenter-=1
		if (currentStudentCenter == 0):
			currentStudentCenter=get_meta("stcmax")
		students[guid]=GraphicStudent.new(col, row, get_meta("stc%d" % currentStudentCenter))
	else:
		currentStudentSide-=1
		if (currentStudentSide == 0):
			currentStudentSide=get_meta("stsmax")
		students[guid]=GraphicStudent.new(col, row, get_meta("sts%d" % currentStudentSide))
	DrawStudent(guid)

func DrawStudent(guid: String):
	find_child("R%dC%d"%[students[guid].row,students[guid].col]).texture = students[guid].res

func DrawEmptyDesc(col, row):
	if col == 1:
		find_child("R%dC%d"%[row,col]).texture = get_meta("emptyc")
	else:
		find_child("R%dC%d"%[row,col]).texture = get_meta("emptys")
		

class GraphicStudent:
	var col
	var row
	var res: CompressedTexture2D
	func _init(c:int, r:int, rs: CompressedTexture2D):
		col = c
		res = rs
		row = r
