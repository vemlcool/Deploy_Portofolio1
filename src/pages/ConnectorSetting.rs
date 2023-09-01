use std::{vec, ops::Index};
use chrono::NaiveTime;
use gloo_storage::{SessionStorage, Storage};
use yew_router::agent::RouteRequest::ChangeRoute;
use yew::prelude::*;
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
    UserSetting,
    ProjectID,
    ProjectStat,
    MsgErr,
    Log,
};

use super::Login::Token;

#[derive(Properties, Clone)]
pub struct SetProps {
    pub _name:String,
}

pub enum Msg {
    InputName(String), //name
    InputDesc(String), //description
    InputBotTok(String), //token
    InputGroupChatID(String), //chatid
    InputDuration1(String),
    InputDuration2(String),
    IssueCreated,
    IssueUpdated,
    IssueDeleted,
    CommentCreated,
    CommentUpdated,
    CommentDeleted,
    Active_btn,
    ScheduleBtn,
    Ignore,
    RequestData,
    ResponseError(String),
    DeleteConnector,
    CopyDataSetting(UserSetting),
    Direct,
    UpdateConnector,
    GetProject,
    CopyDataProject(Vec<ProjectID>),
    TriggerProject(usize),
    UpdateValidate,
    CheckInput,
    InputErrorMsg(String),
    CheckSuccess,
    GetLog,
    CopyDataLog(Vec<Log>),
    Unauth,
    DirectLogin,
    DeleteSuccess,
    BotInfo,
    ChatInfo
}

pub struct ConnectorSetting {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    message: String,
    user_setting:UserSetting, 
    _name: String,
    project_id:Vec<ProjectID>,
    fetch_task: Option<FetchTask>,
    project_stat:Vec<ProjectStat>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    msg_err:MsgErr,
    log:Vec<Log>,
    no_login:bool,
    unauthorized:bool,
    duration1:String,
    duration2:String,
    duration1_check:bool,
    duration2_check:bool
}

