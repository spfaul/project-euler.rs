fn main()
{
	let mut nth_num: u64 = 5;
	loop
	{
		println!("Trying... {}", nth_num);
		let n: u64 = generate_triangle_num(nth_num);
		let mut factor_count: u64 = 0;

		for possible_factor in 1..n as u64
		{
			if possible_factor * possible_factor > n
			{
				break;
			}
		
			if n % possible_factor == 0
			{
				factor_count += 2;
			}
		}

		println!("{}, {}", factor_count, n);

		if factor_count <= 500
		{
			nth_num += 1;
			continue
		}

		println!("FINAL RESULT: {}, {}, {}", nth_num, n, factor_count);
		break;
	}
}

fn generate_triangle_num(nth: u64) -> u64
{
	(1..nth+1).sum()
}