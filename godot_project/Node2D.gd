extends Node2D

# Declare member variables here. Examples:
# var a = 2
# var b = "text"

#export (Texture) var texture setget _set_texture

#func _set_texture(value)
#    texture = value
#	update()

# Called when the node enters the scene tree for the first time.
func _draw():
     draw_rect(Rect2(0, 0, 100, 100), Color("#ff00ff"), true)

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
