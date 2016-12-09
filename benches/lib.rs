// bench command:
//
// ```
// $ rustup run nightly cargo bench
// ```

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

    static LENGTH: usize = 100000;
    static SEED: [usize; 4] = [1, 2, 3, 4];

    #[bench]
    fn bench_rust_quicksort(b: &mut test::Bencher) {
        let mut rng: StdRng = SeedableRng::from_seed(&SEED as &[_]);
	    let mut v: Vec<isize> = rng.gen_iter::<isize>().take(LENGTH).collect();

        b.iter(|| rust_quicksort::quicksort(&mut v))
    }

    #[bench]
    fn bench_quicksort(b: &mut test::Bencher) {
        let mut rng: StdRng = SeedableRng::from_seed(&SEED as &[_]);
	    let mut v: Vec<isize> = rng.gen_iter::<isize>().take(LENGTH).collect();

        b.iter(|| quicksort::quicksort(&mut v))
    }

    #[bench]
    fn bench_sort(b: &mut test::Bencher) {
        let mut rng: StdRng = SeedableRng::from_seed(&SEED as &[_]);
	    let mut v: Vec<isize> = rng.gen_iter::<isize>().take(LENGTH).collect();

        b.iter(|| v.sort())
    }
}