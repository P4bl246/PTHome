use std::time::Instant;
use std::hint::black_box;
use bitdata::bitdata::DataB;
use bitvec::vec::BitVec;

fn main() {
    let runs = 5usize;

    println!("=== Benchmark: DataB vs BitVec ===\n");

    // Test 1: 1-bit pushes (BitVec's strongest case)
    println!("TEST 1: Push 1-bit (BitVec's native case)");
    bench_1bit(1_000, runs);
    bench_1bit(10_000, runs);

    // Test 2: u8 pushes (8-bit typed)
    println!("\nTEST 2: Push u8 (8-bit native type)");
    bench_u8(1_000, runs);
    bench_u8(10_000, runs);

    // Test 3: u16 pushes (16-bit typed)
    println!("\nTEST 3: Push u16 (16-bit native type)");
    bench_u16(1_000, runs);
    bench_u16(10_000, runs);

    // Test 4: u32 pushes (32-bit typed)
    println!("\nTEST 4: Push u32 (32-bit native type)");
    bench_u32(1_000, runs);
    bench_u32(10_000, runs);

    // Test 5: u64 pushes (64-bit typed)
    println!("\nTEST 5: Push u64 (64-bit native type)");
    bench_u64(1_000, runs);
    bench_u64(10_000, runs);

    // Test 6: 24-bit custom size
    println!("\nTEST 6: Push 24-bit custom size");
    bench_24bit(1_000, runs);
    bench_24bit(10_000, runs);

    // Test 7: Mixed sizes (simulating real-world usage)
    println!("\nTEST 7: Push mixed sizes (12, 16, 24, 32 bits)");
    bench_mixed(1_000, runs);
    bench_mixed(10_000, runs);
}

fn bench_1bit(n: usize, runs: usize) {
    let mut times_db = Vec::new();
    let mut times_bv = Vec::new();

    for _ in 0..runs {
        let start = Instant::now();
        let mut db = DataB::new((n / 8) + 8, true).expect("alloc");
        for i in 0..n {
            let _ = db.push(i as u64, black_box(1u8), Some(1), None);
        }
        black_box(db.ptr());
        db.deallocate();
        times_db.push(start.elapsed().as_nanos());

        let start = Instant::now();
        let mut bv: BitVec = BitVec::with_capacity(n);
        for _ in 0..n {
            bv.push(black_box(true));
        }
        black_box(&bv);
        times_bv.push(start.elapsed().as_nanos());
    }

    let avg_db = times_db.iter().sum::<u128>() / times_db.len() as u128;
    let avg_bv = times_bv.iter().sum::<u128>() / times_bv.len() as u128;
    let ratio = avg_bv as f64 / avg_db as f64;
    println!("  n={}: DataB={} ns, BitVec={} ns | BitVec {:.2}x", n, avg_db, avg_bv, ratio);
}

fn bench_u8(n: usize, runs: usize) {
    let mut times_db = Vec::new();
    let mut times_bv = Vec::new();

    for _ in 0..runs {
        let start = Instant::now();
        let mut db = DataB::new(n + 8, true).expect("alloc");
        for i in 0..n {
            // Push the whole u8 at once with native size
            let _ = db.push(i as u64, black_box((i & 0xFF) as u8), None, None);
        }
        black_box(db.ptr());
        db.deallocate();
        times_db.push(start.elapsed().as_nanos());

        let start = Instant::now();
        let mut bv: BitVec = BitVec::with_capacity(n * 8);
        for i in 0..n {
            let byte = (i & 0xFF) as u8;
            // Push 8 bits directly
            bv.push((byte & 0x01) != 0);
            bv.push((byte & 0x02) != 0);
            bv.push((byte & 0x04) != 0);
            bv.push((byte & 0x08) != 0);
            bv.push((byte & 0x10) != 0);
            bv.push((byte & 0x20) != 0);
            bv.push((byte & 0x40) != 0);
            bv.push((byte & 0x80) != 0);
        }
        black_box(&bv);
        times_bv.push(start.elapsed().as_nanos());
    }

    let avg_db = times_db.iter().sum::<u128>() / times_db.len() as u128;
    let avg_bv = times_bv.iter().sum::<u128>() / times_bv.len() as u128;
    let ratio = avg_bv as f64 / avg_db as f64;
    println!("  n={}: DataB={} ns, BitVec={} ns | BitVec {:.2}x", n, avg_db, avg_bv, ratio);
}

fn bench_u16(n: usize, runs: usize) {
    let mut times_db = Vec::new();
    let mut times_bv = Vec::new();

    for _ in 0..runs {
        let start = Instant::now();
        let mut db = DataB::new(n * 2 + 8, true).expect("alloc");
        for i in 0..n {
            let _ = db.push(i as u64, black_box((i & 0xFFFF) as u16), Some(16), None);
        }
        black_box(db.ptr());
        db.deallocate();
        times_db.push(start.elapsed().as_nanos());

        let start = Instant::now();
        let mut bv: BitVec = BitVec::with_capacity(n * 16);
        for i in 0..n {
            let val = (i & 0xFFFF) as u16;
            for b in 0u32..16 {
                bv.push(((val >> b) & 1) == 1);
            }
        }
        black_box(&bv);
        times_bv.push(start.elapsed().as_nanos());
    }

    let avg_db = times_db.iter().sum::<u128>() / times_db.len() as u128;
    let avg_bv = times_bv.iter().sum::<u128>() / times_bv.len() as u128;
    let ratio = avg_bv as f64 / avg_db as f64;
    println!("  n={}: DataB={} ns, BitVec={} ns | BitVec {:.2}x", n, avg_db, avg_bv, ratio);
}

