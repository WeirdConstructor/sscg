extends AudioStreamPlayer

var is_playing = false
var is_disabling = false
var disable_timer = Timer.new()
var enable_tween = Tween.new()
var disable_tween = Tween.new()

var pops = []

func _init():
	disable_timer.one_shot = true
	self.add_child(disable_timer)
	self.add_child(enable_tween)
	self.add_child(disable_tween)

func _ready():
	disable_timer.connect("timeout", self, "disable_beam_now")
	pops.push_back(self.get_node("../Pop1"))
	pops.push_back(self.get_node("../Pop2"))
	pops.push_back(self.get_node("../Pop3"))
	pops.push_back(self.get_node("../Pop4"))

func enable_beam():
	disable_timer.stop()
	print("ENABLE BEAM")
	if not is_playing:
		self.volume_db = -60.0
		enable_tween.remove(self)
		enable_tween.stop_all()
		enable_tween.interpolate_property(self, "volume_db", -60.0, -12.0, 0.1, Tween.TRANS_LINEAR, Tween.EASE_IN)
		enable_tween.start()
		self.play(0.0)
		is_playing = true
	elif is_disabling:
		enable_tween.remove(self)
		enable_tween.stop_all()
		enable_tween.interpolate_property(self, "volume_db", self.volume_db, -12.0, 0.1, Tween.TRANS_LINEAR, Tween.EASE_IN)
		enable_tween.start()

func disable_beam_now():
	is_playing = false
	is_disabling = false
	print("DISABLED BEAM")
	self.stop()

func play_pop():
	var found = false
	var free = []
	for i in range(0,3):
		if not pops[i].playing:
			free.push_back(pops[i])
	if free.size() <= 0:
		free = pops
	free[randi() % free.size()].play()

func disable_beam():
	print("DISABLE BEAM")
	if is_playing:
		is_disabling = true
		enable_tween.reset(self)
		enable_tween.stop_all()
		enable_tween.interpolate_property(self, "volume_db", -12.0, -60.0, 0.2, Tween.TRANS_LINEAR, Tween.EASE_IN)
		enable_tween.start()
		disable_timer.stop()
		disable_timer.start(0.7)
