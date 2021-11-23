// in a 2x2 grid, there is a 3x3 grid of corners or "nodes"
// hence in a 20x20 grid, there is a 21x21 grid of nodes
const MAX_X: u8 = 21;
const MAX_Y: u8 = 21;

use std::collections::HashMap;

fn main()
{

	println!("{}", get_routes(1,1,&mut HashMap::new()));
}


fn get_routes(x: u8, y: u8, cache: &mut HashMap<String, u64>) -> u64
{
	if x == MAX_X || y == MAX_Y
	{
		return 1;
	}

	let right_path: u64;
	if (*cache).contains_key(&format!("{},{}", x+1, y))
	{
		right_path = *((*cache).get(&format!("{},{}", x+1, y)).unwrap());
	}
	else
	{
		right_path = get_routes(x+1, y, cache);
		(*cache).insert(format!("{},{}", x+1, y), right_path); 
	}

	let down_path: u64;
	if (*cache).contains_key(&format!("{},{}", x, y+1))
	{
		down_path = *((*cache).get(&format!("{},{}", x, y+1)).unwrap());
	}
	else
	{
		down_path = get_routes(x, y+1, cache);
		(*cache).insert(format!("{},{}", x, y+1), down_path);
	}
	
	right_path + down_path
}