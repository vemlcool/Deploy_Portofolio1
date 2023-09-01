use std::{vec, ops::Index};

use gloo_storage::{SessionStorage, Storage};
use yew::{prelude::*, callback};
use yew_router::prelude::*;
use crate::router::route::AppRoute;
use bson::{doc, oid::ObjectId};
use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    }
};
use yew_router::agent::RouteRequest::ChangeRoute;

use crate::types::var::{
    GetConnector,
};

use crate::pages::Login::Token;

pub enum Msg {
    Ignore,
    RequestData,
    GetData(Vec<GetConnector>),
    DirectLogin,
    Unauth
}

pub struct ConnectorHome {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    homepage: Vec<GetConnector>,
    no_login:bool,
    unauthorized: bool,
}

impl Component for ConnectorHome {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("this is homepage..........");
        Self {
            homepage:vec![],
            fetch_task: None,
            link: link.clone(),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            no_login:false,
            unauthorized: false
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            // ConsoleService::info("this is first render homepage.....");
            self.link.send_message(Msg::RequestData)
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore=>{
                false
            }
            Msg::RequestData => {
                
            let key = Token{
                access_token: String::new()
            };

            let token: String = SessionStorage::get(key.access_token).unwrap_or("".to_string());           
            let bearer = format!("Bearer {}", &token);
            let request = Request::get("https://atlassian-connector-api.dev-domain.site/connector")
                    .header("Authorization", bearer)
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<GetConnector>, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        let status_number = meta.status.as_u16();
                        
                        match data {
                            Ok(dataok) => {
                                   Msg::GetData(dataok)                             
                            }
                            Err(error) => {
                                if status_number == 401 {
                                    ConsoleService::info(("masuk 401"));
                                    Msg::Unauth
                                } else{
                                    Msg::Ignore
                                }
                            }
                        }
                    });

                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                true
            }
            Msg::GetData(data) => {
                ConsoleService::info(&format!("data is {:?}", data));
                self.homepage = data;
                true
            }
            Msg::DirectLogin =>{
                ConsoleService::info(("Direct Jalan"));
   
                self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
                true
            }
            Msg::Unauth =>{
                let key = Token{
                    access_token: String::new()
                };
        
                let token: String = SessionStorage::get(key.access_token).unwrap_or("".to_string());
                ConsoleService::info("Masuk Unauth");
                if token.is_empty(){
                    self.no_login = true;
                    ConsoleService::info(&format!("nologin: {}", self.no_login));
                } else{
                    self.unauthorized = true;
                    ConsoleService::info(&format!("uantuh: {}", self.unauthorized));
                }
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
        if self.no_login == true || self.unauthorized == true {
            html!{
                {self.view_no_login()}
            }
        } else {
            html! {
                <div
                    style="
                        border:none;
                    "
                >
                    <div
                        style="
                            background: #E3E8ED; 
                            position: absolute;
                            padding-top: 95px;
                            right: 0;
                            left: 0;
                            overflow: auto;
                            height: 100%;
                            border:none;
                        "
                    >    
                        <div style="padding-bottom:30px;"> //button
                            <button type="button" style=" margin-left:87%; background:#A73034; border-color:#A73034;  color:white;  border-radius:15px; height: 40px;"> 
                                <Anchor route=AppRoute::Profile> 
                                    {"Profile"}  
                                </Anchor>
                            </button>
                        </div>

                        <div style="padding-bottom:30px;"> //button
                            <button type="button" style=" margin-left:87%; background:#A73034; border-color:#A73034;  color:white;  border-radius:15px; height: 40px;"> 
                                <Anchor route=AppRoute::ConnectorCreate> 
                                    {"Create Connector"}  
                                </Anchor>
                            </button>
                        </div>

                        
                        <div style="padding-bottom:30px;"> //button
                            <button type="button" style=" margin-left:87%; background:#A73034; border-color:#A73034;  color:white;  border-radius:15px; height: 40px;"> 
                                <Anchor route=AppRoute::Tutorial> 
                                    {"How to Use"}  
                                </Anchor>
                            </button>
                        </div>

                        {self.view_index_data()}
                        {self.view_empty()}
                    </div>
                </div>
            }
        }
    }
}

