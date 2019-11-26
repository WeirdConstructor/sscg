extends Spatial

export var selected = false
export var system_id = 0
export var entity_id = 0
export var label_name = "Station"

func _process(delta):
	if selected:
		self.get_child(1).show()

func _ready():
	pass # Replace with function body.

