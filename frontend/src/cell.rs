use yew::{prelude::*, MouseEvent};

pub enum Msg {
    Click(MouseEvent),
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub active: bool,
    pub onclick: Callback<MouseEvent>,
}

pub struct CellComponent;

impl Component for CellComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        CellComponent
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click(m) => {
                ctx.props().onclick.emit(m);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let active = ctx.props().active;
        let cell_status = {
            if active {
                "cell-live"
            } else {
                "cell-dead"
            }
        };

        let onclick = ctx.link().callback(Msg::Click);
        html! {
            <td
                class={classes!("cell", cell_status)}
                {onclick}
            >
                { if active { 1 } else { 0 } }
            </td>
        }
    }
}
