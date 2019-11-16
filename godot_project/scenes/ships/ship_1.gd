extends Spatial

# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var speed = 0
var thruster_speed = 0
var emergency_warning_timer
var back_engine_particles
var back_engine_light
#func _physics_process(delta):
	#if Input.is_action_pressed("move_forward"):
		#speed.
	
#func _input(event):
	#if event is InputEventMouseMotion:
		#self.rotate_y(deg2rad(-event.relative.x * 0.3));
		
func _process(delta):
	if Input.is_action_pressed("fly_forward"):
		speed += 0.1 * delta;
		back_engine_particles.emitting = true;
		back_engine_light.light_energy = 2.0
	elif Input.is_action_pressed("fly_stop"):
		speed += -0.1 * delta;
		back_engine_particles.emitting = false;
		back_engine_light.light_energy = 1.0
	else:
		speed += -0.01 * delta;
		back_engine_particles.emitting = false;
		back_engine_light.light_energy = 1.0
	if speed < 0:
		speed = 0
	if speed > 2:
		speed = 2;
		
	if Input.is_action_pressed("turn_left"):
		thruster_speed += -0.1 * delta + -(2.0 * speed) * delta
	elif Input.is_action_pressed("turn_right"):
		thruster_speed += 0.1 * delta + (2.0 * speed) * delta
	else:
		if thruster_speed > 0:
			thruster_speed += -0.02  * delta
		elif thruster_speed < 0:
			thruster_speed += 0.02 * delta
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
	back_engine_particles = self.find_node("BackEngineParticles")
	back_engine_light     = self.find_node("BackEngineLight")
	Input.set_mouse_mode(Input.MOUSE_MODE_HIDDEN);
	emergency_warning_timer = Timer.new()
	emergency_warning_timer.connect("timeout", self, "_on_hide_warning")
	self.add_child(emergency_warning_timer)
	pass # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass

func _on_hide_warning():
	self.get_parent().get_node("GUI").get_child(0).hide()

func _on_Area_area_shape_entered(area_id, area, area_shape, self_shape):
	print("SYS: ", area.get_parent().system_id)
	print("SYSE: ", area.get_parent().entity_id)
	var s = area.get_child(0)
	if speed > 0.1:
		self.get_parent().get_node("GUI").get_child(0).show()
		var v = self.get_global_transform().basis;
		self.translation = self.translation + v.z.normalized() * (s.shape.radius * 2)
		emergency_warning_timer.start(2)

func _on_Area_area_shape_exited(area_id, area, area_shape, self_shape):
	pass
