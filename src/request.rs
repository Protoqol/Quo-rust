use crate::types::QuoPayload;
use core::time::Duration;

use ehttp::spawn_future;
#[cfg(target_family = "wasm")]
use ehttp::Request;
#[cfg(target_family = "wasm")]
use ehttp::{fetch_async, Headers, Mode};

#[cfg(not(target_family = "wasm"))]
use ureq::config::Config;
#[cfg(not(target_family = "wasm"))]
use ureq::Agent;

pub fn make_request(target: String, payload: QuoPayload) -> () {
    #[cfg(target_family = "wasm")]
    {
        let mode = Mode::NoCors;

        let url = target;
        let method = String::from("POST");

        let timeout = Option::from(Duration::new(1, 0));

        let body = serde_json_wasm::to_vec(&payload).unwrap();
        let headers = Headers::new(&[("Content-Type", "application/json")]);

        let request = Request {
            mode,
            url,
            method,
            timeout,
            body,
            headers,
        };

        spawn_future(async move {
            let _ = fetch_async(request).await;
        })
    }

    #[cfg(not(target_family = "wasm"))]
    {
        let config = Config::builder().user_agent("Quo-Rust").build();

        let agent = config.new_agent();

        let _ = match agent.post(quo_server).send_json(body) {
            Ok(_response) => {}
            Err(ureq::Error::StatusCode(code)) => {
                eprintln!("[QUO] HTTP {} - is Quo running?", code)
            }
            Err(e) => {
                eprintln!("[QUO] error \"{}\" - is Quo running?", e.to_string())
            }
        };
    }
}

#[cfg(test)]
mod tests {
    //
}
