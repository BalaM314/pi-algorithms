use std::time::Instant;


trait IterativePiCalc {
	fn new() -> Self
		where Self: Sized;
	fn update(&mut self);
	fn get_pi(&self) -> f64;
	fn iterations(&self) -> i128;
}

trait DirectPiCalc {
	fn get_pi(depth:i32) -> f64;
}

struct BaselPiCalculator {
	iterations: i128,
	sum: f64,
}

impl IterativePiCalc for BaselPiCalculator {
	fn new() -> Self {
		Self {
			iterations: 0,
			sum: 0.
		}
	}

	fn update(&mut self){
		self.iterations += 1;
		self.sum += 1. / self.iterations.pow(2) as f64;
	}

	fn get_pi(&self) -> f64 {
		(self.sum * 6.).sqrt()
	}

	fn iterations(&self) -> i128 { self.iterations }
}

struct OddRecipPiCalculator {
	sum: f64,
	x: i128,
}

impl IterativePiCalc for OddRecipPiCalculator {
	fn new() -> Self {
		Self {
			x: 1,
			sum: 0.
		}
	}

	fn update(&mut self){ // 1/1 - 1/3 + 1/5 + 1/7
		self.sum += 1. / (self.x as f64);
		self.x = self.x.signum() * -2 - self.x;
	}

	fn get_pi(&self) -> f64 {
		self.sum * 4.
	}

	fn iterations(&self) -> i128 { (self.x.abs() - 1) / 2 }
}


fn main() {
	let calcs: [Box<dyn IterativePiCalc>; 2] = [Box::new(BaselPiCalculator::new()), Box::new(OddRecipPiCalculator::new())];
	for mut calc in calcs {
		let now = Instant::now();
		for _ in 0..10_000_000 {
			calc.update();
		}
		println!("{:3} iterations: {}", calc.iterations(), calc.get_pi());
		println!("{:#?}", now.elapsed());
	}
}
