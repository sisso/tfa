use rouille::Request;
use rouille::Response;
use serde::Serialize;

#[derive(Serialize)]
struct KeyResp {
    key: Option<String>,
}

fn main() {
    env_logger::builder()
        .filter(None, log::LevelFilter::Debug)
        .init();

    rouille::start_server("0.0.0.0:8881", move |request| {
        if request.url().starts_with("/keys/") {
            log::debug!("{:?}", request);

            match request.method() {
                "GET" => return Response::json(&KeyResp { key: None }),
                "POST" => return Response::empty_204(),
                _ => {}
            }
        }

        return Response::empty_404();
    });

    log::info!("done");
}
