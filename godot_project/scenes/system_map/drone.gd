extends Camera

# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var vox_struct = null

func _physics_process(delta):
	var cast = self.get_child(0)
	var c = cast.get_collider()
	if c:
		var p = cast.get_collision_point()
		var cn = cast.get_collision_normal()
		#d# var n = self.get_child(1)
		#d# n.global_transform.origin = p - (0.13 * cn)
		var vox_coord = vox_struct.to_local(p - (0.1 * cn))
		var vv = Vector3(floor(vox_coord.x), floor(vox_coord.y), floor(vox_coord.z))
		vox_struct.looking_at(vv.x, vv.y, vv.z)
	else:
		vox_struct.looking_at_nothing()

# Called when the node enters the scene tree for the first time.
func _ready():
	vox_struct = self.get_node("../VoxStruct2")

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