fn bench_u32(n: usize, runs: usize) {
    let mut times_db = Vec::new();
    let mut times_bv = Vec::new();

    for _ in 0..runs {
        let start = Instant::now();
        let mut db = DataB::new(n * 4 + 8, true).expect("alloc");
        for i in 0..n {
            let _ = db.push(i as u64, black_box(i as u32), Some(32), None);
        }
        black_box(db.ptr());
        db.deallocate();
        times_db.push(start.elapsed().as_nanos());

        let start = Instant::now();
        let mut bv: BitVec = BitVec::with_capacity(n * 32);
        for i in 0..n {
            let val = i as u32;
            for b in 0u32..32 {
                bv.push(((val >> b) & 1) == 1);
            }
        }
        black_box(&bv);
        times_bv.push(start.elapsed().as_nanos());
    }

    let avg_db = times_db.iter().sum::<u128>() / times_db.len() as u128;
    let avg_bv = times_bv.iter().sum::<u128>() / times_bv.len() as u128;
    let ratio = avg_bv as f64 / avg_db as f64;
    println!("  n={}: DataB={} ns, BitVec={} ns | BitVec {:.2}x", n, avg_db, avg_bv, ratio);
}

fn bench_u64(n: usize, runs: usize) {
    let mut times_db = Vec::new();
    let mut times_bv = Vec::new();

    for _ in 0..runs {
        let start = Instant::now();
        let mut db = DataB::new(n * 8 + 8, true).expect("alloc");
        for i in 0..n {
            let _ = db.push(i as u64, black_box(i as u64), Some(64), None);
        }
        black_box(db.ptr());
        db.deallocate();
        times_db.push(start.elapsed().as_nanos());

        let start = Instant::now();
        let mut bv: BitVec = BitVec::with_capacity(n * 64);
        for i in 0..n {
            let val = i as u64;
            for b in 0u32..64 {
                bv.push(((val >> b) & 1) == 1);
            }
        }
        black_box(&bv);
        times_bv.push(start.elapsed().as_nanos());
    }

    let avg_db = times_db.iter().sum::<u128>() / times_db.len() as u128;
    let avg_bv = times_bv.iter().sum::<u128>() / times_bv.len() as u128;
    let ratio = avg_bv as f64 / avg_db as f64;
    println!("  n={}: DataB={} ns, BitVec={} ns | BitVec {:.2}x", n, avg_db, avg_bv, ratio);
}

fn bench_24bit(n: usize, runs: usize) {
    let mut times_db = Vec::new();
    let mut times_bv = Vec::new();

    for _ in 0..runs {
        let start = Instant::now();
        let mut db = DataB::new((n * 3) + 8, true).expect("alloc");
        for i in 0..n {
            let val = (i & 0xFFFFFF) as u32;
            let _ = db.push(i as u64, black_box(val), Some(24), None);
        }
        black_box(db.ptr());
        db.deallocate();
        times_db.push(start.elapsed().as_nanos());

        let start = Instant::now();
        let mut bv: BitVec = BitVec::with_capacity(n * 24);
        for i in 0..n {
            let val = (i & 0xFFFFFF) as u32;
            for b in 0u32..24 {
                bv.push(((val >> b) & 1) == 1);
            }
        }
        black_box(&bv);
        times_bv.push(start.elapsed().as_nanos());
    }

    let avg_db = times_db.iter().sum::<u128>() / times_db.len() as u128;
    let avg_bv = times_bv.iter().sum::<u128>() / times_bv.len() as u128;
    let ratio = avg_bv as f64 / avg_db as f64;
    println!("  n={}: DataB={} ns, BitVec={} ns | BitVec {:.2}x", n, avg_db, avg_bv, ratio);
}

fn bench_mixed(n: usize, runs: usize) {
    let mut times_db = Vec::new();
    let mut times_bv = Vec::new();

    for _ in 0..runs {
        let start = Instant::now();
        let mut db = DataB::new(n * 2 + 8, true).expect("alloc");
        for i in 0..n {
            match i % 4 {
                0 => { let _ = db.push(i as u64, black_box((i & 0xFFF) as u16), Some(12), None); },
                1 => { let _ = db.push(i as u64, black_box((i & 0xFFFF) as u16), Some(16), None); },
                2 => { let _ = db.push(i as u64, black_box((i & 0xFFFFFF) as u32), Some(24), None); },
                _ => { let _ = db.push(i as u64, black_box(i as u32), Some(32), None); },
            }
        }
        black_box(db.ptr());
        db.deallocate();
        times_db.push(start.elapsed().as_nanos());

        let start = Instant::now();
        let mut bv: BitVec = BitVec::with_capacity(n * 21); // avg of 12,16,24,32
        for i in 0..n {
            let (val, bits) = match i % 4 {
                0 => ((i & 0xFFF) as u64, 12u32),
                1 => ((i & 0xFFFF) as u64, 16u32),
                2 => ((i & 0xFFFFFF) as u64, 24u32),
                _ => (i as u64, 32u32),
            };
            for b in 0..bits {
                bv.push(((val >> b) & 1) == 1);
            }
        }
        black_box(&bv);
        times_bv.push(start.elapsed().as_nanos());
    }

    let avg_db = times_db.iter().sum::<u128>() / times_db.len() as u128;
    let avg_bv = times_bv.iter().sum::<u128>() / times_bv.len() as u128;
    let ratio = avg_bv as f64 / avg_db as f64;
    println!("  n={}: DataB={} ns, BitVec={} ns | BitVec {:.2}x", n, avg_db, avg_bv, ratio);
}
