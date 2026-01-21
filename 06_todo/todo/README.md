# Todo Demo 教学说明

这是一个基于 Actix Web 的最小前后端打通示例：后端提供待办事项 API，前端用原生 HTML/JS 调用接口并展示结果。面向 Rust 新人，重点学习 HTTP 服务、共享状态、文件持久化和前后端交互。

## 快速开始

1. 启动服务：

```bash
cargo run
```

2. 浏览器打开：

```
http://127.0.0.1:8080
```

## 项目结构

- `src/main.rs`：后端代码（接口、数据结构、文件读写）。
- `static/index.html`：前端页面（展示与调用接口）。
- `todos.json`：数据文件（每行一个待办事项）。

## 这些 `use` 是做什么的

代码里有几行常用的 `use`，它们只是把“路径很长的类型/模块名”起个短别名：

```rust
use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs as std_fs;
use std::path::Path;
use std::sync::Mutex;
```

逐行解释：

- `use actix_files as fs;`  
  引入 Actix 的静态文件服务模块，并起别名 `fs`。这样写 `fs::Files::new(...)` 更短。

- `use actix_web::{web, App, HttpResponse, HttpServer, Responder};`  
  引入 Actix Web 的常用类型：
  - `App`：应用配置，负责注册路由和中间件。
  - `HttpServer`：HTTP 服务器本体，负责监听端口。
  - `HttpResponse`：构造响应（比如 200/404）。
  - `Responder`：一个 trait，表示“可以被返回给客户端的类型”。
  - `web`：模块命名空间，提供 `web::Data`、`web::Json`、`web::Path` 等工具。

- `use std::fs as std_fs;`  
  引入标准库文件系统模块，并起别名 `std_fs`，避免和 `actix_files` 混淆。

- `use std::path::Path;`  
  用于判断 `todos.json` 是否存在。

- `use std::sync::Mutex;`  
  多线程共享数据用的互斥锁。Actix Web 默认是多线程工作，因此共享 `Vec<Todo>` 时需要锁保护。

## 接口教学

后端共 3 个接口：

1. `GET /todos`
   - 返回全部待办事项（JSON 数组）。

2. `POST /todos`
   - 请求体：`{ "title": "xxx" }`
   - 服务端会自动生成 `id`，并设置 `completed=false`。

3. `POST /todos/{id}/toggle`
   - 根据 `id` 切换完成状态。

前端调用逻辑：

- 页面加载时 `fetch("/todos")` 拉取列表。
- 点击“添加”按钮时 `POST /todos`。
- 点击某条待办时 `POST /todos/{id}/toggle`。

## 文件存储说明

这个 demo 把数据存在 `todos.json`，并使用标准 JSON 结构：

```json
[
  { "id": 1, "title": "买牛奶", "completed": false },
  { "id": 2, "title": "写 Rust 代码", "completed": true }
]
```

如果发现旧格式（每行一条记录）会自动兼容读取，并在下一次保存时写成标准 JSON。

## 学习建议（循序渐进）

1. 看懂 `Todo` 和 `NewTodo` 两个结构体：知道谁是输出，谁是输入。
2. 跟着 `get_todos` / `add_todo` / `toggle_todo` 三个函数理解接口流程。
3. 理解 `Mutex<Vec<Todo>>`：为什么要加锁，避免并发写冲突。
4. 再看前端 `static/index.html`，理解 `fetch` 如何调用后端。

## 常见问题

Q: 为什么前端不自己生成 `id`？  
A: 服务端统一生成，避免前后端 ID 类型不一致的问题，也更可靠。

Q: 为什么要用 `Mutex`？  
A: Actix Web 默认多线程，多个请求可能同时读写列表，用锁保护更安全。

Q: 为什么不用数据库？  
A: 这是教学 demo，先用文件持久化降低复杂度。

## 可以继续扩展的方向

- 把 `todos.json` 改成标准 JSON 存储。
- 给待办添加“删除”接口。
- 将前端改成更完整的交互（比如输入回车直接提交）。
