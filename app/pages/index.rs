
use binuid_std::{
    component,
    events::UIEvent,
    components::{Node, Props, Cmd, PropValue}, 
    tags::{div, text}
};


#[derive(Debug, Default, Clone)]
pub struct AppModel {
    pub lhs: u32,
    pub rhs: u32
}

#[derive(Debug, Default, Clone)]
pub enum AppMsg {
    #[default]
    None,
    Value(i8)
}


impl From<()> for AppModel {
    fn from(_value: ()) -> AppModel {
        AppModel::default()
    }
}

impl From<UIEvent> for AppMsg {
    fn from(_value: UIEvent) -> AppMsg {
        AppMsg::default()
    }
}

pub fn view(
    _model: &AppModel,
    _props: &[(&'static str, PropValue::<AppMsg>)],
    _children: &[Node<AppModel, AppMsg>]
) -> Node<AppModel, AppMsg> {

    Node::<AppModel, AppMsg>::default()
}


pub fn update(_model: &mut AppModel, _props: &mut Props<AppMsg>, _msg: &AppMsg) -> Cmd<AppMsg> {

    Cmd::None
}

component!(App, app, AppModel, AppMsg, Some(view), Some(update), None, None);
