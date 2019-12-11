extends MultiMeshInstance

var ooo = 0.0;
# Declare member variables here. Examples:
# var a = 2
# var b = "text"

# Called when the node enters the scene tree for the first time.
func _ready():
	var mm = self.multimesh;
	mm.instance_count = 0;
	mm.transform_format = MultiMesh.TRANSFORM_3D;
	mm.instance_count = 2;
	#var t = Transform();
	#t.translated(Vector3(10.0, 10.0, 10.0));
	#mm.set_instance_transform(0, t);
	#t.scaled(Vector3(1.0, 2.0, 1.0));
	#mm.set_instance_transform(1, t2);
	#mm.set_instance_transform(0, t);
	#mm.set_instance_transform(3, t2);

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	ooo += delta;
	var mm = self.multimesh;
	var t = Transform();
	#t = t.translated(Vector3(10.0 + ooo * 2, 10.0, 10.0));
	t = t.translated(Vector3(-999 + ooo * 1, 1, -999));
	#mm.set_instance_transform(0, t);
	##t = t.scaled(Vector3(1.0, 2.0, 1.0));
	#mm.set_instance_transform(1, t2);
	mm.set_instance_transform(0, t);
	#mm.set_instance_transform(1, t.translated(Vector3(5.0, 1.0, 5.0)));
	#mm.set_instance_transform(1, t);
	self.multimesh = mm

