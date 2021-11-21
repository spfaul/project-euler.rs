fn main()
{
	let mut n: u32 = 1;
	
	'outer: loop {
		for i in 1..21
		{
			if n % i != 0
			{
				n += 1;
				continue 'outer
			}
		}

		println!("{}", n);
		break;
	}
	
}