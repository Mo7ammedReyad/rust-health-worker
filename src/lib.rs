use worker::*;

#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let url = req.url()?;
    if url.pathname() == "/health" {
        let body = serde_json::json!({"status":"OK"});
        Response::from_json(&body)
    } else {
        Response::error("Not Found", 404)
    }
}
