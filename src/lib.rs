pub mod p1;
pub mod p10;
pub mod p11;
pub mod p2;
pub mod p3;
pub mod p4;
pub mod p5;
pub mod p6;
pub mod p7;
pub mod p8;
pub mod p9;

pub fn simple_bench<O>(input: &str, function: fn(&str) -> O) {
    benchmarking::warm_up();

    let bench_result = benchmarking::bench_function(|measurer| {
        measurer.measure(|| function(input));
    })
    .unwrap();
    assert!(bench_result.times() > 0);
    eprintln!("Average duration: {:?}", bench_result.elapsed());
}
