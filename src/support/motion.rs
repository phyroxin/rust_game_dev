pub struct MoveState {
	 pub moving	  	  : i8
	,pub moving_up	  : bool
	,pub moving_down  : bool
	,pub moving_left  : bool
	,pub moving_right : bool
	  
	,pub maxspeed 	  : f32
	,pub minspeed 	  : f32
	,pub jump_arc 	  : f32
	,pub width_arc 	  : f32
	,pub t 			  : f32
	,pub j 			  : f32
	,pub w 			  : f32
	,pub h 			  : f32
	,pub speed 		  : f32
	,pub acceleration : f32
	,pub gravity	  : f32
}

impl MoveState {
	pub fn new() -> MoveState {
		MoveState {
			 moving	  	  : 0
			,moving_up	  : false
			,moving_down  : false
			,moving_left  : false
			,moving_right : false
			
			,maxspeed 	  : 0.05
			,minspeed 	  : 0.001
			,speed 		  : 0.001
			,acceleration : 0.003
			,gravity 	  : 0.007
			,jump_arc 	  : 0.1
			,width_arc 	  : 0.2
			,t 			  : 0.
			,j 			  : 0.0
			,w 			  : 1.0
			,h 			  : 1.0
		}
	}
}
