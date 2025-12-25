# 状态基础

现在，你已经掌握了在 Dioxus 中创建用户界面的方法，接下来该学习如何创建和更新应用的状态了。

管理状态，可以说是构建应用过程中最困难的部分。

本指南将带你逐步了解 Dioxus 中状态管理的核心原理：首先重点讲解理论知识，随后转向实际应用。

## 状态管理的理论

归根结底，"状态管理"指的是以下三个步骤：

- 为 UI 初始化数据
- 处理来自用户的事件
- 更新数据并重新渲染 UI

刚开始时，管理这一流程并不复杂，但随着应用规模扩大、引入更多异步操作以及与外部资源交互时，状态管理会变得越来越棘手。

## 针对经验丰富的 Web 开发者

如果你是一位经验丰富的 Web 开发者，初次接触 Dioxus 时应该很快就能上手。Dioxus 中的状态管理深受 React、Preact、SolidJS 和 Svelte 等项目的启发。

Dioxus 采用基于信号的响应式机制。与 SolidJS 不同，Dioxus 明确区分了值的读取和写入操作。Rust 语言本身没有类似 JavaScript 中 Proxy 的对象代理机制，因此 Dioxus 通过调用 `.read()` 和 `.write()` 来追踪响应式变化。

```rust
let mut count = use_signal(|| 0);

rsx! {
    button {
        onclick: move |_| *count.write() += 1,
        "Increment"
    }
    {count.read().to_string()}
}
```

与 Svelte 不同，Dioxus 不会在编译时对你的状态进行转换处理。同时，Dioxus 的组件会多次运行。你可以把 Dioxus 中的状态管理看作是 React 与 SolidJS 的混合体：自动追踪响应式变化，但组件在其生命周期内可以自由多次运行。

此外，Dioxus 还采用了"常识性"的优化措施，比如自动属性记忆化和自动响应式追踪——这两点都比 React 有了巨大提升。Dioxus 还支持提前返回，可选地配合 Suspended 未来，让你在组件中获得类似 `async/await` 的异步数据加载模型。

## 针对经验丰富的 Rust 开发者

如果你是一位经验丰富的 Rust 开发者，可能会对 Dioxus 中使用的一些"不寻常"的原语感到陌生，比如 `use_signal`、`use_memo`、`use_resource` 等。Dioxus 采用有状态的"钩子"（hooks）模式——这种模式最初源自 Web 开发领域（尤其是 React）。

如果你不想在 Dioxus 中编写类似 React 的代码，也可以选择使用结构体存储状态，并以命令式方式处理 UI 更新。你可以这样定义一个结构体来存储状态：

```rust
struct EditorState {
    text: String,
}

impl EditorState {
    fn handle_input(&mut self, event: FormEvent) {
        self.text = event.value();
    }
}
```

然后，在组件中通过单个 `use_signal` 来使用这个状态：

```rust
/// 定义一个组件
#[component]
fn TextEditor() -> Element {
    // 使用一个单一的"状态"对象，包装在 Signal 中
    let mut state = use_signal(EditorState::new);

    rsx! {
        input {
            // 在事件处理器中，调用 `.write()` 获取"状态"对象的可变引用
            oninput: move |event| state.write().handle_input(event)
        }
    }
}
```

Rust 的借用检查器让跨异步任务和回调持有值变得更加复杂。而我们内置的原语则解决了许多问题：

- 可以在静态任务和回调中引用数据
- 调用 `.write()` 会自动将组件加入队列，触发重新渲染
- 即使在异步上下文中，值也不会"过时"（stale）

Dioxus 暴露了其核心运行时函数，比如 `spawn` 和 `needs_update`——这种做法可能更符合 Rust 开发者的习惯。内置的响应式原语智能地利用了这些函数，因此相比简单的状态管理方案，它们通常更加高效，但也需要一定的学习成本。
