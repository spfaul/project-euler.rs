use std::fs;
use std::cmp;

// if anything this is my 13th reason

fn main()
{
	// extract input into vector
	let contents: String = fs::read_to_string("input/11.txt")
		.expect("Something went wrong opening the file!");

	let mut input: Vec<Vec<i32>> = Vec::new();
	for line in contents.split("\n")
	{
		let mut sub_input: Vec<i32> = Vec::new();
		for num in line.split(" ")
		{
			sub_input.push(num.parse().unwrap());
		}
		input.push(sub_input);
	}

	// actually do stuff
	let mut res: u64 = 0;
	
	for (y, line) in input.iter().enumerate()
	{
		for (x, num) in line.iter().enumerate()
		{
			// left
			if x >= 3 {
				res = calc(res, *num, input[y][x-1], input[y][x-2], input[y][x-3]);
			}
			// right
			if x <= line.len()-4 {
				res = calc(res, *num, input[y][x+1], input[y][x+2], input[y][x+3]);
			}

			// up
			if y >= 3 {
				res = calc(res, *num, input[y-1][x], input[y-2][x], input[y-3][x]);
			}

			// down
			if y <= input.len()-4 {
				res = calc(res, *num, input[y+1][x], input[y+2][x], input[y+3][x]);
			}

			// diagnal left up
			if x >= 3 && y >= 3 {
				res = calc(res, *num, input[y-1][x-1], input[y-2][x-2], input[y-3][x-3]);
			}

			// diagnal left down
			if x >= 3 && y <= input.len()-4 {
				res = calc(res, *num, input[y+1][x-1], input[y+2][x-2], input[y+3][x-3]);
			}

			// diagnol right up
			if x <= line.len()-4 && y >= 3 {
				res = calc(res, *num, input[y-1][x+1], input[y-2][x+2], input[y-3][x+3]);
			}

			// diagnal right down
			if x <= line.len()-4 && y <= input.len()-4 {
				res = calc(res, *num, input[y+1][x+1], input[y+2][x+2], input[y+3][x+3]);
			}
			
		}
	}

	println!("Result, {}", res);
}

fn calc(result: u64, a: i32, b: i32, c: i32, d:i32) -> u64
{
	std::cmp::max(result, (a*b*c*d) as u64)
}