use askama::Template;
use axum::routing::get;
use axum::Router;
use listenfd::ListenFd;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "today.jinja2", ext = "html")]
struct TodayTemplate {
    title: &'static str,
    timer_running: bool,
}

async fn today() -> TodayTemplate {
    TodayTemplate {
        title: "Today",
        timer_running: false,
    }
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(today))
        .nest_service("/assets", ServeDir::new("assets"));

    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        // otherwise fall back to local listening
        None => TcpListener::bind("127.0.0.1:3000").await.unwrap(),
    };

    // run it
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
