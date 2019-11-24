extends MeshInstance

func _ready():
	var line_width = 0.8
	var length     = 2000
	var st = SurfaceTool.new()
	st.begin(Mesh.PRIMITIVE_TRIANGLES)
	st.add_color(Color(1, 0, 0))
	st.add_normal(Vector3(0, 1, 0))
	
	self.translate(Vector3(-(length / 2), 0, -(length / 2)))
	
	var line_count = 200.0
	var grid_offs = (length - line_width) / (line_count - 1.0)
	for i in range(line_count):
		st.add_vertex(Vector3(length, 0,              i * grid_offs))
		st.add_vertex(Vector3(length, 0, line_width + i * grid_offs))
		st.add_vertex(Vector3(0,      0, line_width + i * grid_offs))
		st.add_vertex(Vector3(0,      0, line_width + i * grid_offs))
		st.add_vertex(Vector3(0,      0,              i * grid_offs))
		st.add_vertex(Vector3(length, 0,              i * grid_offs))
		
	for i in range(line_count):
		st.add_vertex(Vector3(line_width + i * grid_offs, 0, 0))
		st.add_vertex(Vector3(line_width + i * grid_offs, 0, length))
		st.add_vertex(Vector3(0          + i * grid_offs, 0, length))
		st.add_vertex(Vector3(0          + i * grid_offs, 0, length))
		st.add_vertex(Vector3(0          + i * grid_offs, 0, 0))
		st.add_vertex(Vector3(line_width + i * grid_offs, 0, 0))
	
	self.mesh = st.commit()
