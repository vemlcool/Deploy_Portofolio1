use yew::{
    prelude::*,
    format::{Json},
    services::{
        ConsoleService,
        storage::{StorageService},
    },
};


use crate::components::NavtopConnector::NavtopConnector;

use crate::router::{
    render::Render,
};

pub enum Msg {
}

pub struct App {

}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {  }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {


        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
            html! {
                <div>
                    <NavtopConnector/>
                    <Render/>
                </div>
            }
    }
}
