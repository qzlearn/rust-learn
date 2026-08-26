use core::fmt::NumBuffer;

//学习rust 1.98新特性
fn main() {
    // 创建一个足够容纳usize所有可能值的缓冲区
    let mut buf = NumBuffer::<usize>::new();

    // 将整数格式化到缓冲区中，返回&str
    let s = 42usize.format_into(&mut buf);
    assert_eq!(s, "42");

    // 支持负数（有符号类型）
    let mut buf_i32 = NumBuffer::<i32>::new();
    let s_neg = (-12345i32).format_into(&mut buf_i32);
    assert_eq!(s_neg, "-12345");

    // 大数字也不在话下
    let mut buf_u64 = NumBuffer::<u64>::new();
    let s_big = 18446744073709551615u64.format_into(&mut buf_u64);
    assert_eq!(s_big, "18446744073709551615");
}
