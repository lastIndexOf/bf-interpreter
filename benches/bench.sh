# 获取 sh 的第二个参数
# 用法: ./bench.sh [bench-name]
$type = $1

# cargo bench --features jit,no_output --bench jit_vm
# cargo bench --features ir,no_output --bench ir_vm
# cargo bench --features no_output --bench vm