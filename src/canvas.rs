use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
extern crate js_sys;

use crate::direction::Direction;
use crate::sprite::Sprite;

pub struct Canvas {
    pub canvas: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,
    scaled_width: u32,
    scaled_height: u32,
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn new(attr_id: &str, width: u32, height: u32) -> Canvas {

        let document = web_sys::window().unwrap().document().unwrap();
        
        let canvas = document.get_element_by_id(attr_id).unwrap();
     
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let scaled_width = canvas.width() / width;
        let scaled_height = canvas.height() / height;

        Canvas {
            canvas,
            ctx,
            scaled_width,
            scaled_height,
            width,
            height,
        }
    }

    pub fn draw_sprite(&self, x: u32, y: u32, sprite: &Sprite, direction: &Option<Direction>, frame: u8){
        assert!(x < self.width);
        assert!(y < self.height);

        let x = x * self.scaled_width;
        let y = y * self.scaled_height;

        let curr_sprite;
        match direction{
            Some(direction) => {
                match direction{
                    Direction::Up => curr_sprite = &sprite.up,
                    Direction::Down => curr_sprite = &sprite.down,
                    Direction::Left => curr_sprite = &sprite.left,
                    Direction::Right => curr_sprite = &sprite.right,
                    Direction::IdleUp => curr_sprite = &sprite.idle_up,
                    Direction::IdleDown => curr_sprite = &sprite.idle_down,
                    Direction::IdleLeft => curr_sprite = &sprite.idle_left,
                    Direction::IdleRight => curr_sprite = &sprite.idle_right,
                }
            },
            None => {curr_sprite = &sprite.idle_down} 
        }

        let mut sx: f64 = 0.0;
        if frame < 15 {
            sx = 32.0;
        }
        self.clear_all();
        self.ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&sprite.sheet, curr_sprite.0+sx, curr_sprite.1, 32.0, 32.0, f64::from(x), f64::from(y), 32.0, 32.0).unwrap();
    }

    pub fn clear_all(&self) {
        //self.ctx.set_fill_style_color("black");
        self.ctx.fill_rect(
            0.0,
            0.0,
            f64::from(self.width * self.scaled_width),
            f64::from(self.height * self.scaled_height),
        )
    }
}