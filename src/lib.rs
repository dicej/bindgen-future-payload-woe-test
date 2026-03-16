use crate::{
    exports::wasi::http::handler::Guest,
    wasi::http::types::{ErrorCode, Request, Response},
};

wit_bindgen::generate!({
    path: "wit",
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
