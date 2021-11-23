use std::cmp::max;

fn main()
{
	let mut largest_chain: u64 = 0;
	let mut res: u64 = 0;
	for n in 1..1000000 as u64
	{
		let new_largest: u64 =  max(get_chain_length(n), largest_chain);
		if new_largest > largest_chain
		{
			largest_chain = new_largest;
			res = n;
		}
	}

	println!("Result: {}", res);
}

fn get_chain_length(num: u64) -> u64
{
	let mut n: u64 = num;
	let mut len: u64 = 0;
	
	while n != 1
	{
		if n % 2 == 0
		{
			n /= 2;
		}
		else
		{
			n = 3 * n + 1;
		}
		len += 1;
	}
	return len;
}