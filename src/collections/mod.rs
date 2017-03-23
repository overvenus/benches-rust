use test;

use fnv::FnvHashMap;
use ordermap::OrderMap;

use rand;
use rand::Rng;
use rand::Rand;

fn new_rand_vec<T: Rand>(size: usize) -> Vec<T> {
    let mut rng = rand::thread_rng();
    rng.gen_iter::<T>().take(size).collect::<Vec<T>>()
}

macro_rules! bench_map {
    (qdi $name:ident => $map:expr => $count:expr => $vsize:expr) => {
        #[bench]
        fn $name (b: &mut test::Bencher) {
            let mut map = $map;
            let count = $count;
            let vsize = $vsize;

            let ks = new_rand_vec::<u64>(count);
            let mut vs: Vec<Vec<u8>> = Vec::with_capacity(count);
            for _ in 0..count {
                vs.push(new_rand_vec(vsize));
            }

            for i in 0..count {
                map.insert(ks[i], vs[i].clone());
            }

            let mut backup = Vec::with_capacity(count / 2);
            b.iter(|| {
                // qurey
                for k in ks.iter().take(count / 2) {
                    let _ = map.get(k);
                }

                // delete
                for k in ks.iter().take(count / 2) {
                    let v = map.remove(k).unwrap();
                    backup.push((*k, v));
                }

                // insert
                for (k, v) in backup.drain(..) {
                    map.insert(k, v);
                }
            });
        }
    }
}

bench_map!{qdi bench_fnv_map_little => FnvHashMap::default() => 10 => 100}
bench_map!{qdi bench_fnv_map_large => FnvHashMap::default() => 1000 => 100}
bench_map!{qdi bench_ordermap_little => OrderMap::new() => 10 => 100}
bench_map!{qdi bench_ordermap_large => OrderMap::new() => 1000 => 100}
