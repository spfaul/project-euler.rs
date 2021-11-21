fn main()
{
	let mut total: u64 = 2;
	fib(1, 2, &mut total);
	println!("{}", total);
}

fn fib(a: u64, b: u64, total: &mut u64)
{
	let x = a + b;
	if x % 2 == 0
	{
		*total += x;
	}

	if x < 4 * 10_u64.pow(6)
	{
		fib(b, x, total);
	}
}
