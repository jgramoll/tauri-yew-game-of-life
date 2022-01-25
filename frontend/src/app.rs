use gloo_console as console;
use gloo_timers::callback::Interval;
use rand::Rng;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::bindings;
use crate::cell;

const DEFAULT_ROWS: usize = 30;
const DEFAULT_COLUMNS: usize = 65;
const INTERVAL_MS: u32 = 200;

pub enum Msg {
    ActivateCell(usize, usize),
    Random,
    Step,
    Start,
    Stop,
    Reset,
    Tick,
    ChangeRow(InputEvent),
    ChangeCol(InputEvent),
}

pub struct App {
    active: bool,
    cells: Vec<bool>,
    row_count: usize,
    col_count: usize,
    _interval: Interval,
}

impl App {
    fn toggle_cell(&mut self, row: usize, col: usize) {
        let i = self.row_col_as_idx(row, col);
        self.cells[i] = !self.cells[i];
    }

    fn random_mutate(&mut self) {
        for cell in self.cells.iter_mut() {
            if rand::thread_rng().gen() {
                *cell = true;
            } else {
                *cell = false;
            }
        }
    }

    fn reset(&mut self) {
        self.cells = vec![false; self.row_count * self.col_count];
    }

    fn step(&mut self) {
        let mut new_cells = self.cells.clone();

        for row in 0..self.row_count {
            for col in 0..self.col_count {
                let live_count = self.alive_neighbors(row, col);

                let current_idx = self.row_col_as_idx(row, col);
                let new_value = if self.cells[current_idx] {
                    match live_count {
                        // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
                        0 | 1 => false,
                        // Any live cell with two or three live neighbours lives on to the next generation.
                        2 | 3 => true,
                        // Any live cell with more than three live neighbours dies, as if by overpopulation.
                        _ => false,
                    }
                } else {
                    // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                    live_count == 3
                };
                new_cells[current_idx] = new_value;
            }
        }

        self.cells = new_cells;

        // spawn_local(async move {
        //     bindings::hello("World".into()).await.unwrap();
        //     // todo
        //     console::info!("here");
        // });
    }

    fn alive_neighbors(&self, row: usize, col: usize) -> usize {
        let row = row as isize;
        let col = col as isize;
        [
            self.get_at_index(row + 1, col),
            self.get_at_index(row + 1, col + 1),
            self.get_at_index(row + 1, col - 1),
            self.get_at_index(row - 1, col),
            self.get_at_index(row - 1, col + 1),
            self.get_at_index(row - 1, col - 1),
            self.get_at_index(row, col + 1),
            self.get_at_index(row, col - 1),
        ]
        .iter()
        .filter_map(|&v| v)
        .filter(|&&v| v)
        .count()
    }

    fn get_at_index(&self, row: isize, col: isize) -> Option<&bool> {
        if row < 0 || col < 0 {
            None
        } else {
            self.cells
                .get(self.row_col_as_idx(row as usize, col as usize))
        }
    }

    fn row_col_as_idx(&self, row: usize, col: usize) -> usize {
        row * self.col_count + col
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| Msg::Tick);
        let _interval = Interval::new(INTERVAL_MS, move || callback.emit(()));

        Self {
            active: false,
            cells: vec![false; DEFAULT_ROWS * DEFAULT_COLUMNS],
            row_count: 30,
            col_count: 65,
            _interval,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ActivateCell(row, col) => {
                self.toggle_cell(row, col);
                true
            }
            Msg::Random => {
                self.random_mutate();
                true
            }
            Msg::Step => {
                self.step();
                true
            }
            Msg::Start => {
                self.active = true;
                true
            }
            Msg::Stop => {
                self.active = false;
                true
            }
            Msg::Reset => {
                self.reset();
                true
            }
            Msg::Tick => {
                if self.active {
                    self.step();
                    true
                } else {
                    false
                }
            }
            Msg::ChangeRow(e) => {
                let target: HtmlInputElement = e.target().unwrap_throw().dyn_into().unwrap_throw();
                self.row_count = target.value().parse().unwrap_throw();
                self.reset();
                true
            }
            Msg::ChangeCol(e) => {
                let target: HtmlInputElement = e.target().unwrap_throw().dyn_into().unwrap_throw();
                self.col_count = target.value().parse().unwrap_throw();
                self.reset();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let new_rows = self
            .cells
            .chunks(self.col_count)
            .enumerate()
            .map(|(row, cell_values)| {
                let cells = cell_values.iter().enumerate().map(|(col, &active)| {
                    let onclick = ctx.link().callback(move |_| Msg::ActivateCell(row, col));
                    html! {
                        <cell::CellComponent {active} {onclick} />
                    }
                });
                html! {
                    <tr>
                        { for cells }
                    </tr>
                }
            });

        html! {
            <div>
                <table>
                    { for new_rows }
                </table>
                <div class="game-buttons">
                    <button class="game-button" onclick={ctx.link().callback(|_| Msg::Random)}> { "Random" } </button>
                    <button class="game-button" onclick={ctx.link().callback(|_| Msg::Step)}> { "Step" } </button>
                    <button class="game-button" onclick={ctx.link().callback(|_| Msg::Start)}> { "Start" } </button>
                    <button class="game-button" onclick={ctx.link().callback(|_| Msg::Stop)}> { "Stop" } </button>
                    <button class="game-button" onclick={ctx.link().callback(|_| Msg::Reset)}> { "Reset" } </button>
                </div>
                <div class="game-buttons">
                    <label for="rows">{"Rows"}</label>
                    <input id="rows" type="number" value={self.row_count.to_string()} oninput={ctx.link().callback(Msg::ChangeRow)} />
                    <label for="columns">{"Columns"}</label>
                    <input id="columns" type="number" value={self.col_count.to_string()} oninput={ctx.link().callback(Msg::ChangeCol)} />
                </div>
            </div>
        }
    }
}
