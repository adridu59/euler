/*
 * Project Euler: Problem 3.
 *
 * Copyright (C) 2013, Adrien Tétar. All Rights Reserved.
 */
use std::num::sqrt;

fn isPrime(number: int) -> bool {
	let mut d = 2;
	loop {
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
	let mut g = 0; // largest prime factor
	let mut d = 2; // divider
	let n_const = 600851475143i;
	let mut n = n_const.clone();
	loop {
		
		if n % d == 0 {
			n = n / d;
			if d > g {
				g = d;
			}
		} else {
			loop {
				d += 1;
				if isPrime(d) == true {
					break;
				}
			}
		}
		if d as float > sqrt(n_const as float) {
			break;
		}
	}
	printfln!(g);
}
