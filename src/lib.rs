mod canvas;
mod direction;
mod fren;
mod sprite;

use canvas::Canvas;
use direction::Direction;
use fren::Fren;
use sprite::Sprite;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use gloo::events::EventListener;

use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen(start)]
pub fn main() {

    let sprite = Sprite::new("apu");
    let canvas = Canvas::new("canvas", 1000, 750);
    let fren = Rc::new(RefCell::new(Fren::new(1000, 750, sprite)));
    let canvas = Rc::new(canvas);

    let fren_clone = fren.clone();
    input_handler(fren_clone);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    
    let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        i = (i+1)%30;
        fren.borrow().draw(&canvas, i);
        fren.borrow_mut().update();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window().unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}


fn input_handler(fren: Rc<RefCell<Fren>>){
    let document = web_sys::window().unwrap().document().unwrap();
    let fren_clone = fren.clone();
    let on_keydown = EventListener::new(&document, "keydown", move |event| {

        let keyboard_event = event.clone()
        .dyn_into::<web_sys::KeyboardEvent>()
        .unwrap();
        
        match &keyboard_event.key()[..]{
            "ArrowUp" => fren_clone.borrow_mut().change_direction(Some(Direction::Up)),
            "ArrowDown" => fren_clone.borrow_mut().change_direction(Some(Direction::Down)),
            "ArrowLeft" => fren_clone.borrow_mut().change_direction(Some(Direction::Left)),
            "ArrowRight" => fren_clone.borrow_mut().change_direction(Some(Direction::Right)),
            _ => {},
        }

    });
    on_keydown.forget();

    let fren_clone = fren.clone();
    let on_keyup = EventListener::new(&document, "keyup", move |event| {

        let keyboard_event = event.clone()
        .dyn_into::<web_sys::KeyboardEvent>()
        .unwrap();
        match &keyboard_event.key()[..]{
            "ArrowUp" => fren_clone.borrow_mut().change_direction(Some(Direction::IdleUp)),
            "ArrowDown" => fren_clone.borrow_mut().change_direction(Some(Direction::IdleDown)),
            "ArrowLeft" => fren_clone.borrow_mut().change_direction(Some(Direction::IdleLeft)),
            "ArrowRight" => fren_clone.borrow_mut().change_direction(Some(Direction::IdleRight)),
            _ => {},
        }

    });
    on_keyup.forget();
}