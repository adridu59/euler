/*
 * Project Euler: Problem 10.
 *
 * Copyright (C) 2013, Adrien Tétar. All Rights Reserved.
 */
use std::num::sqrt;

fn isPrime(number: int) -> bool {
	let mut d = 2;
	loop {
		// Don't stall when dividing the number by itself
		if d == number {
			d += 1;
		}
		if number % d == 0 {
			return false;
		}
		d += 1;
		if d as float > sqrt(number as float) {
			return true;
		}
	}
}

fn main() {
	let mut res = 0;
	for i in range(2, 2000000) {
		if isPrime(i) {
			res += i;
		}
	}
	printfln!(res);
}
