#![feature(test)]
extern crate rand;
extern crate test;
extern crate quicksort;
extern crate rust_quicksort;

#[cfg(test)]
mod benches {
    use rand::{Rng, SeedableRng, StdRng};
    use test;
    use quicksort;
    use rust_quicksort;

    #[bench]
    fn bench_rust_quicksort(b: &mut test::Bencher) {
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
		let len: usize = 100000;
	    let mut v: Vec<isize> = rng.gen_iter::<isize>().take(len).collect();

        b.iter(|| rust_quicksort::quicksort(&mut v))
    }

    #[bench]
    fn bench_quicksort(b: &mut test::Bencher) {
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
		let len: usize = 100000;
	    let mut v: Vec<isize> = rng.gen_iter::<isize>().take(len).collect();

        b.iter(|| quicksort::quicksort(&mut v))
    }
}