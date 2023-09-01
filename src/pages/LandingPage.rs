use yew::{
    prelude::*,
};


pub enum Msg {
}

pub struct LandingPage {
}

impl Component for LandingPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {

        Self {
        }
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
        html! {
            <div class="base-landing">
                <div class="landing-page">
                    <div class="container">
                        <div class="header-area">
                            <div class="logo"><b>{"Telkom "}</b>{"Indonesia"}</div>
                        </div>
                        <div class="info">
                            <h1>{"Digital Business Management"}</h1>
                                <p>{"We provide you services to make effiency in life"}</p> 
                        </div>
                        <div class="image">
                            <img src="img/landingpage.png" width="600px" height="600px"/>
                                
                        </div>
                        
                        <div class="clearfix"></div>
                        
                    </div>
                    <div class="services">
                        <h4><b>{"TelConnect"}</b></h4>
                            <p>{"TelConnect is an application that can channel notifications from Jira to popular social media application such as Telegram.
                            The distribution of these notifications can reduce the need to continuously connect to Jira through a web browser and also act as a means for users to communicate directly
                            "}
                            </p>
                    </div>
                </div>
            </div>
        }
    }
}
