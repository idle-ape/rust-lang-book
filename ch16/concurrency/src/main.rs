fn main() {
    // 掩饰如何创建新线程
    threads::print_in_diff_thread();
    // 使用 move 关键字强制闭包获取上下文中值的所有权
    threads::use_move_to_take_the_ownership();

    // 通过 channel 在多线程间传递消息
    message_passing::communicate_between_thread_with_channel();
    // 将 rx 当作一个迭代器来接收消息
    message_passing::recv_msg_with_iterator();
    // 克隆通道的发送端来实现多个发送者
    message_passing::clone_tx_to_send_with_multi_producer();

    // 在单线程中使用Mutex
    shared_state::use_mutex_in_single_thread();
    // 在多线程中使用Mutex，需要通过 Arc<T> 原子引用计数
    shared_state::use_mutex_in_multi_threads();
}
