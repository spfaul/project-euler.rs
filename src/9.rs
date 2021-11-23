fn main()
{
	// this is dumb, so dumb.
	for a in 1..1000 as i32
	{
		for b in 1..1000 as i32
		{
			for c in 1..1000 as i32
			{
				if a.pow(2) + b.pow(2) == c.pow(2) && (a < b && b < c) && a + b + c == 1000
				{
					println!("{}, {}, {}", a, b, c);
					println!("Result: {}", a*b*c);
				}
			}
		}
	}
}