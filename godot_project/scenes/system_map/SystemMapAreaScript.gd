extends Area

# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var tween

func _input_event(camera, event, click_pos, click_normal, shape_idx):
	self.get_parent().get_child(0).translation = click_pos;
	# print("CLI", event);
	if event is InputEventMouseButton:
		if event.button_index == 1 && event.pressed:
			tween = get_child(0);
			var t = self.get_parent().get_child(1).translation;
			var to = Vector3(click_pos.x - 5, t.y, click_pos.z);
			tween.interpolate_property(
				self.get_parent().get_child(1), "translation",
				t, to, 0.25,
				Tween.TRANS_LINEAR, Tween.EASE_IN_OUT);
			tween.start();

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
