use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

fn main() {
    let t1 = std::cmp::Reverse((0, 1));
    let t2 = std::cmp::Reverse((0, 0));
    let t3 = std::cmp::Reverse((1, 0));

    dbg!(t1 < t2);
    dbg!(t1 < t3);

    let combs = (0..5).combinations(3).collect::<Vec<_>>();
    for e in combs {
        dbg!(e);
    }
}
