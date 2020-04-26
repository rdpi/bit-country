#[derive(Debug, Clone)]
pub struct Sprite{
   pub sheet: web_sys::HtmlImageElement,
   pub up: (f64, f64),
   pub down: (f64, f64),
   pub left: (f64, f64),
   pub right: (f64, f64),
   pub idle_up: (f64, f64),
   pub idle_down: (f64, f64),
   pub idle_left: (f64, f64),
   pub idle_right: (f64, f64),
   pub name: String,
}

impl Sprite{
    pub fn new(name: &str) -> Sprite {
        let sheet = web_sys::HtmlImageElement::new().unwrap();
        sheet.set_src(&format!("/assets/sprites/{}/sheet.png", name));
        let up = (0.0, 0.0);
        let down = (65.0, 0.0);
        let left = (129.0, 0.0);
        let right = (193.0, 0.0);
        let idle_up = (0.0, 33.0);
        let idle_down = (65.0, 33.0);
        let idle_left = (129.0, 33.0);
        let idle_right = (193.0, 33.0);

        Sprite {
            sheet,
            up,
            down,
            left,
            right,
            idle_up,
            idle_down,
            idle_left,
            idle_right,
            name: name.to_string(),
        }
    }
}