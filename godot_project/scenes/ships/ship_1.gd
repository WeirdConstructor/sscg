extends Spatial

export var docked = false
export var speed = 0
var engine_on_fract = 0.0
export var engine_on_secs = 0
var thruster_speed = 0
var emergency_warning_timer
var back_engine_particles
var back_engine_light

func _process(delta):
	if docked:
		return

	if Input.is_action_pressed("fly_forward"):
		speed += 0.1 * delta;
		engine_on_fract += delta;
		back_engine_particles.emitting = true;
		back_engine_light.light_energy = 2.0
	elif Input.is_action_pressed("fly_stop"):
		speed += -0.25 * delta;
		engine_on_fract += delta;
		back_engine_particles.emitting = false;
		back_engine_light.light_energy = 1.0
	else:
		speed += -0.03 * delta;
		back_engine_particles.emitting = false;
		back_engine_light.light_energy = 1.0
	if speed < 0:
		speed = 0
	if speed > 2:
		speed = 2;
		
	while engine_on_fract > 1.0:
		engine_on_secs += 1
		engine_on_fract -= 1.0
		
	if Input.is_action_pressed("turn_left"):
		thruster_speed += -0.1 * delta + -(0.5 * speed) * delta
	elif Input.is_action_pressed("turn_right"):
		thruster_speed += 0.1 * delta + (0.5 * speed) * delta
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

func _ready():
	back_engine_particles = self.find_node("BackEngineParticles")
	back_engine_light     = self.find_node("BackEngineLight")
	#Input.set_mouse_mode(Input.MOUSE_MODE_HIDDEN);
	emergency_warning_timer = Timer.new()
	emergency_warning_timer.connect("timeout", self, "_on_hide_warning")
	self.add_child(emergency_warning_timer)

func _on_hide_warning():
	self.get_parent().get_node("GUI").get_child(0).hide()

func _on_Area_area_shape_entered(area_id, area, area_shape, self_shape):
	var s = area.get_child(0)
	if speed > 0.1:
		self.get_parent().get_node("GUI").get_child(0).show()
		var v = self.get_global_transform().basis
		self.translation = self.translation - v.z.normalized() * (s.shape.radius * 2)
		emergency_warning_timer.start(2)
		speed = 0
		thruster_speed = 0
		self.get_parent().on_ship_arrived(true, area.get_parent().system_id, area.get_parent().entity_id)
	else:
		var v = self.get_global_transform().basis
		self.translation = self.translation - v.z.normalized() * (s.shape.radius * 0.5)
		area.get_parent().selected = true
		speed = 0
		thruster_speed = 0
		self.get_parent().on_ship_arrived(false, area.get_parent().system_id, area.get_parent().entity_id)

func _on_Area_area_shape_exited(area_id, area, area_shape, self_shape):
	pass
