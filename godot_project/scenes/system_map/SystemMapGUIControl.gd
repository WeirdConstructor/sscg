extends Control

# Declare member variables here. Examples:
# var a = 2
# var b = "text"

func _input(event):
	if event is InputEventKey:
		self.get_node("GUIDrawing").on_input(event.get_unicode())
	elif event is InputEventMouseMotion:
		var mp = self.get_local_mouse_position()
		self.get_node("GUIDrawing").on_mouse_move(mp.x, mp.y)

func _gui_input(event):
	print("EVENT ", event)
	if event is InputEventKey:
		print("U ", event.get_unicode())

# Called when the node enters the scene tree for the first time.
func _ready():
	self.get_node("GUIDrawing").on_resize(self.rect_size.x, self.rect_size.y);

func _on_GUI_resized():
	self.get_node("GUIDrawing").on_resize(self.rect_size.x, self.rect_size.y);
	
