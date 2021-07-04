use yew::prelude::*;
use yew_router::prelude::*;
use yew::services::ConsoleService;
use std::time::Duration;
use wasm_cookies;
use wasm_cookies::CookieOptions;

mod locales;
use locales::config;

mod pages;
use pages::{
    index::Index, blog::Blog,
};

mod components;
use components::{
    header::Header, footer::Footer,
};



enum Msg {
    LocaleSwitch(String),
    RouteChanged(Route<()>),
}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/blog"]
    Blog,
    #[to = "/"]
    Index,
}

struct Model {
    link: ComponentLink<Self>,
    route_service: RouteService<()>,
    route: Route<()>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    locale: config::Locale,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
       ConsoleService::info(&*format!("Debug mode: {}", true));

        if wasm_cookies::get_raw("language").is_none() {
            let cookies_options = CookieOptions::default();
            wasm_cookies::set("language", "english", &cookies_options.expires_after( Duration::from_secs(31536000)));
        }

        let route_service: RouteService<()> = RouteService::new();
        let route = route_service.get_route();
        let router_agent = RouteAgent::bridge(link.callback(Msg::RouteChanged));


        let locale = config::Locale::get(&wasm_cookies::get_raw("language").unwrap());

        Self {
            link,
            route_service,
            route,
            router_agent,
            locale,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LocaleSwitch(lang) => {
                let cookies_options = CookieOptions::default();
                wasm_cookies::set("language", &*lang, &cookies_options.expires_after( Duration::from_secs(31536000)));
                self.locale = config::Locale::get(&wasm_cookies::get_raw("language").unwrap());
            }

            Msg::RouteChanged(route) => {
                self.route_service.set_route(&route.route, ());
                self.route = route
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            { self.view_nav() }

            <Header title="PROPS FROM MAIN COMPONENT" />

            { self.route_switch() }

            <Footer/>
            </>
        }
    }
}

impl Model {
    fn route_switch(&self) -> Html {
        match AppRoute::switch(self.route.clone()) {
            Some(AppRoute::Index) => html! {<Index/>},
            Some(AppRoute::Blog) => html! {<Blog/>},
            None => html! {}
        }
    }

    fn view_nav(&self) -> Html {
        html! {
            <nav>
                <ul class="nav">
                    <li>
                        <RouterAnchor<AppRoute> route=AppRoute::Index>{self.locale.main()}</RouterAnchor<AppRoute>>
                    </li>
                    <li>
                        <RouterAnchor<AppRoute> route=AppRoute::Blog>{self.locale.blog()}</RouterAnchor<AppRoute>>
                    </li>
                    <li class="dropdown">
                        <span><img src="images/lang.png"/></span>
                        <ul class="dropdown-content" role="menu" aria-expanded="false">
                        <li>
                            <a onclick=self.link.callback(|lang| Msg::LocaleSwitch(String::from("english")))>
                                <img src="images/english.png" height="15" width="15"/> {"English"}
                            </a>
                        </li>
                        <li>
                            <a onclick=self.link.callback(|lang| Msg::LocaleSwitch(String::from("russian")))>
                                <img src="images/russian.png" height="15" width="15"/> {"Русский"}
                            </a>
                        </li>
                        <li>
                            <a onclick=self.link.callback(|lang| Msg::LocaleSwitch(String::from("romanian")))>
                                 <img src="images/romanian.png" height="15" width="15"/> {"Română"}
                            </a>
                        </li>
                        </ul>
                    </li>
                </ul>
            </nav>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}