impl Component for ConnectorSetting {
    type Message = Msg;
    type Properties = SetProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("this is homepage..........");
        Self {   
            message: String::from("initial message"),
            project_stat:vec![],
            project_id:vec![],
            user_setting:UserSetting { 
                name:"".to_string(), 
                description:"".to_string(), 
                schedule:false,
                duration: "".to_string(),
                token: "".to_string(), 
                chatid:"".to_string(),
                active:false,
                event:vec![],
                project:vec![],
                created_at: chrono::Utc::now()
                    .with_timezone(&chrono::FixedOffset::east_opt(7 * 3600).unwrap()),
                updated_at: None
            },
            log:vec![],
            msg_err:MsgErr { 
                header:"".to_string(),
                body:"".to_string(),
            },
            _name:props._name,
            link: link.clone(),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            fetch_task:None,
            no_login:false,
            unauthorized:false,
            duration1:"".to_string(),
            duration2:"".to_string(),
            duration1_check:true,
            duration2_check:true
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            // if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
                //     input.focus();
                // }
                
                // ConsoleService::info("this is first render homepage.....");
                self.link.send_message(Msg::RequestData);
               
            }
        }
        
        fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DeleteConnector => {
                //FETCHING...
                let key = Token{
                    access_token: String::new()
                };

                let token: String = SessionStorage::get(key.access_token).unwrap_or("".to_string());            
                let bearer = format!("Bearer {}", token);

                let request = Request::delete(format!("https://atlassian-connector-api.dev-domain.site/connector/{}", self._name))
                    .header("Authorization", bearer)
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        let status_number = meta.status.as_u16();
                        ConsoleService::info(&format!("data response {:?}", &data));

                        if meta.status.is_success(){
                            Msg::DeleteSuccess
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

            Msg::DeleteSuccess => {
                self.msg_err.header = "Delete Result".to_string();
                self.msg_err.body = "Connector has been successfully deleted".to_string();
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

            Msg::DirectLogin =>{
                ConsoleService::info(("Direct Login"));
   
                self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
                true
            }

            Msg::GetProject => {
                    let key = Token{
                        access_token: String::new()
                    };

                    let token: String = SessionStorage::get(key.access_token).unwrap_or("".to_string());            
                    let bearer = format!("Bearer {}", token);

                    let request = Request::get(format!("https://atlassian-connector-api.dev-domain.site/projects"))
                        .header("Authorization", bearer)
                        .body(Nothing)
                        .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<ProjectID>, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        let status_number = meta.status.as_u16();
                        match data {
                            Ok(dataok) => {

                                if status_number == 200 {
                                    Msg::CopyDataProject(dataok)
                                } else {
                                    Msg::ResponseError(String::from("status bukan 200"))
                                }

                            }
                            Err(error) => {
                                // ConsoleService::info("kondisi error dari server mati");
                                Msg::ResponseError(error.to_string())
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);
                true

            }
            Msg::UpdateConnector => {

                let mut final_project:Vec<ProjectID> = Vec::new();

                for x in self.project_stat.clone() {
                    if x.status {
                        final_project.push(ProjectID { 
                            id: x.id, 
                            name: x.name
                        })
                    }
                }

                let user_setting = UserSetting {
                    name: self.user_setting.name.clone(),
                    description: self.user_setting.description.clone(),

                    token: self.user_setting.token.clone(),
                    active: self.user_setting.active.clone(),
                    schedule: self.user_setting.schedule.clone(),
                    duration: self.user_setting.duration.clone(),

                    chatid: self.user_setting.chatid.clone(),
                    event: self.user_setting.event.clone(),
                    project: final_project,
                    created_at: self.user_setting.created_at.clone(),
                    updated_at: self.user_setting.updated_at.clone()
                };

                // Fetching
                let key = Token{
                    access_token: String::new()
                };
    
                let token: String = SessionStorage::get(key.access_token).unwrap_or("".to_string());            
                let bearer = format!("Bearer {}", token);
                
                let request = Request::put(format!("https://atlassian-connector-api.dev-domain.site/connector/{}", self._name))
                    .header("Authorization", bearer)
                    .header("Content-Type", "application/json")
                    .body(Json(&user_setting))
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        let status_number = meta.status.as_u16();
                        ConsoleService::info(&format!("data response {:?}", &data));

                        if meta.status.is_success(){
                            Msg::CheckInput
                        }else{
                            match data {
                                Ok(dataok) => {
                                    ConsoleService::info(&format!("data response {:?}", &dataok));
                                    Msg::InputErrorMsg(dataok)
                                    // Msg::Direct;
                                }
                                Err(error) => {
                                    ConsoleService::info("ignore.");
                                    Msg::ResponseError(error.to_string())
                                }
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);
                true
            }


            Msg::RequestData => {
                //FETCHING...
                let key = Token{
                    access_token: String::new()
                };
    
                let token: String = SessionStorage::get(key.access_token).unwrap_or("".to_string());            
                let bearer = format!("Bearer {}", token);

                let request = Request::get(format!("https://atlassian-connector-api.dev-domain.site/connector/{}", self._name))
                    .header("Authorization", bearer)
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<UserSetting, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        let status_number = meta.status.as_u16();
                        ConsoleService::info(&format!("data response {:?}", &data));
                        

                        match data {
                            Ok(dataok) => {

                                if status_number == 200 {
                                    Msg::CopyDataSetting(dataok)
                                } else {
                                    Msg::ResponseError(String::from("status bukan 200"))
                                }

                            }
                            Err(error) => {
                                // ConsoleService::info("kondisi error dari server mati");
                                if status_number == 401 {
                                    Msg::Unauth
                                } else {
                                    Msg::ResponseError(error.to_string())
                                }
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);
                true
            }
            Msg::GetLog => {
                //FETCHING...
                let key = Token{
                    access_token: String::new()
                };
    
                let token: String = SessionStorage::get(key.access_token).unwrap_or("".to_string());            
                let bearer = format!("Bearer {}", token);

                let request = Request::get(format!("https://atlassian-connector-api.dev-domain.site/log/{}", self._name))
                    .header("Authorization", bearer)
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<Log>, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        let status_number = meta.status.as_u16();
                        ConsoleService::info(&format!("data response {:?}", &data));
                        
                        match data {
                            Ok(dataok) => {

                                if status_number == 200 {
                                    Msg::CopyDataLog(dataok)
                                } else {
                                    Msg::ResponseError(String::from("status bukan 200"))
                                }

                            }
                            Err(error) => {
                                // ConsoleService::info("kondisi error dari server mati");
                                Msg::ResponseError(error.to_string())
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);
                true
            }

            Msg::InputName(name)=>{
                self.user_setting.name = name;
                ConsoleService::info(&format!("ConnectorName adalah: {:?}", self.user_setting));
                true
            }
            Msg::InputDesc(description)=>{
                self.user_setting.description = description;
                ConsoleService::info(&format!("Description adalah: {:?}", self.user_setting));
                true
            }
            Msg::InputBotTok(token)=>{
                self.user_setting.token = token;
                ConsoleService::info(&format!("Bot Token adalah: {:?}", self.user_setting));
                true
            }
            Msg::InputDuration1(duration)=>{
                self.duration1 = duration;
                if !self.duration1.is_empty(){
                    match NaiveTime::parse_from_str(&self.duration1, "%H:%M") {
                        Ok(_) => self.duration1_check = true,  
                        Err(_) => self.duration1_check = false, 
                    }
                }else{

                }
                true
            }
            Msg::InputDuration2(duration)=>{
                self.duration2 = duration;
                if !self.duration2.is_empty(){
                    match NaiveTime::parse_from_str(&self.duration2, "%H:%M") {
                        Ok(_) => self.duration2_check = true,  
                        Err(_) =>self.duration2_check = false, 
                    }
                }else{

                }
                true
            }
            Msg::InputGroupChatID(chatid)=>{
                self.user_setting.chatid = chatid;
                ConsoleService::info(&format!("Group Chat ID adalah: {:?}", self.user_setting));
                true
            }

            Msg::Ignore=>{
                true
            }
            Msg::Direct=> {
                ConsoleService::info(("Direct Jalan"));

                self.router_agent.send(ChangeRoute(AppRoute::ConnectorHome.into()));
                true
            }
            Msg::IssueCreated=>{
                ConsoleService::info("Issue Created....");
                match self.user_setting.event.iter().position(|i| i=="jira:issue_created"){  

                    Some(x) => { 
                        self.user_setting.event.remove(x);
                        
                    },
                    None    => self.user_setting.event.push("jira:issue_created".to_string()),
                }    
                ConsoleService::info(&format!("Issue Created adalah {:?}", self.user_setting));   
                true
            }
            Msg::IssueUpdated=>{
                ConsoleService::info("Issue Updated....");
                match self.user_setting.event.iter().position(|i| i=="jira:issue_updated"){  

                    Some(x) => { 
                        self.user_setting.event.remove(x);
                        
                    },
                    None    => self.user_setting.event.push("jira:issue_updated".to_string()),
                }    
                ConsoleService::info(&format!("Issue Updated adalah {:?}", self.user_setting));   
                true
            }
            Msg::IssueDeleted=>{
                ConsoleService::info("Issue Deleted....");
                match self.user_setting.event.iter().position(|i| i=="jira:issue_deleted"){  

                    Some(x) => { 
                        self.user_setting.event.remove(x);
                        
                    },
                    None    => self.user_setting.event.push("jira:issue_deleted".to_string()),
                }    
                ConsoleService::info(&format!("Issue Deleted adalah {:?}", self.user_setting));   
                true
            }
            Msg::CommentCreated=>{
                ConsoleService::info("Comment Created....");
                match self.user_setting.event.iter().position(|i| i=="comment_created"){  

                    Some(x) => { 
                        self.user_setting.event.remove(x);
                        
                    },
                    None    => self.user_setting.event.push("comment_created".to_string()),
                }    
                ConsoleService::info(&format!("Comment Created adalah {:?}", self.user_setting));   
                true
            }
            Msg::CommentDeleted=>{
                ConsoleService::info("Comment Deleted....");
                match self.user_setting.event.iter().position(|i| i=="comment_deleted"){  

                    Some(x) => { 
                        self.user_setting.event.remove(x);
                        
                    },
                    None    => self.user_setting.event.push("comment_deleted".to_string()),
                }    
                ConsoleService::info(&format!("Comment Deleted adalah {:?}", self.user_setting));   
                true
            }
            Msg::CommentUpdated=>{
                ConsoleService::info("Comment Updated....");
                match self.user_setting.event.iter().position(|i| i=="comment_updated"){  

                    Some(x) => { 
                        self.user_setting.event.remove(x);
                        
                    },
                    None    => self.user_setting.event.push("comment_updated".to_string()),
                }    
                ConsoleService::info(&format!("Comment Updated adalah {:?}", self.user_setting));   
                true
            }
            Msg::Active_btn=>{
                ConsoleService::info("Active ....");
                if self.user_setting.active == true{  
                    self.user_setting.active = false;
                    ConsoleService::info(&format!("1 adalah {:?}", self.user_setting.active));   
                }else {
                    self.user_setting.active = true;
                    ConsoleService::info(&format!("3 adalah {:?}", self.user_setting.active));
                }

                ConsoleService::info(&format!("Active adalah {:?}", self.user_setting));   
                true
            }
            Msg::ScheduleBtn=>{
                ConsoleService::info("On ....");
                if self.user_setting.schedule == true{  
                    self.user_setting.schedule = false;
                    ConsoleService::info(&format!("1 adalah {:?}", self.user_setting.schedule));   
                }else {
                    self.user_setting.schedule = true;
                    ConsoleService::info(&format!("3 adalah {:?}", self.user_setting.schedule));
                }

                ConsoleService::info(&format!("Schedule adalah {:?}", self.user_setting));   
                true
            }
            Msg::ResponseError(text) => {
                ConsoleService::info(&format!("error is {:?}", text));
                true
            }
            Msg::InputErrorMsg(dataok) => {
                self.msg_err.header = "Error".to_string();
                self.msg_err.body = dataok;
                true
            }
            Msg::CheckInput => {
                if self.msg_err.body.is_empty(){
                    self.msg_err.header = "Success".to_string();
                    self.msg_err.body = "You have successfuly updated your connector".to_string();
                }else{
                    self.link.send_message(Msg::Ignore);
                }
                true
            }
            Msg::UpdateValidate => {
                if self.user_setting.name.is_empty(){
                    self.msg_err.header = "Error".to_string();
                    self.msg_err.body = "Name field cannot be empty".to_string();
                }else{
                    if self.user_setting.chatid.is_empty(){
                        self.msg_err.header = "Error".to_string();
                        self.msg_err.body = "Chat ID field cannot be empty".to_string();
                    }else {
                        if self.user_setting.token.is_empty(){
                            self.msg_err.header = "Error".to_string();
                            self.msg_err.body = "Bot Token field cannot be empty".to_string();
                        }else {
                            if self.user_setting.event.is_empty(){
                                self.msg_err.header = "Error".to_string();
                                self.msg_err.body = "Event field cannot be empty".to_string();
                            }else {    
                                if !self.duration1.is_empty() && self.duration1_check == false {
                                    self.msg_err.header = "Error".to_string();
                                    self.msg_err.body = "From Duration must be a valid time format".to_string();
                                } else{
                                    if !self.duration2.is_empty() && self.duration2_check == false {
                                        self.msg_err.header = "Error".to_string();
                                        self.msg_err.body = "To Duration must be a valid time format".to_string();
                                    } else {
                                        if (self.duration1.is_empty() && !self.duration2.is_empty()) || (!self.duration1.is_empty() && self.duration2.is_empty()){
                                            self.msg_err.header = "Error".to_string();
                                            self.msg_err.body = "Duration must be either both filled or both emptied".to_string();
                                        } else{
                                            if (!self.duration1.is_empty() && !self.duration2.is_empty()) && (self.duration1 == self.duration2)  {
                                                self.msg_err.header = "Error".to_string();
                                                self.msg_err.body = "Duration must not be the same".to_string();
                                            } else{
                                                if self.user_setting.schedule == true && (self.duration1.is_empty()||self.duration2.is_empty()){
                                                    self.msg_err.header = "Error".to_string();
                                                    self.msg_err.body = "Schedule cannot be turned on if duration is not filled".to_string();
                                                } else {
                                                    if !self.duration1.is_empty() && !self.duration2.is_empty(){
                                                        self.user_setting.duration = format!("{}-{}", self.duration1, self.duration2);
                                                        self.msg_err.body = "".to_string();
                                                        self.link.send_message(Msg::UpdateConnector);
                                                    } else {
                                                        self.user_setting.duration = "".to_string();
                                                        self.msg_err.body = "".to_string();
                                                        self.link.send_message(Msg::UpdateConnector);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                      
                            }
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
            Msg::TriggerProject(index)=>{
                ConsoleService::info("TriggerProject....");
                
                if self.project_stat.get(index).unwrap().status {
                    self.project_stat.get_mut(index).unwrap().status = false
                }else{
                    self.project_stat.get_mut(index).unwrap().status = true
                }
                ConsoleService::info(&format!("Issue Created adalah {:?}", self.user_setting));   
                true
            }
            Msg::CopyDataSetting (data) => {
                self.user_setting = data;
                self.link.send_message(Msg::GetProject); //sus

                if !self.user_setting.duration.is_empty(){
                    let split: Vec<&str> = self.user_setting.duration.split('-').collect();
                    self.duration1 = split[0].to_string();
                    self.duration2 = split[1].to_string();
                    ConsoleService::info(&format!("Duration split {:?}[]{:?}", self.duration1, self.duration2));   
                } else{
                    
                }
                
                true
            }
            Msg::CopyDataLog(data) => {
                self.log = data;
                true
            }
            
            Msg::CopyDataProject (data) => {
                ConsoleService::info(&format!("All Project {:?}", &data));
                ConsoleService::info(&format!("Project Stat {:?}", self.project_stat));
                let id_project:Vec<String> = self.user_setting.project.iter().map(|id_|id_.id.clone()).collect();
                
                if self.project_stat.is_empty() {
                    ConsoleService::info(&format!("id project {:?}", &id_project));
                    if id_project.is_empty() {
                        ConsoleService::info(&format!("masuk empty"));
                        self.project_stat = data.iter().map(|project|{
                            ProjectStat {
                                id: project.id.clone(),
                                name: project.name.clone(),
                                status: false,
                            }  
                        }).collect();
                    }else {
                        self.project_stat = data.iter().map(|project|{
                            if id_project.contains(&project.id){
                                ConsoleService::info(&format!("masuk else 1"));
                                ProjectStat {
                                    id: project.id.clone(),
                                    name: project.name.clone(),
                                    status: true,
                                }   
                            }else{
                                ConsoleService::info(&format!("masuk else 2"));
                                ProjectStat {
                                    id: project.id.clone(),
                                    name: project.name.clone(),
                                    status: false,
                                }  
                            }
                        }).collect();
                    }
                }else{
                    
                }
                
                ConsoleService::info(&format!("Selected Project {:?}", self.user_setting.project));
                ConsoleService::info(&format!("Project Stat {:?}", self.project_stat));
                true
            }
            Msg::BotInfo => {
                self.msg_err.header = "Telegram Bot".to_string();
                self.msg_err.body = "You can use your own Bot Token or our Bot by inviting @TelConnectBot to your chat and copy pasting this token \"5855949980:AAGA2pn4k68IdPzGpQHDZ2J67e5Biu0491k\" (without the \"\") to the bot token field\n\n (Make sure the bot is invited to the chat and has permission to send messages)".to_string();
                true
            }
            Msg::ChatInfo => {
                self.msg_err.header = "Chat ID".to_string();
                self.msg_err.body = "Open your Telegram chat by using Telegram Web and copy paste the number in the url (Ex. for \"https://web.telegram.org/k/#-865703519\", -865703519 is the Chat ID)".to_string();
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

        let mut setpoint=0;

        //Issue Created
        let mut event_issue_created= false;
        if self.user_setting.event.clone().iter().any(|i| i=="jira:issue_created"){
            event_issue_created=true;
        }
        ConsoleService::info(&format!("Event_issue_created: {:?}", event_issue_created));
        
        //Issue Updated
        let mut event_issue_updated= false;
        if self.user_setting.event.clone().iter().any(|i| i=="jira:issue_updated"){
            event_issue_updated=true;
        }
        ConsoleService::info(&format!("Event_issue_updated: {:?}", event_issue_updated));

        //Issue Deleted
        let mut event_issue_deleted= false;
        if self.user_setting.event.clone().iter().any(|i| i=="jira:issue_deleted"){
            event_issue_deleted=true;
        }
        ConsoleService::info(&format!("Event_issue_deleted: {:?}", event_issue_deleted));

        //Comment Created
        let mut event_comment_created= false;
        if self.user_setting.event.clone().iter().any(|i| i=="comment_created"){
            event_comment_created=true;
        }
        ConsoleService::info(&format!("Event_comment_created: {:?}", event_comment_created));

        //Comment Updated
        let mut event_comment_updated= false;
        if self.user_setting.event.clone().iter().any(|i| i=="comment_updated"){
            event_comment_updated=true;
        }
        ConsoleService::info(&format!("Event_comment_updated: {:?}", event_comment_updated));

        //Comment Deleted
        let mut event_comment_deleted= false;
        if self.user_setting.event.clone().iter().any(|i| i=="comment_deleted"){
            event_comment_deleted=true;
        }
        ConsoleService::info(&format!("Event_comment_deleted: {:?}", event_comment_deleted));

        //button
        let mut event_button: bool = false;
        event_button = self.user_setting.active;

        let mut schedule_button: bool = false;
        schedule_button = self.user_setting.schedule;

        let created_at = self.user_setting.created_at;
        let updated_at = self.user_setting.updated_at;

        // if self.user_setting.active == true.to_string() {
        //     event_button=true;
        // }else if self.user_setting.active == false.to_string() {
        //     event_button=false;
            
        // }
        ConsoleService::info(&format!("Event_button: {:?}", event_button));
        
        if self.no_login == true || self.unauthorized == true {
            html!{self.view_no_login()}
        } else {
            html! {
                <div>
                    <div class="base-form2">
                        <div class="create-connector" style= "width: 500px"> 
                            <h5>{"Connector Data"}</h5>

                            <div class="input-group" style=" margin: auto; width: 400px">
                                <input type="text" id="" class="form-control p-3 my-2" placeholder="Connector Name"
                                    style="
                                        width: 400px;
                                        height:30px;
                                        margin:auto;
                                    "
                                    oninput=self.link.callback(|data: InputData| Msg::InputName(data.value))
                                    value={self.user_setting.name.clone()}
                                />
                            </div>

                            <div class="input-group" style=" margin: auto; width: 400px">
                                <input type="text" id="" class="form-control p-3 my-2" placeholder="Description"
                                    style="
                                        width: 400px;
                                        height:30px;
                                        margin:auto;
                                    "
                                    value={self.user_setting.description.clone()}
                                    oninput=self.link.callback(|data: InputData| Msg::InputDesc(data.value))
                                />
                            </div>
         
                            // Project button
                            <div class="info"
                            >
                                <h5 style="padding-top:15px">{"Project to Notify"}</h5>
                                
                                <button
                                    type="button"
                                    class="btn btn-primary"
                                    data-bs-toggle="modal"
                                    data-bs-target="#exampleModal"
                                    style="margin:auto; width:400px; background:#A73034; margin-bottom:10px; border-color: gray; color:white;"
                                    onclick=self.link.callback(|_| Msg::GetProject)
                                >
                                    {"Select Project"}
                                </button>
                            </div>

                            <h5 style="padding-top:15px">{"Bot Setting"}</h5>
                                    
                            // Logic Select --> Groupchatid / Bot token
                            
                            <div>
                                <div  class="input-group" style=" margin: auto; width: 400px">
                                    <input type="text" id="Bot_Tok" class="form-control p-3 my-2 " placeholder="Bot Token"
                                        style="
                                            height:30px;
                                            margin:auto;
                                        "
                                        value={self.user_setting.token.clone()}
                                        oninput=self.link.callback(|data: InputData| Msg::InputBotTok(data.value))
                                    />
                                    <button 
                                        type="button"
                                        style="
                                        height:30px;
                                        margin:auto;
                                        "
        
                                        data-bs-toggle="modal"
                                        data-bs-target="#display_msg"
                                        onclick=self.link.callback(|_| Msg::BotInfo)
                                        >
        
                                        {"?"}
                                    </button>

                                </div>

                                <div  class="input-group" style=" margin: auto; width: 400px">
                                    <input type="text" id="Group_ID" class="form-control p-3 my-2 " placeholder="Chat ID"
                                        style="
                                            height:30px;
                                            margin:auto;
                                        "
                                        value={self.user_setting.chatid.clone()}
                                    oninput=self.link.callback(|data: InputData| Msg::InputGroupChatID(data.value))
                                    />

                                    <button 
                                        type="button"
                                        style="
                                        height:30px;
                                        margin:auto;
                                        "
        
                                        data-bs-toggle="modal"
                                        data-bs-target="#display_msg"
                                        onclick=self.link.callback(|_| Msg::ChatInfo)
                                        >
        
                                        {"?"}
                                    </button>
                                </div>

                            </div>
                            // Close logic select

                            <h5 style="padding-top:15px">{"Schedule"}</h5>
                            {//ON OFF BUTTON
                                if schedule_button == true{
                                    html!{
                                        <button type="button" class="btn btn-primary"
                                                onclick=self.link.callback(|_| Msg::ScheduleBtn)
                                                >
                                            {"On"}
                                            </button>
                                        }
                                }else{
                                    html!{
                                        <button type="button" class="btn btn-secondary"
                                            onclick=self.link.callback(|_| Msg::ScheduleBtn)
                                            >
                                            {"Off"}
                                            </button>
                                        }
                                    }
                            }// END ON OFF BUTTON

                            {
                                if self.user_setting.schedule == true {
                                    html!{
                                        <div class="input-group" style=" margin: auto;">
                                            <h7  style="margin:auto;"> {"From"} </h7>
                                            <input type="text" id="" class="form-control p-3 my-2 " placeholder="(Ex. 08:00)"
                                                style="
                                                    width: 10px;
                                                    height:30px;
                                                    margin:auto;
                                                    margin-left:10px;
                                                    margin-right:10px
                                                "
                                                value={self.duration1.clone()}
                                                oninput=self.link.callback(|data: InputData| Msg::InputDuration1(data.value))
                                            />
                                            <h7  style="margin:auto;"> {"to"} </h7>
                                            <input type="text" id="" class="form-control p-3 my-2 " placeholder="(Ex. 17:30)"
                                                style="
                                                    width: 10px;
                                                    height:30px;
                                                    margin:auto;
                                                    margin-left:10px;
                                                    margin-right:10px
                                                "
                                                value={self.duration2.clone()}
                                                oninput=self.link.callback(|data: InputData| Msg::InputDuration2(data.value))
                                            />
                                        </div>
                                    }
                                } else {
                                    html!{

                                    }
                                }
                            }

                            //Check Box
                            <div>
                                <div>
                                    <div style="text-align:center;">
                                        <h5 style="padding-top:15px">{"Event to Notify"}</h5>
                                    </div>

                                    <div class="check-box"
                                            style="
                                            color:black;
                                            margin:auto;
                                            text-align:center;
                                            padding-top:5px;
                                            "
                                    >   
                                        <div class="form-check mb-3" style="margin: auto; width:400px;">
                                            <input class="form-check-input" type="checkbox" value="issuescreated" id="flexCheckDefault"
                                            checked={event_issue_created} 
                                            onclick=self.link.callback(|_| Msg::IssueCreated)
                                            />
                                                <label class="form-check-label" for="flexCheckDefault">{"Issues Created"}</label>
                                        </div>

                                        <div class="form-check mb-3" style="margin: auto; width:400px;">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"
                                            checked={event_issue_updated} 
                                            onclick=self.link.callback(|_| Msg::IssueUpdated)
                                            />
                                                <label class="form-check-label" for="flexCheckDefault">{"Issues Updated"}</label>
                                        </div>

                                        <div class="form-check mb-3" style="margin: auto; width:400px;">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"
                                            checked={event_issue_deleted} 
                                            onclick=self.link.callback(|_| Msg::IssueDeleted)  
                                            />
                                                <label class="form-check-label" for="flexCheckDefault">{"Issues Deleted"}</label>
                                        </div>
                                        <div class="form-check mb-3" style="margin: auto; width:400px;">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"
                                            checked={event_comment_created} 
                                            onclick=self.link.callback(|_| Msg::CommentCreated)   
                                            />
                                                <label class="form-check-label" for="flexCheckDefault">{"Comment Created"}</label>
                                        </div>
                                        <div class="form-check mb-3" style="margin: auto; width:400px;">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"
                                            checked={event_comment_updated} 
                                            onclick=self.link.callback(|_| Msg::CommentUpdated)   
                                            />
                                                <label class="form-check-label" for="flexCheckDefault">{"Comment Updated"}</label>
                                        </div>
                                        <div class="form-check mb-3" style="margin: auto; width:400px;">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"
                                            checked={event_comment_deleted} 
                                            onclick=self.link.callback(|_| Msg::CommentDeleted) 
                                            />
                                                <label class="form-check-label" for="flexCheckDefault">{"Comment Deleted"}</label>
                                        </div> 
                                    </div>
                                </div>
                            </div> // Close Check Box
                            
                            //Flex Button
                            <div style="display:flex; justify-content: space-around; padding-top:15px;"
                            >  
                                // Button Delete
                                <button type="button" class="btn btn-danger"
                                    data-bs-toggle="modal"
                                    data-bs-target="#DeleteModal"
                                >
                                    {"Delete"}
                                </button>

                                {//ON OFF BUTTON
                                if event_button == true{
                                    html!{
                                        <button type="button" class="btn btn-primary"
                                                onclick=self.link.callback(|_| Msg::Active_btn)
                                                >
                                            {"Active"}
                                            </button>
                                        }
                                }else{
                                    html!{
                                        <button type="button" class="btn btn-dark"
                                            onclick=self.link.callback(|_| Msg::Active_btn)
                                            >
                                            {"Deactive"}
                                            </button>
                                        }
                                    }
                                }// END ON OFF BUTTON

                                    // Button Log
                                    <button type="button" class="btn btn-info" style="color:white"
                                        data-bs-toggle="modal"
                                        data-bs-target="#ViewLog"
                                        onclick=self.link.callback(|_| Msg::GetLog)
                                    >
                                    {"View Log"}
                                </button>

                                // Button Save changes
                                <button type="button" class="btn btn-success"
                                    data-bs-toggle="modal"
                                    data-bs-target="#display_msg"
                                    onclick=self.link.callback(|_| Msg::UpdateValidate)
                                >
                                    {"Save"}
                                </button>
                            </div>

                            <Anchor route=AppRoute::ConnectorHome>
                                <button type="button" class="btn btn-secondary" style="margin-top:20px">
                                    {"Cancel"}
                                </button>
                            </Anchor>
                            
                        </div>   
                    </div>
                    
                    //Modal Project List
                    <div 
                        class="modal fade"
                        id="exampleModal"
                        tabindex="-1"
                        aria-labelledby="exampleModalLabel"
                        aria-hidden="true"
                    >
                        <div class="modal-dialog modal-dialog-scrollable"
                        >
                            <div class="modal-content"
                            >
                                <div class="modal-header"
                                >
                                    <h5 class="modal-tittle">
                                        {
                                            if self.project_stat.is_empty(){
                                                {"Error !"} 
                                            }else{
                                                {"Project List"} 
                                            }
                                        }
                                    </h5>

                                    <button 
                                        type="button"
                                        class="btn-close"
                                        data-bs-dismiss="modal"
                                        aria-label="close"
                                    >
                                    </button>
                                </div>
                            
                                <div class="modal-body" style=" text-align:center; margin:auto; "
                                >
                                    {                                                
                                        if self.project_stat.is_empty(){
                                            html! {
                                                {"Something went wrong, Please check your Email and API-Key or make sure you have created project in your Jira account"}
                                                
                                            }
                                            
                                        }else{
                                            self.project_stat.iter().enumerate().map(|(index,i)| {
                                                    html! {
                                                        <div class="form-check mb-3" style=" width:400px;"
                                                        >
                                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"
                                                            checked={i.status}
                                                            onclick=self.link.callback(move |_| Msg::TriggerProject(index))
                                                            />
                                                            <label class="form-check-label" for="flexCheckDefault"> {&i.name} </label>
                                                        </div>
                                                    }
                                            }).collect::<Html>() 
                                        }
                                    }
                                </div>
                                <div class="modal-footer"
                                >
                                    {
                                        if self.project_stat.is_empty(){
                                            html!{
                                                <button
                                                    type="button"
                                                    class="btn btn-primary"
                                                    data-bs-dismiss="modal"
                                                >
                                                    {"Close"}
                                                </button> 
                                            }
                                        }else{
                                            html!{
                                                <>
                                                    <button
                                                        type="button"
                                                        class="btn btn-primary"
                                                        data-bs-dismiss="modal"
                                                    >
                                                        {"Close"}
                                                    </button> 
                                                </>
                                            }
                                        }
                                    }
                                </div>
                            </div>
                        </div>
                    </div> // End Modal Project List

                    // Modal Delete           
                    <div class="modal fade" id="DeleteModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true"
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
                                    <p> {"Are you sure you want to delete this connector?"} </p>
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
                                        data-bs-target="#DeleteResult"
                                        onclick=self.link.callback(|_| Msg::DeleteConnector) 
                                    >
                                        {"Delete"}
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>// End Modal Delete

                    // Modal Delete Result           
                    <div class="modal fade" id="DeleteResult" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true"
                      >
                          <div class="modal-dialog"
                          >
                              <div class="modal-content"
                              >
                                  <div class="modal-header" style="color:black;"
                                  >
                                      <h5 class="modal-tittle" style="font-family: Alexandria; color: #A73034;"><p> {format!("{} !",self.msg_err.header)} </p> </h5>
                                      <button 
                                          type="button"
                                          class="btn-close"
                                          data-bs-dismiss="modal"
                                          aria-label="close"
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
                                          data-bs-dismiss="modal"
                                          onclick=self.link.callback(|_| Msg::Direct) 
                                      >
                                          {"Close"}
                                      </button> 
                                  </div>
                              </div>
                          </div>
                      </div>// End Modal Delete Result

                    //modal view log
                    <div class="modal fade" id="ViewLog" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true"
                    >
                        <div class="modal-dialog"
                        >
                            <div class="modal-content"
                            >
                                <div class="modal-header" style="color:black;"
                                >
                                    <h5 class="modal-tittle"><p> {"Log List"} </p> </h5>
                                    <button 
                                        type="button"
                                        class="btn-close"
                                        data-bs-dismiss="modal"
                                        aria-label="close"
                                    >
                                    </button>
                                </div>
                                <div class="modal-body" style="color:black;" >
                                    {    
                                    if self.log.is_empty(){
                                    html!{
                                        <p> {"This connector doesn't have any log yet"} </p>
                                    }
                                    }else {
                                        self.log.iter().rev().map(|card|{
                                            ConsoleService::info(&format!("Name adalah {:?}",card.event.to_string()));
                                                html!{
                                                        <div class="card mt-4 mb-2"
                                                            style="
                                                                text-decoration:none;
                                                                background: white;
                                                                width:300px;
                                                                margin:auto;
                                                                border:none;
                                                            "
                                                        >
                                                        
                                                            <div class="card-body" style="color: gray;">
                                                                <p class="card-title" 
                                                                    style="color: black;"
                                                                >
                                                                    {&card.event} <br/>
                                                                    {&card.status} <br/>
                                                                    {format!("attempt: {}",&card.attempt)} <br/>
                                                                    {&card.time} <br/>
                                                                </p>
                                                            </div>
                                                        
                                                        </div>
                                                    
                                                }
                                            
                                        }).collect::<Html>()       
                                    }
                                    


                                }
                                </div>
                                <div class="modal-footer"
                                >
                                    <button
                                        type="button"
                                        class="btn btn-primary"
                                        data-bs-dismiss="modal"
                                        onclick=self.link.callback(|_| Msg::Ignore) 
                                    >
                                        {"Close"}
                                    </button> 
                                </div>
                            </div>
                        </div>
                    </div>// End Viewlog


                    //Modal Error Msg
                    {self.msg_1()}

                </div>
            }
        }    
    }
}

impl ConnectorSetting{
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
                                <h5 class="modal-tittle"> <p> {format!("{}!",self.msg_err.header)} </p> </h5>
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
