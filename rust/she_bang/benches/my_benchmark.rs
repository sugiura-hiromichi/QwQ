#![feature(slice_as_chunks)]
#![feature(portable_simd)]
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use std::simd::u64x8;

struct RangePl {
	next: u64,
	end:  u64,
}

impl RangePl {
	fn next_batch(&mut self, target: u64, buffer: &mut [u64],) -> usize {
		self.next = self.next.max(target,);
		let start = self.next;
		if start >= self.end {
			return 0;
		}
		//		let range_len = (self.end - self.next) as usize;
		//		let len = range_len.min(buffer.len(),);

		const PROGRESSION: u64x8 = u64x8::from_array([0, 1, 2, 3, 4, 5, 6, 7,],);
		const LANES: usize = PROGRESSION.lanes();
		const SPEC_LENGTH: usize = LANES * 2;
		if buffer.len() == SPEC_LENGTH && (self.next + SPEC_LENGTH as u64) <= self.end {
			// Specialization for 16 elements
			let low = u64x8::splat(self.next,) + PROGRESSION;
			buffer[..LANES].copy_from_slice(low.as_array(),);

			let high = low + u64x8::splat(LANES as u64,);
			buffer[LANES..].copy_from_slice(high.as_array(),);

			self.next += SPEC_LENGTH as u64;

			SPEC_LENGTH
		} else {
			// General case for slice of any size
			let range_len = (self.end - self.next) as usize;
			let len = range_len.min(buffer.len(),);

			for chunk in buffer[..len].chunks_mut(8,) {
				// This code duplication is required for compiler to vectorize code
				let len = chunk.len();
				for (item, offset,) in chunk.iter_mut().zip(0..len,) {
					*item = self.next + offset as u64;
				}

				self.next += len as u64;
			}

			len
		}
	}
}

fn iter_thous() {
	let mut range = RangePl { next: 0, end: 1000, };
	let mut buffer = [0; 16];
	while range.next_batch(0, &mut buffer,) != 0 {}
}

fn criterion_benchmark(c: &mut Criterion,) {
	c.bench_function("fib 20", |b| b.iter(|| iter_thous(),),);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
