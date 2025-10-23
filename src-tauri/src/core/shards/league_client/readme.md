由于 Rust 没有 MobX 这样的响应式库，所以需要自己实现响应式设计，在这里我们可以尝试将 属性字段都隐藏掉，同时使用 get set 方法来获取字段。并使用 channels 来处理后端到前端的响应

其响应式设计主要是为了在 数据变化时，自动触发 UI 的更新等等事件，

这是一个后端向前端数据更新的过程，在这里 tauri 可以使用 event system，channels， evaluating js 三种方法来实现：https://tauri.app/develop/calling-frontend/#channels

这里推荐使用 channels 方法，其性能最好。

后续 lcu_state 结构体中需要使用 `Arc<Rwlock>` 来封装或者 `Arc<Mutex>` 来封装

```rust
后续可能实现
// 事件类型
#[derive(Clone, Serialize)]
pub enum GameDataEvent {
    SummonerSpellsUpdated { data: HashMap<i32, SummonerSpell> },
    ItemsUpdated { data: HashMap<i32, Item> },
    QueuesUpdated { data: HashMap<i32, Queue> },
    PerksUpdated { data: HashMap<i32, Perk> },
    PerkStylesUpdated { data: PerkStylesData },
    AugmentsUpdated { data: HashMap<i32, Augment> },
    ChampionsUpdated { data: HashMap<i32, ChampionSimple> },
}


// 辅助方法：发射事件
fn emit_event(&self, app_handle: &tauri::AppHandle, event: GameDataEvent) {
    let _ = app_handle.emit_all("game-data-updated", event);
}
```