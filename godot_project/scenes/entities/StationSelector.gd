extends Spatial

export var selected = false
export var system_id = 0
export var entity_id = 0
# Declare member variables here. Examples:
# var a = 2
# var b = "text"

func _process(delta):
	if selected:
		self.get_child(1).show()

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
