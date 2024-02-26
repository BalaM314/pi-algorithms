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

struct BaselPiCalc {
	iterations: i128,
	sum: f64,
}

impl IterativePiCalc for BaselPiCalc {
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

struct LeibnizPiCalc {
	sum: f64,
	x: i128,
}

impl IterativePiCalc for LeibnizPiCalc {
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

struct NilakanthaPiCalc {
	sum: f64,
	i: i128,
}
impl IterativePiCalc for NilakanthaPiCalc {
	fn new() -> Self {
		Self {
			sum: 0.75,
			i: -2,
		}
	}
	fn update(&mut self) { //updates twice
		self.i += 4;
		self.sum += 1. / ((self.i) * (self.i + 1) * (self.i + 2)) as f64 - 1. / ((self.i + 2) * (self.i + 3) * (self.i + 4)) as f64;
	}
	fn get_pi(&self) -> f64 {
		self.sum * 4.
	}
	fn iterations(&self) -> i128 {
		self.i
	}
}


fn main() {
	let calcs: [Box<dyn IterativePiCalc>; 3] = [Box::new(BaselPiCalc::new()), Box::new(LeibnizPiCalc::new()), Box::new(NilakanthaPiCalc::new())];
	for mut calc in calcs {
		let now = Instant::now();
		loop {
			for _ in 0..10_000 {
				calc.update();
			}
			if now.elapsed().as_nanos() > 1_000_000_000 { break; }
		}
		println!("{:3} iterations: {}", calc.iterations(), calc.get_pi());
		println!("{:#?}", now.elapsed());
	}
}
