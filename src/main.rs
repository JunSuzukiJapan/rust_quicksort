extern crate rand;
extern crate myquicksort;

use rand::{Rng, SeedableRng, StdRng};
use myquicksort::quicksort;

fn main(){
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let len: usize = 10;
    let mut v: Vec<isize> = rng.gen_iter::<isize>().take(len).collect();
    println!("{:?}", v);
    quicksort(&mut v);
    println!("{:?}", v);

    let mut prev = v[0];
    for i in 1..v.len() {
        if !(prev < v[i]) {
            panic!("!({:?} < {:?})", prev, v[i]);
        }
        prev = v[i];
    }    
}