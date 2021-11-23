fn main()
{
	let mut total: u64 = 0; 
	for num in 1..2000000 as u32
	{
		if is_prime(num)
		{
			total += num as u64;
		}
	}

	println!("Result: {}", total);
}

// stolen from https://github.com/groumache/project-euler-rust/blob/master/src/useful_func.rs
pub fn is_prime(n: u32) -> bool {
	if n < 2 {
	    return false;
	}

	let sqrt_n: u32 = (f64::from(n)).sqrt() as u32;
	let divisor = (2..=sqrt_n).find(|x| n % x == 0);

	divisor == None
}
