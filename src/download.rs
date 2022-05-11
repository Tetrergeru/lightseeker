use std::{cell::RefCell, collections::HashMap, rc::Rc};

use futures::{
    channel::oneshot,
    future::{join, join_all},
};
use gloo::net::http::Request;
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlImageElement, Url, WebGl2RenderingContext as Gl};

use crate::objects::texture::Texture;

#[derive(Clone)]
pub struct ResourceManager(Rc<RefCell<ResourceBatch>>);

impl ResourceManager {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(ResourceBatch::new())))
    }

    pub async fn download(requests: Vec<ResourceRequest>, gl: Gl) -> ResourceBatch {
        let mut textures = vec![];
        let mut texts = vec![];
        for req in requests.iter() {
            match req {
                ResourceRequest::Image(req) => {
                    textures.push(async {
                        (
                            req.name.clone(),
                            Rc::new(Texture::new(download_image(&req.path).await, &gl)),
                        )
                    });
                }
                ResourceRequest::Text(req) => {
                    texts.push(async {
                        (req.name.clone(), Rc::new(download_text(&req.path).await))
                    });
                }
            };
        }

        let (textures, texts) = join(join_all(textures), join_all(texts)).await;

        ResourceBatch {
            textures: textures.into_iter().collect(),
            texts: texts.into_iter().collect(),
        }
    }

    pub fn merge(&mut self, other: ResourceBatch) {
        let mut inner = self.0.borrow_mut();
        for (name, texture) in other.textures {
            inner.textures.insert(name, texture);
        }
        for (name, text) in other.texts {
            inner.texts.insert(name, text);
        }
    }

    pub fn get_texture(&self, name: &str) -> Rc<Texture> {
        self.0.borrow().textures[name].clone()
    }

    pub fn get_text(&self, name: &str) -> Rc<String> {
        self.0.borrow().texts[name].clone()
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ResourceBatch {
    textures: HashMap<String, Rc<Texture>>,
    texts: HashMap<String, Rc<String>>,
}

impl ResourceBatch {
    fn new() -> Self {
        Self {
            textures: HashMap::new(),
            texts: HashMap::new(),
        }
    }
}

pub enum ResourceRequest {
    Image(RequestedEntity),
    Text(RequestedEntity),
}

impl ResourceRequest {
    pub fn image(path: &str, name: &str) -> Self {
        Self::Image(RequestedEntity::new(path, name))
    }

    pub fn text(path: &str, name: &str) -> Self {
        Self::Text(RequestedEntity::new(path, name))
    }
}

pub struct RequestedEntity {
    pub path: String,
    pub name: String,
}

impl RequestedEntity {
    fn new(path: &str, name: &str) -> Self {
        Self {
            path: path.to_string(),
            name: name.to_string(),
        }
    }
}

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
