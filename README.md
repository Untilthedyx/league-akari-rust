```cmd
npm run tauri dev
npm run tauri build --release
```

## 智能指针

### 1. LazyLock 惰性锁 和 OnceCell

LazyLock 的初始化是 "被动触发" 的（第一次访问时自动执行），而 OnceCell 可以通过 get_or_init 主动控制初始化时机。

### 2. RwLock 读写锁相较于 Mutex

RwLock 可以同时允许多个读操作，但是只允许一个写操作，写操作会阻塞所有的读操作。Mutex 则是无论读写操作都会阻塞。

## 配置文件

Akari 的配置文件存放在 C:\Users\周瑾瑜\AppData\Roaming\league-akari 目录下

Akari 配置文件位置 检索 `app.getPath('userData')`

## 相关

1. [rank-analysis](https://github.com/wnzzer/rank-analysis)
2. [LeagueAkari](https://github.com/Hanxven/LeagueAkari)
3. [League of Legends LCU and Riot Client API Docs](https://github.com/KebsCS/lcu-and-riotclient-api)
4. [Seraphine](https://github.com/Zzaphkiel/Seraphine)
5. [LCU API](https://www.mingweisamuel.com/lcu-schema/tool/#/)
