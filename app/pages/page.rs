use std::{
    logs::log_std,
    web::{
        subscribe::Sub, 
        command::Cmd,
        component::Component
    },
    common::{
        default::Default,
        from::From
    },
    web::events::event::Event
};
use app::components::{
    cards::{Card, CardModel, CardMsg},
    buttons::{Btn, BtnModel, BtnMsg}
};


pub struct AppModel {
    pub card: CardModel,
    pub increase: BtnModel,
    pub decrease: BtnModel
}

pub enum AppMsg {
    None, 
    Increase(BtnMsg),
    Decrease(BtnMsg),
    Card(CardMsg)
}

pub struct AppProps;

pub struct App;

impl Component for App {
    type Model: AppModel;
    type Msg: AppMsg;
    type Props: AppProps;

    fn view(model: Self::Model, props: Self::Props) -> Html {
        html!(
            <div>
                <div>
                    <Btn 
                        model={model.increase}
                        onclick={|event: Event| AppMsg::Increase(BtnMsg::OnClick)}
                    />
                    <Btn 
                        model={model.decrease}
                        onclick={|event: Event| AppMsg::Decrease(BtnMsg::OnClick)}
                    />
                </div>
                <Card
                    model={model.card}
                />
            </div>
        )
    }
    
    fn update(model: Self::Model, msg: Self::MSG) -> Cmd<Self::Model, Self::MSG> {
        match msg {
            AppMsg::Increase(increase_msg) => {
                Cmd::<Self::Model, Self::MSG>::from(Btn::update(model.increase, increase_msg))
            },
            AppMsg::Decrease(decrease_msg) => {
                Cmd::<Self::Model, Self::MSG>::from(Btn::update(model.decrease, decrease_msg))
            },
            AppMsg::Card(card_msg) => {
                Cmd::<Self::Model, Self::MSG>::from(Card::update(model.card, card_msg))
            },
            _ => Cmd::None,
        }
    }

    fn subscribe(model: Self::Model) -> Sub<MSG> {

        Sub::None
    }
}