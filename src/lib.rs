#[macro_use]
extern crate yew;
use yew::prelude::*;
use yew::services::{ConsoleService};

pub struct Model {
    counter: u8,
    console: ConsoleService,
}

pub enum Action {
    INCREASE,
    DECREASE
}

impl Component for Model {
    type Properties = ();
    type Message = Action;

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            counter: 0,
            console: ConsoleService {}
        }
    }
 
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Action::INCREASE => {
                self.counter += 1;
                self.console.log("Erhöht um 1");
                true
            },
            Action::DECREASE => {
                self.counter -= 1;
                self.console.log("Verringert um 1");
                true
            },
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <p>{"Counter: "}{self.counter}</p>
                <button class="btn btn-light btn-lg", onclick=|_| Action::INCREASE, >{"👍"}</button>
                <button class="btn btn-light btn-lg", onclick=|_| Action::DECREASE, >{"👎"}</button>
                <ul>{ 
                        for (0..self.counter).into_iter().map(|x| {
                            self.view_item()
                        })
                    }
                </ul>
            </div>
        }
    }
}

impl Model {
    fn view_item(&self) -> Html<Model> {
        html! {
            <li>{ "List item" }</li>
        }
    }
}