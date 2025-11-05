## commandinfo.rs

> PowerShell 和 sysinfo 无法获取部分进程的命令行，本质是因为它们共享同一套 “系统层查询接口”，受限于 Windows 的用户态保护机制；而 winapi 通过直接调用内核级 API，绕过了这些中间层限制，因此能获取到更多进程的命令行。

```rs
let mut sys = System::new_all();
sys.refresh_all();

println!("Process commands:");
sys.processes()
    .values()
    .filter(|x| x.name().eq_ignore_ascii_case("LeagueClientUx.exe"))
    .for_each(|x| println!("{:?}, {:?}", x.pid(), x.cmd()));
```

在这里我本来想使用 sysinfo 来获取进程命令行，但是这里 cmd 返回的是空值，只能使用 winapi 来进行操作，操作麻烦一点，但是能获取到命令行。

### 仓库相关

在 LeagueAkari 仓库中，在文件 `src\main\utils` 下使用到了一个 `import { tools } from '@leagueakari/league-akari-addons'`，这里的 `tool` 有一个 `getCommandLine1` 方法可以获取进程命令行，对应函数为 `queryUxCommandLineNative`，与此同时还定义了一个使用 wmic 来获取进程命令行的函数 `queryUxCommandLine`，其具体获取使用方法如下：

```ts
  private _queryUxCommandLine() {
    if (this.settings.useWmic) {
      if (!this._common.state.isAdministrator) {
        return []
      }

      return queryUxCommandLine(LeagueClientUxMain.UX_PROCESS_NAME)
    }

    return queryUxCommandLineNative(LeagueClientUxMain.UX_PROCESS_NAME)
  }
```

可以看到这里是需要管理员权限才可以使用 wmic 来获取命令行的，不是管理员权限就只能使用 `queryUxCommandLineNative` 来获取命令行。这也就意味着我们需要对 `import { tools } from '@leagueakari/league-akari-addons'` 的功能进行复刻，也就是如何在不使用管理员权限的情况下获取到进程命令行。在这里 rust 可以直接调用 winapi 通过内核级 API 来获取进程命令行，而 PowerShell 和 sysinfo 并不能获取到。
