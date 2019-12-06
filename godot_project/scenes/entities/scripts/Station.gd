extends Spatial

# Declare member variables here. Examples:
# var a = 2
# var b = "text"

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
#	var t = get_transform()
	#t.origin += Vector3(delta, 0, 0)
	#set_transform(t)
	var rot_speed = deg2rad(2)
	rotate_y(rot_speed * delta)
