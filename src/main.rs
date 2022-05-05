use yew::prelude::*;
use yew_router::prelude::*;

enum Msg {
    AddOne,
    SubtractOne,
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::SubtractOne => {
                self.value -= 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div class={"container py-5"}>
                <button class={"btn btn-primary"} onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <button class={"btn btn-primary"} onclick={link.callback(|_| Msg::SubtractOne)}>{ "-1" }</button>
                <p>{ self.value }</p>
                <p>{"Hello!!"}</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

fn Home() -> Html {
    html! {
        <div>
            <h1>{"Home"}</h1>
        </div>
    }
}
