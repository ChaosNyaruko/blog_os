fn file_len() -> Future {
    let inner_future = async_read_file("foo.txt") 
    return StringLen {
        inner_future: string,
    }
}



fn executor () {
    loop <-task_queue{
    match future.poll(…) {
        Poll::Ready(value) => a = value,
        Poll::Pending => { try xxxx}, // do nothing
    }
}

async fn foo() -> u32 {
    0
}

// by compiler
fn foo() -> impl Future<Output = u32> {
    future::ready(0)
}

async fn example1(min_len: usize) -> String {
    let content = async_read_file("foo.txt").await;
    if content.len() < min_len {
        content + &async_read_file("bar.txt").await
    } else {
        content
    }
    // END
    return 
}
async fn example(min_len: usize) -> String {
    let content = async_read_file("foo.txt").await;
    if content.len() < min_len {
        content + &async_read_file("bar.txt").await
    } else {
        content
    }
    // END
    return 
}

fn example(min_len) -> impl Future<Output = String> {
}

fn example(min_len) -> ExampleStateMachine {
    ExampleStateMachine::Start(StartState {
        min_len
    })
}

hlt


// 状态 + 状态转移= 状态机-> Future
// 状态转移实际写在poll 里
// async task -> 被运行时/Executor调度，由它们来进行poll的过程
