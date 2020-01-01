extends KinematicBody

export var drone_active = false

var vox_struct = null
var camera = null

var view_sensitivity = 0.3
var yaw = 0.0
var pitch = 0.0
var speed = 4.0
var grav = 3.0
var accel = 0.0
var jump_strength = 0.8
var jump_motion = Vector3(0, 0, 0)

var old_on_floor = false

func set_active(is_active):
	drone_active = is_active
	if drone_active:
		camera.current = true;
		Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)
		pitch = 0
		yaw = 180
		self.set_rotation(Vector3(deg2rad(pitch),deg2rad(yaw), 0))
		self.set_translation(vox_struct.get_translation() - Vector3(0, -1, 2))

func _input(event):
	if !drone_active:
		return
		
	if Input.is_action_just_pressed("fly_forward"):
		print("JUMP", self.get_transform().origin, jump_motion)
		jump_motion += Vector3(0, grav, 0) * jump_strength;
		
	if event is InputEventMouseMotion:
		yaw = fmod(yaw - event.relative.x * view_sensitivity, 360) 
		pitch = max(min(pitch - event.relative.y * view_sensitivity, 90),-90)
		self.set_rotation(Vector3(deg2rad(pitch),deg2rad(yaw), 0))

func process_movement(delta):
	var forw = -self.get_transform().basis.z;
	forw.y = 0;
	var righ = -self.get_transform().basis.x;
	var motion = Vector3()

	if Input.is_action_pressed("walk_forward"):
		motion += forw.normalized()
	if Input.is_action_pressed("fly_stop") or Input.is_action_pressed("walk_backward"):
		motion -= forw.normalized()
	if Input.is_action_pressed("turn_right"):
		motion += righ.normalized()
	if Input.is_action_pressed("turn_left"):
		motion -= righ.normalized()
	if Input.is_action_pressed("mine"):
		vox_struct.mine()
		
	#if !self.is_on_floor():
	#	if old_on_floor:
#			print("NOT ON FLOOOR!", self.get_transform().origin, jump_motion)
	jump_motion += Vector3(0, -grav, 0) * delta;
	if jump_motion.dot(Vector3(0, 1, 0)) < -10.0:
		jump_motion = Vector3(0, -10.0, 0)
			
	motion += jump_motion;
	print("MOTON", motion)
	self.move_and_slide(motion * speed, Vector3(0, 1, 0))
	
	if self.is_on_floor() or self.is_on_ceiling():
		jump_motion = Vector3(0, 0, 0);
		
	old_on_floor = self.is_on_floor()
	#self.set_translation(self.get_translation() + motion.normalized() * 5 * delta)

func process_mining_gun(delta):
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
	
func _physics_process(delta):
	if !drone_active:
		return
		
	process_movement(delta)
	process_mining_gun(delta)
	

# Called when the node enters the scene tree for the first time.
func _ready():
	vox_struct = self.get_node("../VoxStruct2")
	camera     = self.get_node("Camera")

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
