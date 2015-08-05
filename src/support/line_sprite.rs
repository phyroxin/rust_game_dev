#[derive(Copy, Clone)]

pub struct Vertex {
	position : [f32; 2]
}

pub struct Line {
	pub line_vertex1 : Vertex,
	pub line_vertex2 : Vertex,
}

impl Line {
	pub fn new() -> Line {
	
		implement_vertex!(Vertex, position);
		
		// square
		Line {
			line_vertex1 : Vertex { position: [-1.0, -0.05] }, // left side 
			line_vertex2 : Vertex { position: [ 1.0, -0.05] }, // right side
			
		}
	}
}