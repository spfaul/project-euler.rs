fn main()
{
	let mut primes_found: u32 = 0;
	let mut n : u32 = 1;

	while (primes_found < 10001)
	{
		n += 1;
		if is_prime(n)
		{
			primes_found += 1;
		}

	}

	println!("{}", n);
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
