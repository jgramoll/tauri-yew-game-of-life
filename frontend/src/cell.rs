use yew::{prelude::*, MouseEvent};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub active: bool,
    pub onclick: Callback<MouseEvent>,
}

pub struct CellComponent;

fn cell_class(active: bool) -> Classes {
    let cell_status = if active { "cell-live" } else { "cell-dead" };
    classes!("cell", cell_status)
}

impl Component for CellComponent {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        CellComponent
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.props().onclick.clone();
        let class = cell_class(ctx.props().active);
        html! {
            <td
                {class}
                {onclick}
            />
        }
    }
}
