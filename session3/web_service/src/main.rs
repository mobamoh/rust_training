use axum::{
    response::Html,
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(say_hello))
        .route("/mem", get(hello_mem_html))
        .route("/fil", get(hello_file_html))
        .route("/json", get(hello_json))
        .route("/getpost", get(hello_getpost_html))
        .route("/post", post(hello_post_html));

    // app.route("/", say_hello);
    // app.route("/hello", hello_html);
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn say_hello() -> Html<&'static str> {
    Html("<h1>Hello, Mo!</h1>")
}

async fn hello_mem_html() -> Html<&'static str> {
    let html_file = include_str!("hello.html");
    Html(html_file)
}

async fn hello_file_html() -> Html<String> {
    let path = std::path::Path::new("src/html/hello.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

#[derive(serde::Serialize)]
struct HelloMessage {
    msg: String,
}

async fn hello_json() -> axum::Json<HelloMessage> {
    let msg = HelloMessage {
        msg: String::from("Hello, from Json"),
    };
    axum::Json(msg)
}

async fn hello_getpost_html() -> Html<String> {
    let path = std::path::Path::new("src/html/post.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

async fn hello_post_html() -> Html<&'static str> {
    Html("<h1>Answer from Post!</h1>")
}
