extern crate glenum;
#[macro_use]
extern crate stdweb;

use std::cell::RefCell;
use std::rc::Rc;

use glenum::*;

use stdweb::web::{document, window, Element};

use stdweb::unstable::TryInto;

pub struct WebGL2RenderingContext {
    reference: stdweb::Reference,
}

impl WebGL2RenderingContext {
    pub fn new(canvas: &Element) -> WebGL2RenderingContext {
        let gl = js! { return (@{canvas}).getContext("webgl2"); };

        WebGL2RenderingContext {
            reference: gl.into_reference().unwrap(),
        }
    }

    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        js! {
            (@{&self.reference}).clearColor(@{r},@{g},@{b},@{a});
        }
    }

    pub fn clear(&self, bit: BufferBit) {
        js! {
            (@{&self.reference}).clear(@{bit as i32})
        }
    }
}

pub struct App {
    gl: WebGL2RenderingContext,
    last_time: i64,
}

impl App {
    pub fn new(canvas: &Element) -> App {
        let gl = WebGL2RenderingContext::new(&canvas);

        App {
            gl: gl,
            last_time: now(),
        }
    }

    pub fn init(&self) {
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
    }

    pub fn render(&mut self) {
        let cur_time = now();

        if cur_time - self.last_time > 1000 {
            self.gl
                .clear_color(rand() as f32, rand() as f32, rand() as f32, 1.0);
            self.gl.clear(BufferBit::Color);

            self.last_time = cur_time;
        }
    }
}

fn rand() -> f64 {
    let value = js! {
        return Math.random();
    };

    value.try_into().unwrap()
}

fn now() -> i64 {
    let value = js! {
        return Date.now();
    };

    value.try_into().unwrap()
}

fn main_loop(app: Rc<RefCell<App>>) {
    app.borrow_mut().render();

    window().request_animation_frame(move |_| {
        main_loop(app);
    });
}

fn main() {
    stdweb::initialize();
    let canvas = document().get_element_by_id("canvas").unwrap();
    let app = Rc::new(RefCell::new(App::new(&canvas)));

    app.borrow_mut().init();

    window().request_animation_frame(move |_| {
        main_loop(app);
    });

    stdweb::event_loop();
}
