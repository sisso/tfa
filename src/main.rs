use rouille::Request;
use rouille::Response;
use serde::Serialize;
use std::cell::RefCell;
use std::io::Read;
use std::sync::Mutex;
use structopt::StructOpt;

#[derive(StructOpt)]
struct CommandArgs {
    #[structopt(default_value = "8881", long)]
    port: i32,
}

#[derive(Serialize)]
struct KeyResp {
    value: Option<Value>,
}

type Key = String;
type Value = String;

struct State {
    requests: Vec<(Key, Option<Value>)>,
}

fn main() {
    let args = CommandArgs::from_args();

    env_logger::builder()
        .filter(None, log::LevelFilter::Debug)
        .init();

    let state = Mutex::new(State { requests: vec![] });

    rouille::start_server(format!("0.0.0.0:{}", args.port), move |request| {
        let url = request.url();

        if url.starts_with("/keys/") {
            log::debug!("{:?}", request);

            let key = &url["/keys/".len()..];
            log::info!("key {}", key);

            let mut state = state.lock().unwrap();
            let index = state.requests.iter().position(|(k, v)| k == key);
            let data = request.data();

            match request.method() {
                "GET" if index.is_none() => {
                    state.requests.push((key.to_string(), None));
                    return Response::json(&KeyResp { value: None });
                }

                "GET" => {
                    return Response::json(&KeyResp {
                        value: state.requests[index.unwrap()].1.take(),
                    });
                }
                "POST" if index.is_none() => {
                    return Response::empty_404();
                }

                "POST" if data.is_none() => {
                    return Response::empty_400();
                }

                "POST" => {
                    let mut rb = data.unwrap();
                    let mut value = String::new();
                    rb.read_to_string(&mut value)
                        .expect("fail to parse request body as string");

                    state.requests[index.unwrap()].1 = Some(value);
                    return Response::empty_204();
                }
                _ => {}
            }
        }

        return Response::empty_404();
    });

    log::info!("done");
}
