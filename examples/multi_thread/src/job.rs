use gloo::timers::callback::Interval;
use serde::{Deserialize, Serialize};
use yew_agent::{Agent, AgentLink, HandlerId, Job};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    GetDataFromServer,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    DataFetched,
}

pub enum Msg {
    Initialized,
    Updating,
    DataFetched,
}

pub struct Worker {
    link: AgentLink<Worker>,
    _interval: Interval,
}

impl Agent for Worker {
    type Reach = Job<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        let duration = 3;

        let interval = {
            let link = link.clone();
            Interval::new(duration, move || link.send_message(Msg::Updating))
        };

        link.send_message(Msg::Initialized);
        Self {
            link,
            _interval: interval,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Initialized => {
                log::info!("Initialized!");
            }
            Msg::Updating => {
                log::info!("Tick...");
            }
            Msg::DataFetched => {
                log::info!("Data was fetched");
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        log::info!("Request: {:?}", msg);
        match msg {
            Request::GetDataFromServer => {
                // TODO fetch actual data
                self.link.respond(who, Response::DataFetched);
                self.link.send_message(Msg::DataFetched);
            }
        }
    }
}
