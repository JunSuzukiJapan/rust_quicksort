#![feature(test)]
extern crate rand;
extern crate test;
extern crate quicksort;
extern crate myquicksort;

#[cfg(test)]
mod benches {
    use rand::{Rng, SeedableRng, StdRng};
    use test;
    use quicksort;
    use myquicksort;

    #[bench]
    fn bench_my_quicksort(b: &mut test::Bencher) {
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
		let len: usize = 10000; // 20000000;
	    let mut v: Vec<isize> = rng.gen_iter::<isize>().take(len).collect();

        b.iter(|| myquicksort::quicksort(&mut v))
    }

    #[bench]
    fn bench_inplace_quicksort(b: &mut test::Bencher) {
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
		let len: usize = 10000; // 20000000;
	    let mut v: Vec<isize> = rng.gen_iter::<isize>().take(len).collect();

        b.iter(|| quicksort::quicksort(&mut v))
    }
}