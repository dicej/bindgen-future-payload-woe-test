use crate::{exports::wasi::http0_3_0_rc_2026_02_09::handler::Guest, wasi::http0_3_0_rc_2026_02_09::types::{ErrorCode, Request, Response}};

wit_bindgen::generate!({
    path: "/home/ivan/github/spin/wit",
    world: "wasi:http/service@0.3.0-rc-2026-02-09",
    generate_all,
});

struct Component;

export!(Component);

impl Guest for Component {
    async fn handle(_request: Request) -> Result<Response, ErrorCode> {
        todo!()
    }
}

