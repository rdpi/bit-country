use stdweb::web::html_element::ImageElement;

#[derive(Debug)]
pub struct Sprite{
   pub up: ImageElement,
   pub down: ImageElement,
   pub left: ImageElement,
   pub right: ImageElement,
   pub idle: ImageElement,
   pub name: String,
}

impl Sprite{
    pub fn new(name: &str) -> Sprite {
        let up = ImageElement::new();
        up.set_src(&format!("/assets/sprites/{}/up.png", name));
        let down = ImageElement::new();
        down.set_src(&format!("/assets/sprites/{}/down.png", name));
        let left = ImageElement::new();
        left.set_src(&format!("/assets/sprites/{}/down.png", name));
        let right = ImageElement::new();
        right.set_src(&format!("/assets/sprites/{}/down.png", name));
        let idle = ImageElement::new();
        idle.set_src(&format!("/assets/sprites/{}/idle.png", name));

        Sprite {
            up,
            down,
            left,
            right,
            idle,
            name: name.to_string(),
        }
    }
}