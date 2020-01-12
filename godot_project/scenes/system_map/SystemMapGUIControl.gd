extends Control

# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var fps_label

func _input(event):
	if event is InputEventKey:
		if event.is_pressed():
			if event.get_scancode() == KEY_BACKSPACE:
				self.find_node("GUIDrawing").on_input(-1)
			else:
				self.find_node("GUIDrawing").on_input(event.get_unicode())
	elif event is InputEventMouseMotion:
		var mp = self.get_local_mouse_position()
		self.find_node("GUIDrawing").on_mouse_move(
		    mp.x, mp.y,
			Input.is_mouse_button_pressed(BUTTON_LEFT),
			Input.is_mouse_button_pressed(BUTTON_RIGHT),
			Input.is_key_pressed(KEY_CONTROL),
			Input.is_key_pressed(KEY_SHIFT))
	elif event is InputEventMouseButton:
		if event.is_pressed():
			var mp = self.get_local_mouse_position()
			self.find_node("GUIDrawing").on_mouse_click(mp.x, mp.y)

func _process(delta):
	fps_label.text = "FPS: " + str(Engine.get_frames_per_second())

func _ready():
	fps_label = self.get_node("FPS")
	self.find_node("GUIDrawing").on_resize(self.rect_size.x, self.rect_size.y);
	self.find_node("ViewportContainer").rect_size = self.rect_size

func _on_GUI_resized():
	self.find_node("GUIDrawing").on_resize(self.rect_size.x, self.rect_size.y);
	self.find_node("ViewportContainer").rect_size = self.rect_size
	
