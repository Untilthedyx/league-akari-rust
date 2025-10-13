## API

rust 中 LazyLock 相当于 OnceLock<Mutex>，是专门为延迟初始化设计的，在初始化完成后，不需要每次都访问锁

### summoner.rs

这里主要是获取召唤师信息，即 根据名字获取，根据 puuid 获取，还有获取当前召唤师信息