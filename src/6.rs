fn main()
{
	let mut sum_of_squares: i32 = 0;
	for n in 1..101 as i32
	{
		sum_of_squares += n.pow(2); 
	}

	let sum_of_nums: i32 = (0..101).fold(0, |a, b| a + b);
	let sum_of_nums_squared: i32 = sum_of_nums.pow(2);
	

	println!("{}", sum_of_nums_squared - sum_of_squares);

}