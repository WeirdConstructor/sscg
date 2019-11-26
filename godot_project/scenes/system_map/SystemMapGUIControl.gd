extends Control

# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var fps_label

func _input(event):
	if event is InputEventKey:
		if event.is_pressed():
			if event.get_scancode() == KEY_BACKSPACE:
				self.get_node("GUIDrawing").on_input(-1)
			else:
				self.get_node("GUIDrawing").on_input(event.get_unicode())
	elif event is InputEventMouseMotion:
		var mp = self.get_local_mouse_position()
		self.get_node("GUIDrawing").on_mouse_move(mp.x, mp.y)
	elif event is InputEventMouseButton:
		if event.is_pressed():
			var mp = self.get_local_mouse_position()
			self.get_node("GUIDrawing").on_mouse_click(mp.x, mp.y)

func _process(delta):
	fps_label.text = "FPS: " + str(Engine.get_frames_per_second())

func _ready():
	fps_label = self.get_node("FPS")
	self.get_node("GUIDrawing").on_resize(self.rect_size.x, self.rect_size.y);

func _on_GUI_resized():
	self.get_node("GUIDrawing").on_resize(self.rect_size.x, self.rect_size.y);
	
