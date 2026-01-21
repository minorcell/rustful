use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json;
use std::fs as std_fs;
use std::path::Path;
use std::sync::Mutex;

// 定义待办事项的结构体：作为接口返回的数据模型
#[derive(serde::Serialize, serde::Deserialize)]
struct Todo {
    // 待办事项的唯一 ID（服务端生成）
    id: u64,
    // 待办事项标题
    title: String,
    // 是否完成
    completed: bool,
}

// 仅用于“新增”接口的请求体：前端只需要传标题
#[derive(serde::Deserialize)]
struct NewTodo {
    title: String,
}

// Actix 在多线程中共享状态时，通常用 Mutex 包裹
type TodoList = Mutex<Vec<Todo>>;

// 数据文件路径：项目根目录下的 todos.json
const DATA_FILE: &str = "todos.json";

// 从本地文件加载待办事项
// 优先读取标准 JSON：[{ "id": 1, "title": "...", "completed": false }]
// 兼容旧格式：每行一条记录（id,title,completed）
fn load_todos() -> Vec<Todo> {
    // 如果文件不存在，返回空列表
    if Path::new(DATA_FILE).exists() {
        // 读取整个文件内容
        let data = std_fs::read_to_string(DATA_FILE).expect("无法读取数据文件");
        // 先尝试按 JSON 解析
        if let Ok(todos) = serde_json::from_str::<Vec<Todo>>(&data) {
            return todos;
        }

        // JSON 解析失败时，退回到旧的“每行一条记录”格式
        let mut todos: Vec<Todo> = Vec::new();
        for line in data.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3
                && let (Ok(id), Ok(completed)) = (parts[0].parse::<u64>(), parts[2].parse::<bool>())
            {
                // 解析成功就追加到列表里
                todos.push(Todo {
                    id,
                    title: parts[1].to_string(),
                    completed,
                });
            }
        }

        todos
    } else {
        // 文件不存在返回空列表
        vec![]
    }
}

// 将待办事项保存到本地文件
// 使用标准 JSON，方便后续扩展与通用工具读取
fn save_todos(todos: &Vec<Todo>) {
    let data = serde_json::to_string_pretty(todos).expect("无法序列化为 JSON");
    std_fs::write(DATA_FILE, data).expect("无法写入数据文件");
}

// GET /todos
// 获取所有待办事项，返回 JSON 数组
async fn get_todos(data: web::Data<TodoList>) -> impl Responder {
    // Mutex 需要 lock 才能读写
    let todos = data.lock().unwrap();
    HttpResponse::Ok().json(&*todos)
}

// POST /todos
// 添加一个待办事项：根据已有列表生成新 ID
async fn add_todo(todo: web::Json<NewTodo>, data: web::Data<TodoList>) -> impl Responder {
    let mut todos = data.lock().unwrap();
    // 生成下一个 ID：取最大 ID + 1
    let next_id = todos.iter().map(|item| item.id).max().unwrap_or(0) + 1;
    let new_todo = Todo {
        id: next_id,
        // trim 去掉用户输入的首尾空格
        title: todo.title.trim().to_string(),
        completed: false,
    };
    todos.push(new_todo);
    // 及时持久化到文件
    save_todos(&todos);
    HttpResponse::Created().finish()
}

// POST /todos/{id}/toggle
// 切换某个待办事项的完成状态
async fn toggle_todo(path: web::Path<u64>, data: web::Data<TodoList>) -> impl Responder {
    // 从路径参数中取出 id
    let id = path.into_inner();
    let mut todos = data.lock().unwrap();
    if let Some(todo) = todos.iter_mut().find(|item| item.id == id) {
        todo.completed = !todo.completed;
        // 保存更新结果
        save_todos(&todos);
        return HttpResponse::Ok().finish();
    }

    HttpResponse::NotFound().finish()
}

// 程序入口：启动 HTTP 服务
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动时先读文件，恢复历史数据
    let initial_todos = load_todos();
    // web::Data 是 Actix 的共享状态容器
    let todo_list = web::Data::new(TodoList::new(initial_todos));

    HttpServer::new(move || {
        App::new()
            // 把共享数据挂到应用上
            .app_data(todo_list.clone())
            // 定义路由：GET/POST /todos
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(add_todo))
            // 定义路由：POST /todos/{id}/toggle
            .route("/todos/{id}/toggle", web::post().to(toggle_todo))
    })
    // 绑定本地端口
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
