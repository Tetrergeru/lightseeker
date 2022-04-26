use gloo::net::http::Request;
use wasm_bindgen_futures::JsFuture;

pub async fn download_text(path: &str) -> String {
    let resp = Request::get(path).send().await.unwrap();
    let text: String = JsFuture::from(resp.as_raw().text().unwrap())
        .await
        .unwrap()
        .as_string()
        .unwrap();
    text
}
