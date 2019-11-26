extends Spatial

var tmout = 0

func _ready():
	update_label()

func _process(delta):
	tmout += delta
	if (tmout > 10.0):
		tmout = 0
		self.update_label()

func update_label():
	var lbl = self.get_parent().label_name
	print("UPDATE LABEL: ", lbl)
	self.get_child(0).get_child(0).text = lbl
	self.get_child(0).get_child(0).update()
