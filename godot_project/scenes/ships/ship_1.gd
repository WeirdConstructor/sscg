extends Spatial

# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var speed = 0
var thruster_speed = 0
#func _physics_process(delta):
	#if Input.is_action_pressed("move_forward"):
		#speed.
	
#func _input(event):
	#if event is InputEventMouseMotion:
		#self.rotate_y(deg2rad(-event.relative.x * 0.3));
		
func _process(delta):
	if Input.is_action_pressed("fly_forward"):
		speed += 0.1 * delta;
	else:
		speed += -0.1 * delta;
	if speed < 0:
		speed = 0
	if speed > 2:
		speed = 2;
		
	if Input.is_action_pressed("turn_left"):
		thruster_speed += -0.1 * delta
	elif Input.is_action_pressed("turn_right"):
		thruster_speed += 0.1 * delta
	else:
		if thruster_speed > 0:
			thruster_speed += -0.1 * delta
		elif thruster_speed < 0:
			thruster_speed += 0.1 * delta
		if abs(thruster_speed) < 0.01:
			thruster_speed = 0.0
			
	if thruster_speed < -1:
		thruster_speed = -1
	elif thruster_speed > 1:
		thruster_speed = 1

	var v = self.get_global_transform().basis;
	self.translation = self.translation + v.z.normalized() * speed
	self.rotate_y(deg2rad(2 * thruster_speed))
#	var n2 = self.get_node("RayCast2").get_collider();
#	print("DO2:", n2)

# Called when the node enters the scene tree for the first time.
func _ready():
	Input.set_mouse_mode(Input.MOUSE_MODE_HIDDEN);
	pass # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass


func _on_Area_area_shape_entered(area_id, area, area_shape, self_shape):
	print("SYS: ", area.get_parent().system_id)
	print("SYSE: ", area.get_parent().entity_id)
