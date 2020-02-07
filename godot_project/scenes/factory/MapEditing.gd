extends Area

# Declare member variables here. Examples:
# var a = 2
# var b = "text"

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass


func _on_Area_input_event(camera, event, click_position, click_normal, shape_idx):
	#print("CLICK:", click_position, event);
	if event is InputEventMouseButton:
		var map_pos = self.get_parent().world_to_map(click_position)
		print("CL:", event.button_index, map_pos)
		self.get_parent().set_cell_item(map_pos.x, map_pos.y, map_pos.z, GridMap.INVALID_CELL_ITEM, 0)
	
