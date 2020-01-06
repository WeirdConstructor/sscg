extends AudioStreamPlayer

var is_playing = false
var disable_timer = Timer.new()
var enable_tween = Tween.new()

func _ready():
	disable_timer.connect("timeout", self, "disable_beam_now")
	enable_tween.interpolate_method(self, "foo", -60.0, 0.0, 1.0, Tween.TRANS_LINEAR, Tween.EASE_IN)
	enable_tween.interpolate_property(self, "volume_db", -10.0, 0.0, 1, Tween.TRANS_LINEAR, Tween.EASE_IN)

func foo(f):
	print("FOOO", f)
	
func enable_beam():
	disable_timer.stop()
	print("ENABLE BEAM")
	if not is_playing:
		self.volume_db = -60.0
#		enable_tween.set_active(true)
#		enable_tween.reset(self)
		enable_tween.start()
		self.play()
		is_playing = true

func disable_beam_now():
	is_playing = false
	self.stop()
	
func disable_beam():
	if is_playing:
		disable_timer.start(0.7)
