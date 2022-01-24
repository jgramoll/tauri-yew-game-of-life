use gloo_console as console;
use gloo_timers::callback::Interval;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

const ROWS: usize = 20;
const COLUMNS: usize = 50;

enum Msg {
    Payload(String),
    Tick,
}

struct App {
    cells: Vec<bool>,
    value: String,
    _interval: Interval,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| Msg::Tick);
        let _interval = Interval::new(200, move || callback.emit(()));

        Self {
            cells: vec![false; ROWS * COLUMNS],
            // value: 0,
            // value: String::from("init"),
            value: String::new(),
            _interval,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                // todo
                console::info!("here");
                false
            }
            Msg::Payload(s) => {
                self.value = s;
                true
            } // Msg::AddOne => {
              //     let callback = ctx.link().callback(Msg::Payload);
              //     spawn_local(async move {
              //         let payload = hello("World".into()).await.unwrap().as_string().unwrap();
              //         callback.emit(payload);
              //     });
              //     false
              // }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let new_rows = self
            .cells
            .chunks(COLUMNS)
            //.enumerate()
            .map(|cell_values| {
                //
                let cells = cell_values.iter().map(|value| {
                    html! {
                        <td>
                            { if *value { 1 } else { 0 } }
                        </td>
                    }
                });
                html! {
                    <tr>
                        { for cells }
                    </tr>
                }
            });

        let link = ctx.link();

        html! {
            <div>
                <table>
                    { for new_rows }
                </table>
                // <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value.clone() }</p>
            </div>
        }
    }
}

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn hello(name: String) -> Result<JsValue, JsValue>;
}

fn main() {
    yew::start_app::<App>();
}