impl ConnectorHome {
    fn view_index_data(&self) -> Vec<Html> {
        type Anchor = RouterAnchor<AppRoute>;
        self.homepage.iter().map(|card|{
            ConsoleService::info(&format!("Name adalah {:?}",card.name.to_string()));
                html!{
                        <div class="card mt-4 mb-2"
                            style="
                                text-decoration:none;
                                background: white;
                                width:1200px;
                                margin:auto;
                                border:none;
                            "
                        >
                            <Anchor route=AppRoute::ConnectorSetting { _name:card.name.to_string()}>
                                <div class="card-body" style="color: gray;">
                                    <h4 class="card-title" 
                                        style="color: black;"
                                    >
                                        {&card.name}

                                        {
                                            if card.schedule == true && card.active == true{
                                                html!{
                                                    <span class="badge bg-info" style="margin-left: 10px">
                                                        {format!("On schedule: {}", card.duration)}
                                                    </span>
                                                }
                                            } else{
                                                html!{
                                                    
                                                }
                                            }
                                        } 
                                    </h4>
                                    
                                    <h6 class="card-title">
                                        {&card.description}
                                    </h6>

                                    {
                                        if card.active ==  true{
                                            html!{
                                                <span class="badge bg-success">
                                                    {"Active"}
                                                </span>
                                            }
                                        } else{
                                            html!{
                                                <span class="badge bg-danger">
                                                    {"Deactive"}
                                                </span>
                                            }
                                        }
                                    }
    
                                    <span class="badge bg-secondary" style="margin: 0 10px 0;">{format!("Created : {}",&card.created_at.format("%d %b %Y, %H:%M:%S").to_string())}</span>                      
                                    {
                                        if card.updated_at.is_some(){
                                            html!{
                                                <span class="badge bg-secondary">{format!("Updated : {}",&card.updated_at.unwrap().format("%d %b %Y, %H:%M:%S").to_string())}</span>
                                            }   
                                        }else{
                                            html!{

                                            }
                                        }

                                    }

                                </div>
                            </Anchor>
                        </div>
                    
                }
        }).collect()          
    }

    fn view_empty(&self) -> Html {
        if self.homepage.is_empty() && self.no_login == false && self.unauthorized == false{
            html!{
                <h5 style="text-align:center"> {"Connector List Empty"} </h5>
            }
        }else {
            html!{

            }
        }
    }

    fn view_no_login(&self) -> Html {
 
        if self.no_login == true{
            ConsoleService::info(&format!("Masuk nologin view"));
            html!{      
                <div
                    style="
                        border:none;
                        padding-top: 250px;
                    "
                >          
                <div style="background: #A73034; width:50%; height:200px; font-family: Alexandria; color: #A73034; 
                     margin: auto;
                     align-items: center; text-align:center" >
                    <h5 class="modal-title" style="padding-top:20px"> <p style="color:white"> {format!("Unauthorized!")} </p> </h5>
                    <div class="modal-body" style="color:black;" >
                        <p style="color:white; padding-left:20px; padding-right:20px; padding-bottom:30px"> {format!("You must be logged in to access this page!")} </p>
                    </div>
                        <button
                            type="button"
                            style="
                                background:#ffffff;
                                border-color:#ffffff;
                                color:white;
                                border-radius:15px;
                                width: 70px;
                                height: 40px; 
                            "

                            class="btn btn-primary"
                            data-bs-dismiss="modal"
                            onclick=self.link.callback(|_| Msg::DirectLogin)
                            >
                            <p style="color:#A73034"> {"Close"} </p>
                        </button>         
                    </div>
                </div>
            }
        } else if self.unauthorized == true{
            html!{
                <div
                style="
                    border:none;
                    padding-top: 250px;
                    "
                >          
                <div style="background: #A73034; width:50%; height:200px; font-family: Alexandria; color: #A73034; 
                    margin: auto;
                    align-items: center; text-align:center" >
                    <h5 class="modal-title" style="padding-top:20px"> <p style="color:white"> {format!("Unauthorized!")} </p> </h5>
                    <div class="modal-body" style="color:black;" >
                        <p style="color:white; padding-left:20px; padding-right:20px; padding-bottom:20px"> 
                            {format!("Your session token has already expired or is no longer valid!")} <br/> 
                            {format!("Please logout and login back to use this page")}
                        </p>
                    </div>
                        <button
                            type="button"
                            style="
                                background:#ffffff;
                                border-color:#ffffff;
                                color:white;
                                border-radius:15px;
                                width: 70px;
                                height: 40px; 
                            "

                            class="btn btn-primary"
                            data-bs-dismiss="modal"
                            onclick=self.link.callback(|_| Msg::DirectLogin)
                            >
                            <p style="color:#A73034"> {"Close"} </p>
                        </button>         
                    </div>
                </div>
            }
        }else{
            html!{
                
            }
        }
    }
}
