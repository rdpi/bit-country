#[macro_use]
extern crate stdweb;

mod canvas;
mod direction;
mod fren;
mod sprite;

use canvas::Canvas;
use direction::Direction;
use fren::Fren;
use sprite::Sprite;

use stdweb::traits::*;
use stdweb::web::{event::KeyDownEvent, event::KeyUpEvent, IEventTarget, html_element::ImageElement};

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    stdweb::initialize();

    let sprite = Sprite::new("apu");
    let canvas = Canvas::new("canvas", 400, 400);
    let fren = Rc::new(RefCell::new(Fren::new(400, 400, sprite)));

    fren.borrow().draw(&canvas, 0);

    stdweb::web::document().add_event_listener({
        let fren = fren.clone();
        move |event: KeyDownEvent| {
            match event.key().as_ref() {
                "ArrowLeft" => fren.borrow_mut().change_direction(Some(Direction::Left)),
                "ArrowRight" => fren.borrow_mut().change_direction(Some(Direction::Right)),
                "ArrowDown" => fren.borrow_mut().change_direction(Some(Direction::Down)),
                "ArrowUp" => fren.borrow_mut().change_direction(Some(Direction::Up)),
                _ => {}
            };
        }
    });

    stdweb::web::document().add_event_listener({
        let fren = fren.clone();
        move |event: KeyUpEvent| {
            match event.key() {
                _ => fren.borrow_mut().change_direction(None),
            };
        }
    });

    fn game_loop(fren : Rc<RefCell<Fren>>, canvas: Rc<Canvas>, time: u32, frame: u8) {
        stdweb::web::set_timeout(
            move || {
                let frame = (frame + 1)%32;
                game_loop(fren .clone(), canvas.clone(), time, frame);
                fren.borrow_mut().update();
                fren.borrow().draw(&canvas, frame);
            },
            time,
        );
    }

    game_loop(fren , Rc::new(canvas), 16, 0);

    stdweb::event_loop();
}