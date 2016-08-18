use rand;
use rand::distributions::{IndependentSample, Range};

pub fn roll(count: i32, sides: i32) -> i32 {
	let mut rand = rand::thread_rng();
	let sides_range = Range::new(1, sides + 1);

	let mut result = 0;
	for _ in 0..count {
		result += sides_range.ind_sample(&mut rand);
	}
	result
}
