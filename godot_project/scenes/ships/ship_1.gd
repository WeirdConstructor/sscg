extends Spatial

export var no_fuel = false
export var docked = false
export var speed = 0
var engine_on_fract = 0.0
export var engine_on_secs = 0
var thruster_speed = 0
var emergency_warning_timer
var back_engine_particles
var back_engine_light

var safe_dock_speed         = 0.1;
var max_speed               = 2.0;
var no_fuel_max_speed       = 0.2;
var accel                   = 0.1;
var decel                   = 0.25;
var max_space_wind_friction = 0.001;

var drone_active = false;
var view_sensitivity = 0.3;
var yaw = 0.0;
var pitch = 0.0;


func sscg_save():
	return {
		"speed": speed,
		"engine_on_fract": engine_on_fract,
		"engine_on_secs": engine_on_secs,
		"thruster_speed": thruster_speed,
		"x": int(((self.translation.x + 1000.0) * 10000.0) / 2000.0),
		"y": int(((self.translation.z + 1000.0) * 10000.0) / 2000.0),
		"rot_z": self.rotation.y,
	}

func sscg_load(state):
	speed           = state["speed"]
	engine_on_fract = state["engine_on_fract"]
	engine_on_secs  = state["engine_on_secs"]
	thruster_speed  = state["thruster_speed"]
	self.translation.x = -1000.0 + (float(state["x"]) * 2000.0) / 10000.0
	self.translation.y = 1.2
	self.translation.z = -1000.0 + (float(state["y"]) * 2000.0) / 10000.0
	self.rotation.y = state["rot_z"]

func drone_process(delta):
	
	var cam = self.get_node("../drone")
	var forw = -cam.get_transform().basis.z;
	var righ = -cam.get_transform().basis.x;
	var motion = Vector3()
	if Input.is_action_pressed("fly_forward"):
		motion += forw.normalized()
	if Input.is_action_pressed("fly_stop"):
		motion -= forw.normalized()
	if Input.is_action_pressed("turn_left"):
		motion += righ.normalized()
	if Input.is_action_pressed("turn_right"):
		motion -= righ.normalized()
	cam.set_translation(cam.get_translation() + motion.normalized() * 5 * delta)

func _input(event):
	if event.is_action_pressed("drone"):
		drone_active = !drone_active;
		if drone_active:
			self.get_node("../drone").current = true;
			Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)
			pitch = 0
			yaw = 180
			var cam = self.get_node("../drone")
			cam.set_rotation(Vector3(deg2rad(pitch),deg2rad(yaw), 0))
			self.get_node("../drone").set_translation(self.get_node("../VoxStruct").get_translation() - Vector3(0, -1, 2))
		else:
			self.get_node("Camera").current = true;
			Input.set_mouse_mode(Input.MOUSE_MODE_VISIBLE)
	if event is InputEventMouseMotion:
		if drone_active:
			yaw = fmod(yaw - event.relative.x * view_sensitivity, 360) 
			pitch = max(min(pitch - event.relative.y * view_sensitivity, 90),-90)
			var cam = self.get_node("../drone")
			cam.set_rotation(Vector3(deg2rad(pitch),deg2rad(yaw), 0))
				
func _physics_process(delta):
	if docked:
		return
		
	if drone_active:
		self.drone_process(delta);
		return

	if Input.is_action_pressed("fly_forward"):
		speed += accel * delta;
		engine_on_fract += delta;
		back_engine_particles.emitting = true;
		back_engine_light.light_energy = 2.0
	elif Input.is_action_pressed("fly_stop"):
		speed += -decel * delta;
		engine_on_fract += delta;
		back_engine_particles.emitting = false;
		back_engine_light.light_energy = 1.0
	else:
		var friction_x = (speed / max_speed)
		var friction = (friction_x * max_space_wind_friction) + (friction_x * (0.1 * max_space_wind_friction));
		speed += -friction * delta;
		back_engine_particles.emitting = false;
		back_engine_light.light_energy = 1.0
		
	if speed < 0:
		speed = 0
	if speed > max_speed:
		speed = max_speed;

	if no_fuel && speed > no_fuel_max_speed:
		speed = no_fuel_max_speed;

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
	
	if self.translation.x > 1000.0:
		self.translation.x -= 2000.0
	if self.translation.x < -1000.0:
		self.translation.x += 2000.0
	if self.translation.z > 1000.0:
		self.translation.z -= 2000.0
	if self.translation.z < -1000.0:
		self.translation.z += 2000.0


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
	if speed > safe_dock_speed:
		self.get_parent().get_node("GUI").get_child(0).show()
		var v = self.get_global_transform().basis
		self.translation = self.translation - v.z.normalized() * (s.shape.radius * 2)
		emergency_warning_timer.start(5)
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
