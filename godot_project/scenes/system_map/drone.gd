extends Camera

# Declare member variables here. Examples:
# var a = 2
# var b = "text"

func _physics_process(delta):
	var cast = self.get_child(0);
	var c = cast.get_collider()
	var p = cast.get_collision_point()
	var n = self.get_child(1)
	n.global_transform.origin = p;
	# var local_vox = voxelgrid.to_local(p)
	# manually decode local_vox to a face...

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
