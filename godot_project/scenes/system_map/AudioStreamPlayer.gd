extends AudioStreamPlayer

# Declare member variables here. Examples:
# var a = 2
# var b = "text"

const tracks = [
	"nemeton_preview_v02.ogg",
	"Ryan_Andersen_-_07_-_Synthwave.ogg",
];

var i = 0;

func load_next():
	i = i + 1
	if i >= len(tracks):
		i = 0
	var stream = load('res://music/' + tracks[i])
	set_stream(stream)
	play()

# Called when the node enters the scene tree for the first time.
func _ready():
	load_next()

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
func _on_AudioStreamPlayer_finished():
	load_next()
