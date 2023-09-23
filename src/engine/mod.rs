use duid_app::duid_core::{
    console::info,
    core::router::Router
};
use std::rc::Rc;

pub struct BinuidEngine {
    pub router: Router
}

impl BinuidEngine {

    pub fn start() -> BinuidEngine {
        info!("Bravo Djedou, it works from wasm!!!!!");

        let duid_engine = BinuidEngine {
            router: Router::new()
        };

        duid_engine.render_route(None);
        duid_engine
    }

    fn render_route(&self, route: Option<Rc<&'static str>>) {
        match route {
            Some(r) => {
                self.load_user_app_by_route(&r);
            },
            None => {
                self.load_user_app_by_route("/.");
            }
        }
    }

    fn load_user_app_by_route(&self, route: &'static str) {
        //duid_app::duid_core::user_app!(route);
    }
}

