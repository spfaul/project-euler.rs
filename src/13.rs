use std::fs;

fn main()
{
	// grab input
	let contents: String = fs::read_to_string("input/13.txt")
		.expect("Something went wrong with the file!");

	// instead of storing the whole 50 digits, just store
	// first 15ish digits, because anything after that is 
	// too small to affect the first 10 digits of the total 
	let mut v: Vec<u64> = Vec::new();
	for line in contents.split("\n")
	{
		let string = &line[0..=15];
		v.push(string.parse().unwrap());
	}

	let total: u64 = v.iter().sum();
	println!("Result: {}", &total.to_string()[0..=9]); 
}