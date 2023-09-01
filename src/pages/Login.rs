use base64::engine::general_purpose;
use yew::html::Scope;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::route::AppRoute;
use yew_router::agent::RouteRequest::ChangeRoute;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
        storage::{Area, StorageService}
    }
};
use base64::{self, Engine};

use crate::types::var::{
    UserInput,
    MsgErr,
};

use gloo_storage::{SessionStorage, Storage};
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Token {
    pub access_token: String
}

pub enum Msg {
    InputUsername(String), // name
    InputPassword(String), //Description
    Ignore, 
    Direct, //go to homepage 
    CreateConnector,
    InputErrorMsg(String),
    CreateValidate,
    ErrorBotType(String),
    CheckInput(String),
    CheckSuccess,
}

pub struct Login {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    user_input:UserInput,
    fetch_task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    msg_err:MsgErr,
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("Login Page");
        Self {
            user_input: UserInput { 
                username:"".to_string(),
                password:"".to_string(),
            },
            msg_err:MsgErr { 
                header:"".to_string(),
                body:"".to_string(),
            },
            fetch_task: None,
            link: link.clone(),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
        }
    }
    
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            ConsoleService::info("this is first render homepage.....");

        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputUsername(name)=>{
                self.user_input.username = name;
                ConsoleService::info(&format!("Username adalah: {:?}", self.user_input));
                true
            }
            Msg::InputPassword(description)=>{
                self.user_input.password = description;
                ConsoleService::info(&format!("Password adalah: {:?}", self.user_input));
                true
            }
            //test
            Msg::CreateConnector => {
                

                let username = self.user_input.username.clone();
                let password = self.user_input.password.clone();
                
                let auth = format!("{}:{}", username, password);
                let b64 = general_purpose::STANDARD.encode(auth);

                let basic = format!("Basic {}", b64);

                let request = Request::get("https://atlassian-connector-api.dev-domain.site/login")
                    .header("Authorization", basic)
                    // .header("Content-Type", "application/json")
                    .body(Nothing)
                    .expect("Could not build request.");


                let callback = 
                    self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();

                        let status_number = meta.status.as_u16();

                        ConsoleService::info(&format!("data is {:?}", &data));
                        ConsoleService::info(&format!("status is {:?}", status_number));

                        let key = Token{
                            access_token: String::new()
                        };

                        if meta.status.is_success(){
                            match data{
                                Ok(dataok) =>{
                                    SessionStorage::set(key.access_token, dataok.clone()).unwrap();
                                    Msg::CheckInput(dataok)
                                },
                                Err(error) => {
                                    ConsoleService::info("ignore.");
                                    Msg::Ignore
                                }
                            }
                        }else{
                            match data {
                                Ok(dataok) => {
                                    ConsoleService::info(&format!("data response {:?}", &dataok));
                                    Msg::InputErrorMsg(dataok)
                                }
                                Err(error) => {
                                    ConsoleService::info("ignore.");
                                    Msg::Ignore
                                }
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);


                true
            }
            Msg::Direct=> {
                ConsoleService::info(("Direct Jalan"));
                // Mes::Render;
                // let navtop = NavtopConnector{
                //     link: Scope::get_component(NavtopConnector),
                //     router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
                //     value: 0,
                // };

                self.router_agent.send(ChangeRoute(AppRoute::ConnectorHome.into()));
                true
            }
            Msg::Ignore => {
                false
            }
            Msg::InputErrorMsg(dataok) => {
                self.msg_err.header = "Error".to_string();
                self.msg_err.body = dataok;
                true
            }
            Msg::ErrorBotType(data) => {

                true
            }
            Msg::CheckInput(dataok) => {
                if self.msg_err.body.is_empty(){
                    self.msg_err.header = "Success".to_string();
                    self.msg_err.body = format!("You have successfuly log in");
                }else{
                    self.link.send_message(Msg::Ignore);
                }
                true
            }
            Msg::CreateValidate => {
                if self.user_input.username.is_empty(){
                   self.msg_err.header = "Error".to_string();
                   self.msg_err.body = "Username cannot be empty".to_string();
                }else{
                    if self.user_input.password.is_empty(){
                        self.msg_err.header = "Error".to_string();
                        self.msg_err.body = "Password field cannot be empty".to_string();
                    }else{
                        self.msg_err.body = "".to_string();
                        ConsoleService::info(&format!("msg err body {}", self.msg_err.body));
                        self.link.send_message(Msg::CreateConnector);
                    }
                }      
                true
            }

            Msg::CheckSuccess => {           
                if self.msg_err.header == "Success"{
                    self.link.send_message(Msg::Direct)
                }else{
                    self.link.send_message(Msg::Ignore)
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
        html! {
            <div class="base-form">
                <div class="create-connector" style="width: 500px">
                    <h5>{"Login Form"}</h5>
                    <div class="input-group" style=" margin: auto; width: 300px">
                        <input type="text"  id="emailInput" class="form-control p-3 my-2 " placeholder="Username"
                        style="
                            width: 400px;
                            height:30px;
                            margin:auto;
                        "
                            value={self.user_input.username.clone()}
                            oninput=self.link.callback(|data: InputData| Msg::InputUsername(data.value))
                            
                        />
                    </div>

                    <div class="input-group" style=" margin: auto; width: 300px">
                        <input type="password" id="emailInput" class="form-control p-3 my-2 " placeholder="Password"
                        style="
                            height:30px;
                            margin:auto;
                        "
                            value={self.user_input.password.clone()}
                            oninput=self.link.callback(|data: InputData| Msg::InputPassword(data.value))
                        />
                    </div>
                    
                    
                    
                    <p style="color:#3a3b3a; font-size: 12px">
                            {"Haven't created an account? "} 
                            <Anchor route=AppRoute::Register>
                                <u style="color:black; font-size: 12px">
                                    {"Click Here"}
                                </u>
                            </Anchor>    
                    </p>
                    
                   

                    <div style=" text-align:center;"
                    >
                        <button type="button" class="home"
                                style="
                                background:#A73034;
                                border-color:#A73034;
                                color:white;
                                border-radius:15px;
                                height: 40px;
                                margin-top:15px;
                                
                            " 
                            data-bs-toggle="modal"
                            data-bs-target="#display_msg"
                            onclick=self.link.callback(|_| Msg::CreateValidate)
                        >
                            {"Login"}
                        </button>
                    </div>
                    
                </div>
                    {self.msg_1()}      
            </div>
          
        }
    }
}

impl Login{
    fn msg_1(&self)->Html{
        html!{
            <div style="background: #A73034; font-family: Alexandria; color: #A73034;" >
                <div class="modal fade" id="display_msg" data-bs-backdrop="static" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true"
                >
                    <div class="modal-dialog"
                    >
                        <div class="modal-content"
                        >
                            <div class="modal-header"
                            >
                                <h5 class="modal-tittle"> <p> {format!("{} !",self.msg_err.header)} </p> </h5>
                                <button 
                                    type="button"
                                    class="btn-close"
                                    data-bs-dismiss="modal"
                                    aria-label="close"
                                    onclick=self.link.callback(|_| Msg::CheckSuccess)
                                >
                                </button>
                            </div>
                            <div class="modal-body" style="color:black;" >
                                <p> {format!("{} !",self.msg_err.body)} </p>
                            </div>
                            <div class="modal-footer"
                            >
                                <button
                                    type="button"
                                    style="
                                        background:#A73034;
                                        border-color:#A73034;
                                        color:white;
                                        border-radius:15px;
                                        width: 70px;
                                        height: 40px; 
                                    "

                                    class="btn btn-primary"
                                    data-bs-dismiss="modal"
                                    onclick=self.link.callback(|_| Msg::CheckSuccess)
                                >
                                <p> {"Close"} </p>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
        
    }
}
