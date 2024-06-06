extends Node2D

#contains the map between studentGuid (from model), and the position and so the ref to the student sprite
var students: Dictionary = {}

var currentStudentSide = 1
var currentStudentCenter = 1

var harvests: Array[CompressedTexture2D] = []

# Called when the node enters the scene tree for the first time.
func _ready():
	harvests = [ get_meta("hasp"), get_meta("hche"),get_meta("hmus"),get_meta("hlem")]
	pass # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func ListenStudentGraduated(sta: BaseParam.KnowledgesParam):
	DrawEmptyDesc(students[sta.studentGuid].col, students[sta.studentGuid].row)
	pass

func ListenStudentTaught(stu: BaseParam.StudentTaughtParam):
	if(!students.has(stu.studentGuid)):
		printerr("No key with same guid already exists: %s" % stu.studentGuid)
		return
	students[stu.studentGuid].knowledges = stu.acquiredKnowledge
	if(students[stu.studentGuid].row == 0):
		DrawLessonLearned(stu.studentGuid)
	pass

func ListenStudentWelcomed(stu: BaseParam.NewStudentParam):
	if(students.has(stu.studentGuid)):
		printerr("Another key with same guid already exists: %s" % stu.studentGuid)
		return
	DrawNewStudent(stu.studentPosCol, stu.studentPosRow, stu.studentGuid)
	
func ListenStudentChanged(stu: BaseParam.ChangeStudentParam):
	if(!students.has(stu.studentGuid)):
		printerr("No key with same guid already exists: %s" % stu.studentGuid)
		return
	DrawEmptyDesc(stu.studentPosCol, stu.studentPosRowPrev)
	students[stu.studentGuid].col = stu.studentPosCol
	students[stu.studentGuid].row = stu.studentPosRowNew
	DrawStudent(stu.studentGuid)

func DrawNewStudent(col: int, row: int, guid: String):
	if(col==1):
		currentStudentCenter-=1
		if (currentStudentCenter == 0):
			currentStudentCenter=get_meta("stcmax")
		students[guid]=GraphicStudent.new(col, row, get_meta("stc%d" % currentStudentCenter), get_meta("stc%dr0" % currentStudentCenter), [])
	else:
		currentStudentSide-=1
		if (currentStudentSide == 0):
			currentStudentSide=get_meta("stsmax")
		students[guid]=GraphicStudent.new(col, row, get_meta("sts%d" % currentStudentSide), get_meta("sts%dr0" % currentStudentSide), [])
	DrawStudent(guid)

func DrawStudent(guid: String):
	var res = students[guid].res
	if(students[guid].row == 0) : 
		res = students[guid].resR0
	find_child("R%dC%d"%[students[guid].row,students[guid].col]).texture = res
	if(students[guid].row == 0) : 
		DrawLessonLearned(guid)

func DrawEmptyDesc(col, row):
	var d = find_child("R%dC%d" % [row,col])
	if(row == 0):
		var h0 = d.find_child("Harvest0")
		var h1 = d.find_child("Harvest1")
		var h2 = d.find_child("Harvest2")
		h0.visible = false
		h1.visible = false
		h2.visible = false
	if col == 1:
		d.texture = get_meta("emptyc")
	else:
		d.texture = get_meta("emptys")
	
func DrawLessonLearned(guid: String):
	var k = students[guid].knowledges
	var d = find_child("R%dC%d" % [students[guid].row,students[guid].col])
	var h0 = d.find_child("Harvest0")
	var h1 = d.find_child("Harvest1")
	var h2 = d.find_child("Harvest2")
	h0.visible = false
	h1.visible = false
	h2.visible = false
	if(k.size()>0):
		h0.texture = harvests[k[0]]
		h0.visible = true
	if(k.size()>1):
		h1.texture = harvests[k[1]]
		h1.visible = true
	if(k.size()>2):
		h2.texture = harvests[k[2]]
		h2.visible = true
	pass

class GraphicStudent:
	var col:int
	var row:int
	var res: CompressedTexture2D
	var resR0: CompressedTexture2D
	var knowledges: Array[BaseParam.SEASON]
	func _init(c:int, r:int, rs: CompressedTexture2D, r0: CompressedTexture2D, skills:Array[BaseParam.SEASON] ):
		col = c
		res = rs
		row = r
		resR0 = r0
		knowledges = skills
