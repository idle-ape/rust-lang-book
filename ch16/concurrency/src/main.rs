fn main() {
    // 掩饰如何创建新线程
    threads::print_in_diff_thread();
    // 使用 move 关键字强制闭包获取上下文中值的所有权
    threads::use_move_to_take_the_ownership();
}
