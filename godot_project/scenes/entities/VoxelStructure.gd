extends Spatial

export var selected = false
export var system_id = 0
export var entity_id = 0
export var label_name = "Station"

func on_wlambda_init():
	self.get_child(0).on_wlambda_init()

func _ready():
	pass