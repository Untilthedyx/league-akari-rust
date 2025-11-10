# Tool for League of Legends

## 优势

## 如何构建

1. 安装本地项目依赖

```cmd
npm install @tauri-apps/cli --save-dev
```

2. 打包构建

```cmd
npm run tauri dev
npm run tauri build --release
```

## 主要参考

| Project                                                   | Description                               |
| --------------------------------------------------------- | ----------------------------------------- |
| [LeagueAkari](https://github.com/LeagueAkari/LeagueAkari) | the basic project struct and informations |
| [rank-analysis](https://github.com/wnzzer/rank-analysis)  | the rust things                           |

## 声明

本软件作为基于 Riot 提供的 League Client Update (LCU) API 开发的辅助工具，由于其设计和实施均未采用侵入性技术手段，理论上不会直接干预或修改游戏数据。

然而需明确指出：

1. 未来腾讯可能更新反作弊系统或其他保护服务，可能会对本软件的使用产生兼容性问题。
2. 使用本软件可能带来包括但不限于游戏账户封禁、数据损坏或其他负面后果。

使用本软件的用户需自行承担由此产生的所有风险与后果。开发者对可能的损失不承担任何责任。

## MIT License