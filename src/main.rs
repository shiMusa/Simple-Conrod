#![feature(duration_extras)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]



extern crate sabi;


use sabi::{elements};
use elements::{*, action::*, basic::*, container::*};



fn main() {

    let animation = Animation::new();
    let mut button = Button::new()
        .with_id("testbutton".to_string());
    animation(&mut *button, ActionMsg::empty());
    animation(&mut *button, ActionMsg::empty());
    animation(&mut *button, ActionMsg::empty());
    animation(&mut *button, ActionMsg::empty());
    animation(&mut *button, ActionMsg::empty());
    animation(&mut *button, ActionMsg::empty());

    let mut multi_socket = MultiSocket::new(button)
        .with_action_receive(ActionMsg{
            sender_id: "testbutton".to_string(),
            msg: ActionMsgData::Click
        }, Box::new(animation));


    sabi::example();
}
