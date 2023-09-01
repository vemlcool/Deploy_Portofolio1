use gloo_storage::{SessionStorage, Storage};
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
    }
};
use base64::{self, Engine};

use crate::types::var::{
    WebhookInput,
    MsgErr
};

use super::Login::Token;


pub enum Msg {
    InputJiraEmail(String), // name
    InputJiraApiKey(String), //Description
    InputJiraUrl(String),
    Ignore, 
    Direct, //go to homepage 
    CreateWebhook,
    InputErrorMsg(String),
    CreateValidate,
    CheckInput,
    CheckSuccess,
    DirectLogin,
    Unauth,
    ApiKeyInfo,
    JiraUrlInfo,
    EmailInfo
}

pub struct WebhookCreate {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    user_input:WebhookInput,
    fetch_task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    msg_err:MsgErr,
    no_login:bool,
    unauthorized:bool,
}

impl Component for WebhookCreate {
    type Message = Msg;
    type Properties = ();

    
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("Create Webhook Page");
        Self {
            user_input: WebhookInput { 
                email:"".to_string(),
                api_key:"".to_string(),
                jira_url:"".to_string(),
            },
            msg_err:MsgErr { 
                header:"".to_string(),
                body:"".to_string(),
            },
            fetch_task: None,
            link: link.clone(),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            no_login:false,
            unauthorized:false
        }
    }
    
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            ConsoleService::info("this is first render homepage.....");

        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputJiraEmail(email)=>{
                self.user_input.email = email;
                ConsoleService::info(&format!("Jira Email adalah: {:?}", self.user_input));
                true
            }
            Msg::InputJiraApiKey(api_key)=>{
                self.user_input.api_key = api_key;
                ConsoleService::info(&format!("Api Key adalah: {:?}", self.user_input));
                true
            }
            Msg::InputJiraUrl(jira_url)=>{
                self.user_input.jira_url = jira_url;
                ConsoleService::info(&format!("Jira URL adalah: {:?}", self.user_input));
                true
            }

            //test
            Msg::CreateWebhook => {
                
                let user = WebhookInput {
                    email: self.user_input.email.clone(),
                    api_key: self.user_input.api_key.clone(),
                    jira_url: self.user_input.jira_url.clone()
                };

                let key = Token{
                    access_token: String::new()
                };
    
                let token: String = SessionStorage::get(key.access_token).unwrap_or("".to_string());
                let bearer = format!("Bearer {}", token);


                let request = Request::post("https://atlassian-connector-api.dev-domain.site/webhook")
                    .header("Authorization", bearer)
                    .header("Content-Type", "application/json")
                    .body(Json(&user))
                    .expect("Could not build request.");

                let callback = 
                    self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();

                        let status_number = meta.status.as_u16();

                        ConsoleService::info(&format!("data is {:?}", data));
                        ConsoleService::info(&format!("status is {:?}", status_number));

                        if meta.status.is_success(){
                            Msg::CheckInput
                        }else{
                            match data {
                                Ok(dataok) => {
                                    ConsoleService::info(&format!("data response {:?}", &dataok));
                                    Msg::InputErrorMsg(dataok)
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
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);

                true
            }
            Msg::Direct=> {
                ConsoleService::info(("Direct Jalan"));

                self.router_agent.send(ChangeRoute(AppRoute::Profile.into()));
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
            Msg::CheckInput => {
                if self.msg_err.body.is_empty(){
                    self.msg_err.header = "Success".to_string();
                    self.msg_err.body = "You have successfuly created a webhook".to_string();
                }else{
                    self.link.send_message(Msg::Ignore);
                }
                true
            }
            Msg::ApiKeyInfo => {
                self.msg_err.header = "API Token".to_string();
                self.msg_err.body = format!("
                    You can get your API Token by logging into your Jira account
                    and going to this URL https://id.atlassian.com/manage-profile/security/api-tokens
                ");
                true
            }
            Msg::JiraUrlInfo => {
                self.msg_err.header = "Jira Url".to_string();
                self.msg_err.body = format!("
                    Go to your Jira software and your Jira url is the first part of the url (Ex. for \"jiraurlnametest.atlassian.net/jira/software/\", jiraurlnametest.atlassian.net is the Jira url)
                ");
                true
            }
            Msg::EmailInfo => {
                self.msg_err.header = "Jira Email".to_string();
                self.msg_err.body = format!("
                    Email used for the registration of your Jira account
                ");
                true
            }
            Msg::CreateValidate => {
                if self.user_input.email.is_empty(){
                   self.msg_err.header = "Error".to_string();
                   self.msg_err.body = "Jira Email cannot be empty".to_string();
                }else{
                    if self.user_input.api_key.is_empty(){
                        self.msg_err.header = "Error".to_string();
                        self.msg_err.body = "Jira API Key field cannot be empty".to_string();
                    }else{
                        if self.user_input.jira_url.is_empty(){
                            self.msg_err.header = "Error".to_string();
                            self.msg_err.body = "Jira URL field cannot be empty".to_string();
                        }else{
                            self.msg_err.body = "".to_string();
                            ConsoleService::info(&format!("msg err body {}", self.msg_err.body));
                            self.link.send_message(Msg::CreateWebhook);
                        }
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

            Msg::Unauth =>{
                let key = Token{
                    access_token: String::new()
                };
        
                let token: String = SessionStorage::get(key.access_token).unwrap_or("".to_string());
                ConsoleService::info("Masuk Unauth");
                self.msg_err.header = "Error".to_string();
                if token.is_empty(){
                    self.no_login = true;
                    ConsoleService::info(&format!("nologin: {}", self.no_login));
    
                    self.msg_err.body = "You must be login to access this page function".to_string();
                } else{
                    self.unauthorized = true;
                    ConsoleService::info(&format!("uantuh: {}", self.unauthorized));

                    self.msg_err.body = "Your session token has already expired or is no longer valid!\nPlease logout and login back to use this page".to_string();
                }
                true
            }

            Msg::DirectLogin =>{
                ConsoleService::info(("Direct Jalan"));
   
                self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
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
        html! {
            <div class="base-form">
                <div class="create-connector" style="width: 500px">
                    <h5>{"Create Webhook Form"}</h5>

                    <div class="input-group" style=" margin: auto; width: 300px">
                        <input type="text"  id="emailInput" class="form-control p-3 my-2 " placeholder="Jira Email"
                        style="
                            height:30px;
                            margin:auto;
                        "
                            value={self.user_input.email.clone()}
                            oninput=self.link.callback(|data: InputData| Msg::InputJiraEmail(data.value))
                            
                        />
                        <button 
                            type="button"
                            style="
                            height:30px;
                            margin:auto;
                            "

                            data-bs-toggle="modal"
                            data-bs-target="#display_msg"
                            onclick=self.link.callback(|_| Msg::EmailInfo)
                            >

                            {"?"}
                        </button>
                    </div>

                    <div class="input-group" style=" margin: auto; width: 300px">
                        <input type="text" id="emailInput" class="form-control p-3 my-2 " placeholder="Jira API Token"
                        style="
                            height:30px;
                            margin:auto;
                        "
                            value={self.user_input.api_key.clone()}
                            oninput=self.link.callback(|data: InputData| Msg::InputJiraApiKey(data.value))
                        />
                         <button 
                                type="button"
                                style="
                                height:30px;
                                margin:auto;
                                "

                                data-bs-toggle="modal"
                                data-bs-target="#display_msg"
                                onclick=self.link.callback(|_| Msg::ApiKeyInfo)
                            >

                            {"?"}
                        </button>
                    </div>
                   
                    <div class="input-group" style=" margin: auto; width: 300px">
                        <input type="text" id="emailInput" class="form-control p-3 my-2 " placeholder="Jira URL"
                        style="
                            height:30px;
                            margin:auto;
                        "
                            value={self.user_input.jira_url.clone()}
                            oninput=self.link.callback(|data: InputData| Msg::InputJiraUrl(data.value))
                        />

                        <button 
                                type="button"
                                style="
                                height:30px;
                                margin:auto;
                                "

                                data-bs-toggle="modal"
                                data-bs-target="#display_msg"
                                onclick=self.link.callback(|_| Msg::JiraUrlInfo)
                            >
                            {"?"}
                        </button>
                    </div>
                
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
                            {"Submit"}
                        </button>
                    </div>
                    
                </div>
                    {self.msg_1()}      
            </div>
          
        }
    }
}

impl WebhookCreate{
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
                                <p> {format!("{}",self.msg_err.body)} </p>
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
