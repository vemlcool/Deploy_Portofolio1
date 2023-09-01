use gloo_storage::{SessionStorage, Storage};
use serde::Deserialize;
use yew_router::agent::RouteRequest::ChangeRoute;
use yew::{prelude::*, virtual_dom::key, services::ConsoleService};
use yew_router::prelude::*;
use crate::router::route::AppRoute;

pub enum Msg {
    AddOne,
    LogOut,
    Ignore,
    Render
}

#[derive(Debug, Deserialize)]
pub struct Token {
    pub access_token: String
}

pub struct NavtopConnector {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    value: i64,
}

impl Component for NavtopConnector {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {

        Self {
            link: link.clone(),
            value: 0,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::Ignore=>{
                false
            }
            Msg::LogOut => {
                
                SessionStorage::clear();
                self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
                true
            }
            Msg::Render => {
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }    

    fn view(&self) -> Html {

        type Anchor = RouterAnchor<AppRoute>;
        

        let key = Token{
            access_token: String::new()
        };

        let token: String = SessionStorage::get(key.access_token).unwrap_or("".to_string());
        if token.is_empty(){
            html! {
                <div class="header">       
                <input type="checkbox" class="openSidebarMenu" id="openSidebarMenu"/>
                    <label for="openSidebarMenu" class="sidebarIconToggle">
                        <div class="spinner diagonal part-1"></div>
                        <div class="spinner horizontal"></div>
                        <div class="spinner diagonal part-2"></div>
                    </label>
                <div class="sidebar" id="sidebarMenu">
                    <ul class="sidebarMenuInner">
                    <Anchor route=AppRoute::LandingPage>
                        <li>{"Digital Business"} <span>{"Web Development"}</span></li>
                    </Anchor>    
                        //Connector
                        <Anchor route=AppRoute::ConnectorHome>
                        <li class="sidebarDrop"><a>{"TelConnect"}</a>
                        </li>
                        </Anchor>
                        
                    </ul>
                </div>
            <h5 class="new-navbar" style="text-align:justify;">{"Digital Business"}</h5>
    
                <Anchor route=AppRoute::Login>
                    <p class="new-navbar" style="margin-left:75%">{"Login"}</p>
                </Anchor>
    
                <Anchor route=AppRoute::Register>
                    <p class="new-navbar" style="margin-left:85%">{"Register"}</p>
                </Anchor>
    
            </div>
            }
        } else {
            html! {
                <div class="header">       
                <input type="checkbox" class="openSidebarMenu" id="openSidebarMenu"/>
                    <label for="openSidebarMenu" class="sidebarIconToggle">
                        <div class="spinner diagonal part-1"></div>
                        <div class="spinner horizontal"></div>
                        <div class="spinner diagonal part-2"></div>
                    </label>
                <div class="sidebar" id="sidebarMenu">
                    <ul class="sidebarMenuInner">
                    <Anchor route=AppRoute::LandingPage>
                        <li>{"Digital Business"} <span>{"Web Development"}</span></li>
                    </Anchor>  
                        //Connector
                        <Anchor route=AppRoute::ConnectorHome>
                        <li class="sidebarDrop"><a>{"TelConnect"}</a>
                        </li>
                        </Anchor>
                        
                    </ul>
                </div>
            <h5 class="new-navbar" style="text-align:justify;">{"Digital Business"}</h5>
    
                
                <p class="new-navbar" style="margin-left:75%" onclick=self.link.callback(|_| Msg::LogOut)>{"Logout"}</p>
               
            </div>
            }
        }


        
    }
}