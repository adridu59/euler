/*
 * Project Euler: Problem 4.
 *
 * Copyright (C) 2013, Adrien Tétar. All Rights Reserved.
 */
fn isPalindrome(num: int) -> bool {
	let s = num.to_str();
	s == s.rev_iter().collect::<~str>()
}

fn main() {
	let mut n = 0;
	let mut i = 100;
	let mut j = 100;
	while i < 1000 {
		j = 100;
		while j < 1000 {
			if isPalindrome(i*j) == true && i*j > n {
				n = i * j;
			}
			j += 1;
		}
		i += 1;	
	}
	println!(n);
}
