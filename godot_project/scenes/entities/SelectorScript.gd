extends Area

# Declare member variables here. Examples:
# var a = 2
# var b = "text"

func _area_entered(area):
	print("AREA ENTE");

func _mouse_entered():
	print("MOUSE ENTER")
	self.get_parent().get_child(1).show()
	
func _mouse_exited():
	print("MOUSE ENTER")
	self.get_parent().get_child(1).hide()
	
func _input_event(camera, event, click_pos, cl, pre):
	print("EVE", event, get_parent().system_id, get_parent().entity_id);
	if event is InputEventMouseButton:
		self.get_parent().selected = true

# Called when the node enters the scene tree for the first time.
func _ready():
	self.get_parent().get_child(1).hide()

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
