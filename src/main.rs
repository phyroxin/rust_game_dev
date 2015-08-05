#[macro_use]
extern crate glium;

use glium::Surface;
use support::MoveState;
use support::Square;
use support::Line;
mod support;

fn main() {

    use glium:: DisplayBuild;
	
	// build window
	let display = glium::glutin::WindowBuilder::new()
		.with_dimensions(600, 600)
		.with_title(format!("Bouncy Box of Justice"))
		.build_glium()
		.unwrap();
	
	// square
	let square_object = Square::new();
	let square = vec![square_object.square_vertex1, square_object.square_vertex2, square_object.square_vertex3, square_object.square_vertex4, square_object.square_vertex5];
	
	let vertex_buffer = glium::VertexBuffer::new(&display, &square).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
	
	// line
	let line_object = Line::new();
	let line = vec![line_object.line_vertex1, line_object.line_vertex2];
	
	let vertex_buffer2 = glium::VertexBuffer::new(&display, &line).unwrap();
	let indices2 = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);
	
	// shader script
	let vertex_shader_src = r#"
		#version 140
		
		in vec2 position;
		out vec2 my_attr; // new attribute
		
		uniform mat4 matrix;
		
		void main() {
			my_attr = position; // need to set the value of each 'out' variable.
			gl_Position = matrix * vec4(position, 0.0, 1.0);
		}
	"#;
	
	let vertex_shader_src2 = r#"
		#version 140
		
		in vec2 position;
		
		void main() {
			gl_Position = vec4(position, 0.0, 1.0);
		}
	"#;
	
	let fragment_shader_src = r#"
		#version 140
		
		in vec2 my_attr;
		out vec4 color;
		
		void main() {
			color = vec4(0.3, 0.3, 0.3, 1.0); // build a vec4 from a vec2 and two floats
		}
	"#;
	
	let fragment_shader_src2 = r#"
		#version 140

		out vec4 color;
		
		void main() {
			color = vec4(0.3, 0.3, 0.3, 1.0); // build a vec4 from a vec2 and two floats
		}
	"#;
	
	let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
	let program2 = glium::Program::from_source(&display, vertex_shader_src2, fragment_shader_src2, None).unwrap();

	let mut move_state = MoveState::new();

	let mut uniforms;
	
	loop {
	
		// TODO: add bounce mechanic
		
		if move_state.moving_up {
			
			move_state.width_arc = 0.1;
			move_state.jump_arc  -= move_state.gravity;
			move_state.j 		 += move_state.jump_arc;
			move_state.w 		 -= move_state.jump_arc * 0.7;
			move_state.h 		 += move_state.jump_arc / 0.9;
			
			if move_state.j < 0.05 {
				move_state.moving_up = false;
				move_state.jump_arc  = 0.1;
				move_state.j 		 = 0.0;
				move_state.w 		 = 1.1;
				move_state.h 		 = 1.0;
			}
		}
		else {
			if move_state.w > 1.0 {
				move_state.width_arc -= move_state.gravity + 0.5;
				move_state.w		 += move_state.width_arc * 0.005;
			}
			else {
				move_state.width_arc = 0.1;
				move_state.w		 = 1.0;
			}
		}
		
		if move_state.moving_down {
			println!("Down key pressed: {}", "S");
		}
		
		if move_state.moving_left {
			
			if move_state.speed < move_state.maxspeed {
				move_state.speed += move_state.acceleration;
				move_state.h	 -= move_state.speed * 0.6;
				move_state.j     -= (move_state.speed * 0.7) / 17.0;
			}
			
			move_state.t -= move_state.speed;
						
			if move_state.t < -1.05 {
				move_state.t = 1.05;
			}
		}
		else if move_state.moving_right {
		
			if move_state.speed < move_state.maxspeed {
				move_state.speed += move_state.acceleration;
				move_state.h	 -= move_state.speed * 0.2;
				move_state.j     -= (move_state.speed * 0.3) / 17.0;
			}
			
			move_state.t += move_state.speed;
			
			if move_state.t > 1.05 {
				move_state.t = -1.05;
			}
		}
		else {
			
			if move_state.speed > move_state.minspeed {
				
				move_state.speed -= move_state.acceleration;
				move_state.h	 += move_state.speed * 0.2;
				move_state.w	 -= move_state.speed * 0.3;
				move_state.j     += (move_state.speed * 0.3) / 17.0;
				
				if move_state.moving == 1 {
					move_state.t -= move_state.speed;
				}
				else if move_state.moving == 2 {
					move_state.t += move_state.speed;
				}
			}
			else {
				move_state.speed		= 0.001; 
				move_state.acceleration = 0.002;
				move_state.moving		= 0;
			}
			
		}
		
		uniforms = uniform! {
			matrix: [
				[move_state.w, 0.0, 0.0, 0.0],
				[0.0, move_state.h, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[move_state.t, move_state.j, 0.0, 1.0],
			]
		};

		// drawing
		let mut target = display.draw();	
		target.clear_color(0.6,0.6,0.6,1.0);	
		
		target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
		target.draw(&vertex_buffer2, &indices2, &program2, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
		
		target.finish().unwrap();
		
		// listing the events produced by the window and waiting to be received
		for ev in display.poll_events() {
			match ev {
				glium::glutin::Event::Closed => return, // the window has been closed by the user
				
				glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::A)) => {
					move_state.moving_left	= true;
					move_state.moving		= 1;
				},
				
				glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released, _, Some(glium::glutin::VirtualKeyCode::A)) => {
					move_state.moving_left	= false;
				},
				
				glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::D)) => {
					move_state.moving_right = true;
					move_state.moving		= 2;
				},
				
				glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released, _, Some(glium::glutin::VirtualKeyCode::D)) => {
					move_state.moving_right = false;
				},
				
				glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::W)) => {
					move_state.moving_up	= true;
				},
				
				glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released, _, Some(glium::glutin::VirtualKeyCode::W)) => {
					//move_state.moving_up	= false;
				},
				
				glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::S)) => {
					move_state.moving_down	= true;
				},
				
				glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released, _, Some(glium::glutin::VirtualKeyCode::S)) => {
					move_state.moving_down	= false;
				},
				_ => ()
			}
		}
	}
}
