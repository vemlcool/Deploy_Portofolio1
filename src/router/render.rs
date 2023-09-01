use yew::prelude::*;
use yew_router::{
    prelude::*,
    service::RouteService,
};

use crate::pages::{
    ConnectorHome::ConnectorHome,
    ConnectorCreate::ConnectorCreate,
    ConnectorSetting::ConnectorSetting,
    LandingPage::LandingPage,
    Login::Login,
    Register::Register,
    Profile::Profile,
    WebhookCreate::WebhookCreate,
    Tutorial::Tutorial
};
use crate::router::route::AppRoute;

pub enum Msg {}


pub struct Render {}

impl Component for Render {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let render = Router::render(|switch: AppRoute| {
            let mut route_service = RouteService::new();
            match switch {
                // Connector
                AppRoute::LandingPage => {
                    html! {
                        <LandingPage/>
                    }
                }
                AppRoute::ConnectorHome => {
                    html! {
                        <ConnectorHome/>
                    }
                }
                AppRoute::ConnectorCreate => {
                    html! {
                        <ConnectorCreate/>
                    }
                }
                AppRoute::ConnectorSetting {_name} => {
                    html! {
                        <ConnectorSetting _name=_name/>
                    }
                }

                AppRoute::Login => {
                    html! {
                        <Login/>
                    }
                }

                AppRoute::Register => {
                    html! {
                        <Register/>
                    }
                }

                AppRoute::Profile => {
                    html! {
                        <Profile/>
                    }
                }

                AppRoute::WebhookCreate => {
                    html! {
                        <WebhookCreate/>
                    }
                }

                AppRoute::Tutorial => {
                    html! {
                        <Tutorial/>
                    }
                }

                _ => {
                    route_service.set_route("/", ());
                    html! {
                        <LandingPage/>
                    }

                }
            }
        });


        html! {
            <Router<AppRoute, ()> render=render/>
        }
    }
}
