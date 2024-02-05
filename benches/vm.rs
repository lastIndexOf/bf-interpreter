// use brainfuck_interpreter::{
//     lexer::Lexer,
//     vm::{VirtualMachine, VM},
// };

use brainfuck_interpreter::{
    lexer::Lexer,
    vm::{VirtualMachine, VM},
};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("vm_group");
    group.sample_size(10);
    group.bench_with_input(BenchmarkId::new("vm", 10), &10, |b, _| {
        b.iter(|| {
            let filename = "./bf/mandelbrot.bf";

            let lexer = Lexer::from(std::fs::File::open(filename).unwrap());
            let mut vm = VirtualMachine::new();

            vm.run(lexer);
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
