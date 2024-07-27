use yew::html::Scope;
use yew::prelude::*;
use crate::life_lib::{Universe, Cell};
use gloo::timers::callback::Interval;

pub struct UniverseComponent {
    universe: Universe,
    link: Scope<Self>,
    running: bool,
    _interval: Option<Interval>,
}

pub enum Msg {
    Tick,
    Start,
    Stop,
}

impl Component for UniverseComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            universe: Universe::new(),
            link: ctx.link().clone(),
            running: false,
            _interval: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                self.universe.tick();
                true
            }
            Msg::Start => {
                if !self.running {
                    self.running = true;
                    let link = ctx.link().clone();
                    self._interval = Some(Interval::new(50, move || {
                        link.send_message(Msg::Tick);
                    }));
                }
                true
            }
            Msg::Stop => {
                self.running = false;
                self._interval = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cells = self.universe.render();
        html! {
            <div class="container mx-auto p-4 bg-white rounded-lg shadow-md text-center">
                <h1 class="text-2xl font-bold mb-4">{ "Game of Life" }</h1>
                <button class="bg-blue-500 text-white px-4 py-2 rounded mr-2" onclick={ctx.link().callback(|_| Msg::Tick)}>{ "Next Generation" }</button>
                <button class="bg-green-500 text-white px-4 py-2 rounded mr-2" onclick={ctx.link().callback(|_| Msg::Start)}>{ "Start" }</button>
                <button class="bg-red-500 text-white px-4 py-2 rounded" onclick={ctx.link().callback(|_| Msg::Stop)}>{ "Stop" }</button>
                <pre class="bg-gray-800 text-white p-4 rounded mt-4">{ cells }</pre>
            </div>
        }
    }
}