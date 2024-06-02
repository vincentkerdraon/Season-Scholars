extends Node2D


# Called when the node enters the scene tree for the first time.
func _ready():
	#ListenWindowHarvestChanged(BaseParam.WindowHarvestChangedParam.new(1, BaseParam.SEASON.ASPARAGUS, BaseParam.SEASON.ASPARAGUS, BaseParam.SEASON.ASPARAGUS))
	#ListenWindowHarvestChanged(BaseParam.WindowHarvestChangedParam.new(2, BaseParam.SEASON.CHERRY, BaseParam.SEASON.CHERRY,-1))
	#ListenWindowHarvestChanged(BaseParam.WindowHarvestChangedParam.new(3, BaseParam.SEASON.MUSHROOM,-1, BaseParam.SEASON.MUSHROOM))
	#ListenWindowHarvestChanged(BaseParam.WindowHarvestChangedParam.new(4, BaseParam.SEASON.LEMON, BaseParam.SEASON.LEMON, BaseParam.SEASON.LEMON))
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
	
func GetWindows(id: int)->Node2D:
	match id:
		1:
			return $Window1
		2:
			return $Window2
		3:
			return $Window3
		4: 
			return $Window4
		_:
			printerr("Windows not reconized %d" % id)
			return null

func GetHarvestSeason(season: BaseParam.SEASON)->CompressedTexture2D:
	match season:
		BaseParam.SEASON.ASPARAGUS:
			return get_meta("harvestAsparagus")
		BaseParam.SEASON.CHERRY:
			return get_meta("harvestCherry")
		BaseParam.SEASON.MUSHROOM:
			return get_meta("harvestMushroom")
		BaseParam.SEASON.LEMON:
			return get_meta("harvestLemon")
		_:
			printerr("Season not reconized %d"%season)
			return null
			

func DisplayWindowSprite(id:int, meta: CompressedTexture2D):
	var window = GetWindows(id)
	window.find_child("WindowSprite").texture = meta
	window.find_child("WindowSprite").visible=true
	window.find_child("WindowHarvest1").visible=false
	window.find_child("WindowHarvest2").visible=false
	window.find_child("WindowHarvest3").visible=false

func DisplayWindowHarvestSprite(id:int, s1: CompressedTexture2D, s2: CompressedTexture2D, s3: CompressedTexture2D):

	var window = GetWindows(id)
	if(window == null):
		return
	window.find_child("WindowSprite").visible=false
	window.find_child("WindowHarvest1").visible=true
	window.find_child("WindowHarvest1").texture=s1
	window.find_child("WindowHarvest2").visible=true
	window.find_child("WindowHarvest2").texture=s2
	window.find_child("WindowHarvest3").visible=true
	window.find_child("WindowHarvest3").texture=s3

func ListenWindowChanged(param:BaseParam.WindowChangedParam):
	if(param.isOpen):
		DisplayWindowSprite(param.windowId, get_meta("Open"))
	else:
		DisplayWindowSprite(param.windowId, get_meta("closed"))
		
func ListenWindowHarvestChanged(param: BaseParam.WindowHarvestChangedParam):
	DisplayWindowHarvestSprite(param.windowId, GetHarvestSeason(param.sea1), GetHarvestSeason(param.sea2), GetHarvestSeason(param.sea3))
