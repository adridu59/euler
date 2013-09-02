/*
 * Project Euler: Problem 7.
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

fn returnPrime(rank: int) -> int {
	let mut primeCpt = 0;
	let mut cpt = 2;
	loop {
		if isPrime(cpt) {
			primeCpt += 1;
			if primeCpt == rank {
				return cpt;
			}
		}
		cpt += 1;
	}
}

fn main() {
	printfln!(returnPrime(10001));
}
