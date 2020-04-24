use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::html_element::ImageElement;
use stdweb::web::{document, CanvasRenderingContext2d};

pub struct Canvas {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,
    scaled_width: u32,
    scaled_height: u32,
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn new(attr_id: &str, width: u32, height: u32) -> Canvas {
        let canvas: CanvasElement = document()
            .query_selector(attr_id)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

        let scaled_width = canvas.width() / width;
        let scaled_height = canvas.height() / height;

        js! { console.log(@{width}) }
        js! { console.log(@{height}) }
        js! { console.log(@{scaled_width}) }
        js! { console.log(@{scaled_height}) } 

        Canvas {
            canvas,
            ctx,
            scaled_width,
            scaled_height,
            width,
            height,
        }
    }

    pub fn draw(&self, x: u32, y: u32, sprite: ImageElement, frame: u8){
        assert!(x < self.width);
        assert!(y < self.height);

        let x = x * self.scaled_width;
        let y = y * self.scaled_height;

        self.ctx.fill_rect(
            f64::from(x), 
            f64::from(y), 
            f64::from(self.scaled_width), 
            f64::from(self.scaled_height),
        );
        
        let mut sx: f64 = 0.0;
        if frame < 16 {
            sx = 32.0;
        }
        self.ctx.draw_image_s(sprite, sx, 0.0, 32.0, 32.0, f64::from(x), f64::from(y), 64.0, 64.0).unwrap();
    }

    pub fn clear_all(&self) {
        self.ctx.set_fill_style_color("black");
        self.ctx.fill_rect(
            0.0,
            0.0,
            f64::from(self.width * self.scaled_width),
            f64::from(self.height * self.scaled_height),
        )
    }
}