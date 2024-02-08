// use brainfuck_interpreter::{
//     lexer::Lexer,
//     vm::{VirtualMachine, VM},
// };

use brainfuck_interpreter::{
    lexer::Lexer,
    vm::{jit::JitVM, VirtualMachine},
};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("jit_vm_group");
    group.sample_size(10);
    group.bench_with_input(BenchmarkId::new("jit_vm", 10), &10, |b, _| {
        b.iter(|| {
            let filename = "./bf/mandelbrot.bf";

            let lexer = Lexer::from(std::fs::File::open(filename).unwrap());
            let mut vm = VirtualMachine::new();

            vm.run_with_jit(lexer).unwrap();
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
