use std::ops::Deref;
use std::sync::LazyLock;
use tokio::runtime::Handle;
use tokio::task::JoinHandle;
use ulid::Ulid;

static DASH_MAP: LazyLock<dashmap::DashMap<String, String>> = LazyLock::new(|| dashmap::DashMap::new());

pub static RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| tokio::runtime::Builder::new_multi_thread().build().unwrap());

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .max_blocking_threads(10)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(root());

}

async fn root(){
    // sync_func_1();
    //
    // sync_func_2();
    sync_func_3();
    // sync_func_4();
}

// no_return
fn sync_func_1(){

    // 使用 spawn_blocking 来启动一个阻塞任务
    let j = tokio::task::spawn_blocking(|| async_func());

    // println!("result:{:?}", result)
}


fn sync_func_2(){
    // 使用 spawn_blocking 来启动一个阻塞任务
    let handler = tokio::task::spawn(async_func());
}



fn sync_func_3(){
    // 使用 block_in_place 来等待异步任务完成
    let r = tokio::task::block_in_place(|| {
        let r2 = Handle::current().block_on(async_func());
        return r2;
    });
    println!("result:{:?}", r)
}

fn sync_func_4(){
    let ulid = futures::executor::block_on(async move {
        RUNTIME.deref().spawn(async move {
            async_func().await
        }).await.unwrap()
    });

    println!("ulid:{:?}", ulid)
}



async fn async_func() -> String{
    println!("before:{:?}", DASH_MAP);
    DASH_MAP.insert("key".to_string(), "value".to_string());

    println!("mid:{:?}", DASH_MAP);

    DASH_MAP.remove("key");
    println!("after:{:?}", DASH_MAP);

    return Ulid::new().to_string();
}