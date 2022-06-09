use rouille::Response;

use std::io::Read;
use std::sync::Mutex;
use structopt::StructOpt;

#[derive(StructOpt)]
struct CommandArgs {
    #[structopt(default_value = "8881", long)]
    port: i32,
}

type Key = String;
type Value = String;

struct State {
    requests: Vec<(Key, Option<Value>)>,
}

fn main() {
    let args = CommandArgs::from_args();

    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    run_server(args.port);
}

fn run_server(port: i32) {
    let state = Mutex::new(State { requests: vec![] });

    rouille::start_server(format!("0.0.0.0:{}", port), move |request| {
        let url = request.url();

        if !url.starts_with("/keys/") {
            log::info!("GET {} - invalid url", url);
            return Response::empty_404();
        }

        log::debug!("{:?}", request);

        let key = &url["/keys/".len()..];

        let mut state = state.lock().unwrap();
        let index = state.requests.iter().position(|(k, _)| k == key);
        let data = request.data();

        match request.method() {
            "GET" if index.is_none() => {
                log::info!("GET {} - empty ", url);
                state.requests.push((key.to_string(), None));
                return Response::text("");
            }

            "GET" => match state.requests[index.unwrap()].1.take() {
                Some(value) => {
                    log::info!("GET {} - value", url);
                    return Response::text(value);
                }
                None => {
                    log::info!("GET {} - empty", url);
                    return Response::text("");
                }
            },
            "POST" if index.is_none() => {
                log::info!("POST {} - key request not found", url);
                return Response::empty_404();
            }

            "POST" if data.is_none() => {
                log::info!("POST {} - request has no request body", url);
                return Response::empty_400();
            }

            "POST" => {
                log::info!("POST {} - providing value", url);

                let mut rb = data.unwrap();
                let mut value = String::new();
                rb.read_to_string(&mut value)
                    .expect("fail to parse request body as string");

                state.requests[index.unwrap()].1 = Some(value);
                return Response::empty_204();
            }
            method => {
                log::info!("{} {} - invalid method", method, url);
                return Response::empty_404();
            }
        }
    });
}
