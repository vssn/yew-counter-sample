extern crate yew;
extern crate yew_counter_sample;

use stdweb::web::{document, IParentNode};
use yew::prelude::*;
use yew_counter_sample::Model;

fn main() {
    yew::initialize();

    let element = document()
        .query_selector("#app")
        .expect("#app in place")
        .expect("is an element");
    App::<Model>::new().mount(element);
    yew::run_loop();
}