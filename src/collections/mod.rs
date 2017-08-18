use test;

use fnv::{FnvHashMap, FnvBuildHasher};
use ordermap::OrderMap;

use rand;
use rand::Rng;
use rand::Rand;

fn new_rand_vec<T: Rand>(size: usize) -> Vec<T> {
    let mut rng = rand::thread_rng();
    rng.gen_iter::<T>().take(size).collect::<Vec<T>>()
}

macro_rules! bench_map {
    (q $name:ident => $map:expr => $count:expr => $vsize:expr) => {
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

            let mut c = 0;
            b.iter(|| {
                // qurey
                let _ = map.get(&ks[c]);
                c += 1;
                c %= count;
            });
        }
    };

    (di $name:ident => $map:expr => $count:expr => $vsize:expr) => {
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

            let mut c = 0;
            b.iter(|| {
                // delete
                let k = &ks[c];
                let v = map.remove(k).unwrap();
                map.insert(*k, v);
                c += 1;
                c %= count;
            });
        }
    };

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
    };
}

// Query, delete and insert
bench_map!{qdi bench_fnv_map_little => FnvHashMap::default() => 10 => 100}
bench_map!{qdi bench_fnv_map_large => FnvHashMap::default() => 1000 => 100}
bench_map!{qdi bench_ordermap_little => OrderMap::new() => 10 => 100}
bench_map!{qdi bench_ordermap_large => OrderMap::new() => 1000 => 100}
bench_map!{qdi bench_fnv_ordermap_little => OrderMap::with_capacity_and_hasher(0, FnvBuildHasher::default()) => 10 => 100}
bench_map!{qdi bench_fnv_ordermap_large => OrderMap::with_capacity_and_hasher(0, FnvBuildHasher::default()) => 1000 => 100}

// Query
bench_map!{q bench_fnv_map_query_16 => FnvHashMap::default() => 16 => 100}
bench_map!{q bench_ordermap_query_16 => OrderMap::new() => 16 => 100}
bench_map!{q bench_fnv_ordermap_query_16 => OrderMap::with_capacity_and_hasher(0, FnvBuildHasher::default()) => 16 => 100}

bench_map!{q bench_fnv_map_query_32 => FnvHashMap::default() => 32 => 100}
bench_map!{q bench_ordermap_query_32 => OrderMap::new() => 32 => 100}
bench_map!{q bench_fnv_ordermap_query_32 => OrderMap::with_capacity_and_hasher(0, FnvBuildHasher::default()) => 32 => 100}

bench_map!{q bench_fnv_map_query_64 => FnvHashMap::default() => 64 => 100}
bench_map!{q bench_ordermap_query_64 => OrderMap::new() => 64 => 100}
bench_map!{q bench_fnv_ordermap_query_64 => OrderMap::with_capacity_and_hasher(0, FnvBuildHasher::default()) => 64 => 100}

bench_map!{q bench_fnv_map_query_128 => FnvHashMap::default() => 128 => 100}
bench_map!{q bench_ordermap_query_128 => OrderMap::new() => 128 => 100}
bench_map!{q bench_fnv_ordermap_query_128 => OrderMap::with_capacity_and_hasher(0, FnvBuildHasher::default()) => 128 => 100}

bench_map!{q bench_fnv_map_query_512 => FnvHashMap::default() => 512 => 100}
bench_map!{q bench_ordermap_query_512 => OrderMap::new() => 512 => 100}
bench_map!{q bench_fnv_ordermap_query_512 => OrderMap::with_capacity_and_hasher(0, FnvBuildHasher::default()) => 512 => 100}

// Delete and insert
bench_map!{di bench_fnv_map_delete_insert_16 => FnvHashMap::default() => 16 => 100}
bench_map!{di bench_ordermap_delete_insert_16 => OrderMap::new() => 16 => 100}
bench_map!{di bench_fnv_ordermap_delete_insert_16 => OrderMap::with_capacity_and_hasher(0, FnvBuildHasher::default()) => 16 => 100}

bench_map!{di bench_fnv_map_delete_insert_32 => FnvHashMap::default() => 32 => 100}
bench_map!{di bench_ordermap_delete_insert_32 => OrderMap::new() => 32 => 100}
bench_map!{di bench_fnv_ordermap_delete_insert_32 => OrderMap::with_capacity_and_hasher(0, FnvBuildHasher::default()) => 32 => 100}

bench_map!{di bench_fnv_map_delete_insert_64 => FnvHashMap::default() => 64 => 100}
bench_map!{di bench_ordermap_delete_insert_64 => OrderMap::new() => 64 => 100}
bench_map!{di bench_fnv_ordermap_delete_insert_64 => OrderMap::with_capacity_and_hasher(0, FnvBuildHasher::default()) => 64 => 100}

bench_map!{di bench_fnv_map_delete_insert_128 => FnvHashMap::default() => 128 => 100}
bench_map!{di bench_ordermap_delete_insert_128 => OrderMap::new() => 128 => 100}
bench_map!{di bench_fnv_ordermap_delete_insert_128 => OrderMap::with_capacity_and_hasher(0, FnvBuildHasher::default()) => 128 => 100}

bench_map!{di bench_fnv_map_delete_insert_512 => FnvHashMap::default() => 512 => 100}
bench_map!{di bench_ordermap_delete_insert_512 => OrderMap::new() => 512 => 100}
bench_map!{di bench_fnv_ordermap_delete_insert_512 => OrderMap::with_capacity_and_hasher(0, FnvBuildHasher::default()) => 512 => 100}
