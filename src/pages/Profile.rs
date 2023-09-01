use std::{vec, ops::Index};

use gloo_storage::{SessionStorage, Storage};
use yew::{prelude::*, callback};
use yew_router::agent::RouteRequest::ChangeRoute;
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

use crate::types::var::{
    UserInfo, MsgErr
};

use crate::pages::Login::Token;

pub enum Msg {
    Ignore,
    RequestData,
    GetData(UserInfo),
    CheckWebhook,
    RepairWebhook,
    Direct,
    CheckInput(String),
    InputErrorMsg(String),
    CheckSuccess,
    DeleteWebhook,
    DeleteAccount,
    DeleteAccountSuccess,
    DeleteWebhookSuccess,
    DirectLogin,
    Unauth
}

pub struct Profile {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    homepage: UserInfo,
    msg_err:MsgErr,
    no_login: bool,
    unauthorized: bool
}

impl Component for Profile {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("this is homepage..........");
        Self {
            homepage:UserInfo { 
                username:"".to_string(), 
                password:"".to_string(), 
                created_at: chrono::Utc::now()
                    .with_timezone(&chrono::FixedOffset::east_opt(7 * 3600).unwrap()),
                jira_email: None,
                jira_api_key: None,
                jira_url: None,
                webhook_url: None,
                webhook_functional: None,
                webhook_last_check: None
            },
            msg_err:MsgErr { 
                header:"".to_string(),
                body:"".to_string(),
            },
            
            link: link.clone(),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            fetch_task: None,
            no_login: false,
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
            Msg::RequestData => {
                
                let key = Token{
                    access_token: String::new()
                };

                let token: String = SessionStorage::get(key.access_token).unwrap_or("".to_string());            
                let bearer = format!("Bearer {}", token);

                let request = Request::get("https://atlassian-connector-api.dev-domain.site/user")
                    .header("Authorization", bearer)
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<UserInfo, anyhow::Error>>>| {
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
            Msg::CheckWebhook => {
                
                let key = Token{
                    access_token: String::new()
                };
    
                let token: String = SessionStorage::get(key.access_token).unwrap_or("".to_string());            
                let bearer = format!("Bearer {}", token);
    
                  let request = Request::get("https://atlassian-connector-api.dev-domain.site/webhook")
                        .header("Authorization", bearer)
                        .body(Nothing)
                        .expect("Could not build request.");
                    let callback = 
                        self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                            let (meta, Json(data)) = response.into_parts();
                            let status_number = meta.status.as_u16();
                            

                            if meta.status.is_success(){
                                Msg::CheckInput(data.unwrap())
                            }else{
                                match data {
                                    Ok(dataok) => {
                                        Msg::InputErrorMsg(dataok)
                                    }
                                    Err(error) => {
                                        ConsoleService::info(&format!("error is {:?}", error.to_string()));
                                        Msg::Ignore
                                    }
                                }
                            }
                        });
                    let task = FetchService::fetch(request, callback).expect("failed to start request");
                    self.fetch_task = Some(task);
                    true
            }
            Msg::RepairWebhook => {
                
                let key = Token{
                    access_token: String::new()
                };
    
                let token: String = SessionStorage::get(key.access_token).unwrap_or("".to_string());            
                let bearer = format!("Bearer {}", token);
    
                  let request = Request::get("https://atlassian-connector-api.dev-domain.site/webhook/repair")
                        .header("Authorization", bearer)
                        .body(Nothing)
                        .expect("Could not build request.");
                    let callback = 
                        self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                            let (meta, Json(data)) = response.into_parts();
                            let status_number = meta.status.as_u16();
                            
                            if meta.status.is_success(){
                                Msg::CheckInput(data.unwrap())
                            }else{
                                match data {
                                    Ok(dataok) => {
                                        Msg::InputErrorMsg(dataok)

                                    }
                                    Err(error) => {
                                        Msg::Ignore
                                    }
                                }
                            }
                        });
                    let task = FetchService::fetch(request, callback).expect("failed to start request");
                    self.fetch_task = Some(task);
                    true
            }

            Msg::InputErrorMsg(dataok) => {
                self.msg_err.header = "Error".to_string();
                ConsoleService::info(&format!("Checkfail {:?}", dataok));
                self.msg_err.body = dataok;
                true
            }

            Msg::CheckInput(dataok) => {
                
                self.msg_err.header = "Webhook Check Result".to_string();
                ConsoleService::info(&format!("Checkok {:?}", dataok));

                self.msg_err.body = dataok;
                
                true
            }

            Msg::Direct=> {
                ConsoleService::info(("Direct Back"));

                self.router_agent.send(ChangeRoute(AppRoute::Profile.into()));
                true
            }

            Msg::CheckSuccess => {           
                if self.msg_err.header == "Webhook Check Result"{
                    self.link.send_message(Msg::Direct)
                }else{
                    self.link.send_message(Msg::Ignore)
                }                 

                true
            }
            Msg::DeleteAccountSuccess => {
                self.msg_err.header = "Delete Account Result".to_string();
                self.msg_err.body = "Your account has been successfully deleted".to_string();
                true
            }
            Msg::DeleteWebhookSuccess => {
                self.msg_err.header = "Delete Webhook Result".to_string();
                self.msg_err.body = "Your Webhook has been successfully deleted".to_string();
                true
            }
            Msg::DeleteWebhook=> {
                //FETCHING...
                let key = Token{
                    access_token: String::new()
                };

                let token: String = SessionStorage::get(key.access_token).unwrap_or("".to_string());            
                let bearer = format!("Bearer {}", token);

                let request = Request::delete(format!("https://atlassian-connector-api.dev-domain.site/webhook"))
                    .header("Authorization", bearer)
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        let status_number = meta.status.as_u16();
                        ConsoleService::info(&format!("data response {:?}", &data));

                        if meta.status.is_success(){
                            Msg::DeleteWebhookSuccess
                        }else{
                            match data {
                                Ok(dataok) => {
                                    ConsoleService::info(&format!("data response not ok {:?}", &dataok));
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

            Msg::DeleteAccount=> {
                //FETCHING...
                let key = Token{
                    access_token: String::new()
                };

                let token: String = SessionStorage::get(key.access_token).unwrap_or("".to_string());            
                let bearer = format!("Bearer {}", token);

                let request = Request::delete(format!("https://atlassian-connector-api.dev-domain.site/user"))
                    .header("Authorization", bearer)
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        let status_number = meta.status.as_u16();
                        ConsoleService::info(&format!("data response {:?}", &data));

                        if meta.status.is_success(){
                            Msg::DeleteAccountSuccess
                        }else{
                            match data {
                                Ok(dataok) => {
                                    ConsoleService::info(&format!("data response not ok {:?}", &dataok));
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
        type Anchor = RouterAnchor<AppRoute>;        
        ConsoleService::info(&format!("Data reload adalah {:?}",self.homepage));
        if self.no_login == true || self.unauthorized == true {
            html!{self.view_no_login()}
        } else {
            html! {
                <div class="base-form">
                    <div style="padding-bottom:30px;"> //button
                        <button type="button" style=" margin-left:87%; background:#A73034; border-color:#A73034;  color:white;  border-radius:15px; height: 40px;"> 
                            <Anchor route=AppRoute::ConnectorHome> 
                                {"Connector Page"}  
                            </Anchor>
                        </button>
                    </div>

                    <div class="create-connector" style=" margin: auto; width: 900px">
                    <h5>{"Profile"}</h5>

                    <div class="input-group" style=" margin: auto; width: 500px">
                        <label for="username" style="margin-top:15px; margin-right:33px"> {"Username"} </label>
                        <input type="text"  id="username" class="form-control p-3 my-2 " placeholder="Username"
                        style="
                            height:30px;
                            margin:auto;
                        "
                            value={self.homepage.username.clone()}
                            readonly=true
                        />
                    </div>

                    <div class="input-group" style=" margin: auto; width: 500px">
                    <label for="created" style="margin-top:15px; margin-right:31px"> {"Created at"} </label>
                        <input type="text" id="created" class="form-control p-3 my-2 " placeholder="Date"
                        style="
                            height:30px;
                            margin:auto;
                        "
                            value={self.homepage.created_at.clone().format("%d %b %Y, %H:%M:%S").to_string()}
                            readonly=true
                        />
                    </div>


                    <div class="input-group" style=" margin: auto; width: 500px">
                    <label for="email" style="margin-top:15px; margin-right:36px"> {"Jira Email"} </label>
                        <input type="text" id="email" class="form-control p-3 my-2 " placeholder="Email"
                        style="
                            height:30px;
                            margin:auto;
                        "
                            value={self.homepage.jira_email.clone().unwrap_or("None".to_string())}
                            readonly=true
                        />
                    </div>

                    <div class="input-group" style=" margin: auto; width: 500px">
                    <label for="apikey" style="margin-top:15px; margin-right:10px; font-size:15px"> {"Jira API Token"} </label>
                        <input type="text" id="apikey" class="form-control p-3 my-2 " placeholder="Apikey"
                        style="
                            height:30px;
                            margin:auto;
                        "
                            value={self.homepage.jira_api_key.clone().unwrap_or("None".to_string())}
                            readonly=true
                        />
                    </div>
                    
                    <div class="input-group" style=" margin: auto; width: 500px">
                    <label for="url" style="margin-top:15px; margin-right:50px"> {"Jira URL"} </label>
                        <input type="text" id="url" class="form-control p-3 my-2 " placeholder="URL"
                        style="
                            height:30px;
                            margin:auto;
                        "
                            value={self.homepage.jira_url.clone().unwrap_or("None".to_string())}
                            readonly=true
                        />
                    </div>

                    <div class="input-group" style=" margin: auto; width: 500px">
                    <label style="margin-top:15px; margin-right:62px"> {"Webhook Status"} </label>
                        
                            {                                      
                                if self.homepage.webhook_functional.clone().unwrap_or(false) == true{
                                    html!{
                                        <span class="badge bg-success" style="padding-top:15px">
                                            {"Functional"}
                                        </span>
                                    }
                                } else{
                                    html!{
                                        <span class="badge bg-warning" style="padding-top:15px; color:white">
                                            {"Nonfunctional"}
                                        </span>
                                    }
                                }
                            }
                        
                        
                    </div>

                    <div class="input-group" style=" margin: auto; width: 500px;margin-bottom:10px">
                    <label for="check" style="margin-top:15px; margin-right:25px"> {"Webhook Last Check"} </label>
                        <input type="text" id="check" class="form-control p-3 my-2 " placeholder="Check"
                        style="
                            height:30px;
                            margin:auto;
                        "
                            value={self.homepage.webhook_last_check.clone().unwrap_or("None".to_string())}
                            readonly=true
                        />
                    </div>

                    {
                        if self.homepage.webhook_url.is_none(){
                            html!{
                                <div style=" text-align:center; margin-bottom:10px"
                                >
                                    <button type="button" class="btn btn-primary"
                                >
                                        <Anchor route=AppRoute::WebhookCreate> 
                                            {"Create Webhook"}  
                                        </Anchor>
                                    </button>
                                </div>    
                            }
                        } else {
                            html!{
                                <div style=" text-align:center; margin-bottom:10px"
                                >
                                    <button type="button" class="btn btn-danger" style="margin-right:5px"
                                        data-bs-toggle="modal"
                                        data-bs-target="#DeleteWebhookModal"
                                    >
                                        {"Delete Webhook"}
                                    </button>
            
                                    <button type="button" class="btn btn-warning" style="margin-right:5px; color:white"
                                        data-bs-toggle="modal"
                                        data-bs-target="#display_msg"
                                        onclick=self.link.callback(|_| Msg::RepairWebhook)
                                    >
                                        {"Repair Webhook"}
                                    </button>

                                    <button type="button" class="btn btn-success" style="margin-right:5px"
                                        
                                        data-bs-toggle="modal"
                                        data-bs-target="#display_msg"
                                        onclick=self.link.callback(|_| Msg::CheckWebhook)
                                    >
                                        {"Check Webhook"}
                                    </button>
                                </div>
                            }
                        }
                    }        
                   
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
                                data-bs-target="#DeleteAccount"
                            >
                                {"Delete Account"}
                            </button>
                        </div>

                    <div style=" text-align:justify;"
                    >
                        
                    </div>
                    
                </div>
                    {self.msg_1()}      
                    {self.msg_2()}
                    {self.msg_3()} 
            </div>
        
            }
        }
    }
}

impl Profile {
    fn msg_1(&self)->Html{
        html!{
            <div style="background: #A73034; font-family: Alexandria; color: #A73034; z-index: 100000" >
                <div class="modal fade" id="display_msg" data-bs-backdrop="static" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true"
                >
                    <div class="modal-dialog"
                    >
                        <div class="modal-content"
                        >
                            <div class="modal-header"
                            >
                                <h5 class="modal-tittle"> <p> {format!("{}!",self.msg_err.header)} </p> </h5>
                                <button 
                                    type="button"
                                    class="btn-close"
                                    data-bs-dismiss="modal"
                                    aria-label="close"
                                    onclick=self.link.callback(|_| Msg::RequestData)
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
                                    onclick=self.link.callback(|_| Msg::RequestData)
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

    fn msg_2(&self) -> Html{
        html!{
            <div class="modal fade" id="DeleteWebhookModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true"
            >
                <div class="modal-dialog"
                >
                    <div class="modal-content"
                    >
                        <div class="modal-header" style="color:black;"
                        >
                            <h5 class="modal-tittle"><p> {"Delete Confirmation"} </p> </h5>
                            <button 
                                type="button"
                                class="btn-close"
                                data-bs-dismiss="modal"
                                aria-label="close"
                            >
                            </button>
                        </div>
                        <div class="modal-body" style="color:black;" >
                            <p> {"Are you sure you want to delete your webhook? (This action cannot be undone)"} </p>
                        </div>
                        <div class="modal-footer"
                        >
                            <button
                                type="button"
                                class="btn btn-secondary"
                                data-bs-dismiss="modal"
                                onclick=self.link.callback(|_| Msg::Ignore) 
                            >
                                {"Cancel"}
                            </button> 
                            
                            <button
                                type="button"
                                class="btn btn-danger"
                                data-bs-dismiss="modal"
                                data-bs-toggle="modal"
                                data-bs-target="#display_msg"
                                onclick=self.link.callback(|_| Msg::DeleteWebhook) 
                            >
                                {"Delete"}
                            </button>
                        </div>
                    </div>
                </div>
            </div>// End Modal Delete
        }
    }

    fn msg_3(&self) -> Html{
        html!{
            <div class="modal fade" id="DeleteAccount" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true"
            >
                <div class="modal-dialog"
                >
                    <div class="modal-content"
                    >
                        <div class="modal-header" style="color:black;"
                        >
                            <h5 class="modal-tittle"><p> {"Account Delete Confirmation"} </p> </h5>
                            <button 
                                type="button"
                                class="btn-close"
                                data-bs-dismiss="modal"
                                aria-label="close"
                            >
                            </button>
                        </div>
                        <div class="modal-body" style="color:black;" >
                            <p> {"All of your remaining connector, log and webhook data will be deleted permanently. Are you sure you want to delete your entire account?"} </p>
                        </div>
                        <div class="modal-footer"
                        >
                            <button
                                type="button"
                                class="btn btn-secondary"
                                data-bs-dismiss="modal"
                                onclick=self.link.callback(|_| Msg::Ignore) 
                            >
                                {"Cancel"}
                            </button> 
                            
                            <button
                                type="button"
                                class="btn btn-danger"
                                data-bs-dismiss="modal"
                                data-bs-toggle="modal"
                                data-bs-target="#display_msg"
                                onclick=self.link.callback(|_| Msg::DeleteAccount) 
                            >
                                {"Delete"}
                            </button>
                        </div>
                    </div>
                </div>
            </div>// End Modal Delete
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
