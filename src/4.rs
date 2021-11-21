use std::cmp;

fn main()
{

	let mut maximum = 0;
	for i in 100..1000
	{
		for j in 100..1000
		{
			if is_palindrome((i*j).to_string())
			{
				maximum = std::cmp::max(i*j, maximum);
			}
		}
	}

	println!("{}", maximum);
}


fn is_palindrome(s: String) -> bool
{
	if s.len() == 1
	{
		return true;
	}

	let mut front: u8 = 0;
	let mut end: u8 = (s.len() as u8) - 1;

	while front < end
	{
		if s.chars().nth(front as usize).unwrap() != s.chars().nth(end as usize).unwrap()
		{
			return false;
		}
		front += 1;
		end -= 1;
	}

	return true;
}
