#[derive(Copy, Clone)]

pub struct Vertex {
	position : [f32; 2]
}

pub struct Square {
	pub square_vertex1 : Vertex,
	pub square_vertex2 : Vertex,
	pub square_vertex3 : Vertex,
	pub square_vertex4 : Vertex,
	pub square_vertex5 : Vertex
}

impl Square {
	pub fn new() -> Square {
	
		implement_vertex!(Vertex, position);
		
		// square
		Square {
			square_vertex1 : Vertex { position: [-0.05, -0.05] }, // bottom left corner 
			square_vertex2 : Vertex { position: [-0.05,  0.05] }, // top left corner
			square_vertex3 : Vertex { position: [ 0.05,  0.05] }, // top right corner
			square_vertex4 : Vertex { position: [ 0.05, -0.05] }, // bottom right corner
			square_vertex5 : Vertex { position: [-0.05, -0.05] }, // bottom left corner
		}
	}
}