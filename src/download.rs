use futures::channel::oneshot;
use gloo::net::http::Request;
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlImageElement, Url};

pub async fn download_text(path: &str) -> String {
    let resp = Request::get(path).send().await.unwrap();
    let text: String = JsFuture::from(resp.as_raw().text().unwrap())
        .await
        .unwrap()
        .as_string()
        .unwrap();
    text
}

pub async fn download_image(path: &str) -> HtmlImageElement {
    let resp = Request::get(path).send().await.unwrap();
    let blob = JsFuture::from(resp.as_raw().blob().unwrap()).await.unwrap();

    let url = Url::create_object_url_with_blob(&blob.unchecked_into()).unwrap();
    let image = HtmlImageElement::new().unwrap();

    let (send, recv) = oneshot::channel();

    let on_load_closure = Closure::once(Box::new(move || {
        send.send(()).unwrap();
    }) as Box<dyn FnOnce()>);
    image.set_onload(Some(on_load_closure.as_ref().unchecked_ref()));
    on_load_closure.forget();

    let cloned_path = path.to_string();
    let on_error_closure = Closure::wrap(Box::new(move || {
        panic!("image {} loading failed", cloned_path);
    }) as Box<dyn FnMut()>);
    image.set_onerror(Some(on_error_closure.as_ref().unchecked_ref()));
    on_error_closure.forget();

    image.set_src(&url);

    recv.await.unwrap();

    image
}
