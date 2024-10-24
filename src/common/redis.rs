use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use once_cell::sync::OnceCell;
use redis::AsyncCommands;

static REDIS_POOL: OnceCell<Pool<RedisConnectionManager>> = OnceCell::new();

pub async fn init_redis_pool() {
    let manager = RedisConnectionManager::new("redis://default:redis@localhost").unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();
    assert!(REDIS_POOL.set(pool).is_ok());
}

pub fn get_pool() -> Option<&'static Pool<RedisConnectionManager>> {
    REDIS_POOL.get()
}

pub async fn set(key: &str, val: &str) {
    get_pool()
        .unwrap()
        .get()
        .await
        .unwrap()
        .set::<&str, &str, ()>(key, val)
        .await
        .unwrap();
}

pub async fn set_ex(key: &str, val: &str, ex: u64) {
    get_pool()
        .unwrap()
        .get()
        .await
        .unwrap()
        .set_ex::<&str, &str, ()>(key, val, ex)
        .await
        .unwrap();
}

pub async fn get(key: &str) -> String {
    get_pool()
        .unwrap()
        .get()
        .await
        .unwrap()
        .get(key)
        .await
        .unwrap_or("".to_string())
}

#[tokio::test]
async fn redis_test() {
    init_redis_pool().await;
    set_ex("foo", "bbb", 10).await;
    let result: String = get("foo1").await;
    println!("aaa: {}", result)
}
