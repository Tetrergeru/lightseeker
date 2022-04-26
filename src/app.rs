use web_sys::HtmlCanvasElement;
use yew::{html, Component, Context, Html, NodeRef};

use crate::{color::Color, gl_context::GlContext};

pub struct App {
    canvas_ref: NodeRef,
}

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas
                width=1000
                height=1000
                ref={self.canvas_ref.clone()}
            />
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            let context = GlContext::new(canvas, 1000, 1000);
            context.checkerboard(20.0, Color::BLACK, Color::WHITE);
        }
    }
}
