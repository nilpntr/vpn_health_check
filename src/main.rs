use std::fs::File;
use std::io::Read;
use fateful::{err_prefix, fatal};
use once_cell::sync::Lazy;
use tide::{Request, Response};
use tide::log;

static INTERFACE: Lazy<String> = Lazy::new(|| {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        fatal!(err_prefix!(), "missing arg interface");
    }
    format!("{}", &args[1])
});

#[tokio::main]
async fn main() -> tide::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        fatal!(err_prefix!(), "missing arg interface");
    }

    println!("Running for interface: {}", &args[1]);

    let mut app = tide::new();
    log::start();
    app.with(log::LogMiddleware::new());
    app.at("/").get(index);
    app.listen(format!("{}:{}", std::env::var("HOST").unwrap_or(String::from("127.0.0.1")), std::env::var("PORT").unwrap_or(String::from("54321")))).await?;
    Ok(())
}

async fn index(_: Request<()>) -> tide::Result {
    let up = check_file_exists();
    let mut res = Response::new(if up {200} else {503});
    res.set_content_type("application/json");
    res.set_body(format!(r#"{{"up": {}}}"#, up));
    Ok(res)
}

fn check_file_exists() -> bool {
    let mut file = match File::open(format!("/sys/class/net/{}/carrier", INTERFACE.as_str())) {
        Ok(f) => f,
        Err(_) => return false,
    };
    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        return false
    }
    true
}