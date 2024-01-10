use xilem_web::{
    elements::html::*, get_element_by_id, interfaces::*, memoize, static_view, t, App, View,
    ViewMarker,
};

#[rustfmt::skip]
static ADJECTIVES: &[&str] = &[
    "pretty", "large", "big", "small", "tall", "short", "long", "handsome", "plain", "quaint",
    "clean", "elegant", "easy", "angry", "crazy", "helpful", "mushy", "odd", "unsightly",
    "adorable", "important", "inexpensive", "cheap", "expensive", "fancy",
];

static COLOURS: &[&str] = &[
    "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black",
    "orange",
];

static NOUNS: &[&str] = &[
    "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger",
    "pizza", "mouse", "keyboard",
];

fn random(max: usize) -> usize {
    (js_sys::Math::random() * 1000.0) as usize % max
}

fn generate_label() -> String {
    let adjective = ADJECTIVES[random(ADJECTIVES.len())];
    let colour = COLOURS[random(COLOURS.len())];
    let noun = NOUNS[random(NOUNS.len())];
    let mut label = String::with_capacity(adjective.len() + colour.len() + noun.len() + 2);
    label.push_str(adjective);
    label.push(' ');
    label.push_str(colour);
    label.push(' ');
    label.push_str(noun);
    label
}

struct Row {
    id: usize,
    label: String,
}

struct AppState {
    next_id: usize,
    selected: Option<usize>,
    rows: Vec<Row>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            next_id: 1,
            selected: None,
            rows: Vec::new(),
        }
    }
}

impl AppState {
    fn create(&mut self, n: usize) -> impl ExactSizeIterator<Item = Row> {
        let id = self.next_id;
        self.next_id += n;
        (id..(id + n)).map(move |id| Row {
            id,
            label: generate_label(),
        })
    }
    fn run(&mut self) {
        self.rows.clear();
        self.add();
    }
    fn run_lots(&mut self) {
        self.rows.clear();
        let data = self.create(10000);
        self.rows.extend(data);
    }
    fn add(&mut self) {
        let data = self.create(1000);
        self.rows.extend(data);
    }
    fn remove(&mut self, id: usize) {
        if let Some(i) = self.rows.iter().position(|r| r.id == id) {
            self.rows.remove(i);
        }
    }
    fn update(&mut self) {
        let step = 10;
        for i in (0..(self.rows.len() / step)).map(|x| x * step) {
            self.rows[i].label.push_str(" !!!");
        }
    }
    fn clear(&mut self) {
        self.rows.clear();
    }
    fn select(&mut self, id: usize) {
        self.selected = Some(id);
    }
    fn swap_rows(&mut self) {
        if self.rows.len() >= 999 {
            self.rows.swap(1, 998);
        }
    }
}

fn control_buttons() -> impl View<AppState> + ViewMarker {
    static_view(|| {
        let control_button = |label, id, cb: fn(&mut AppState)| {
            div(button(label)
                .attr("type", "button")
                .attr("class", "btn btn-primary btn-block")
                .attr("id", id)
                .on_click(move |state, _| cb(state)))
            .attr("class", "col-sm-6 smallpad")
        };

        div(div((
            div(h1(
                "xilem_web elements-splice-hydration-simple-events (non-keyed)",
            ))
            .attr("class", "col-md-6"),
            div(div((
                control_button("Create 1,000 rows", "run", AppState::run),
                control_button("Create 10,000 rows", "runlots", AppState::run_lots),
                control_button("Append 1,000 rows", "add", AppState::add),
                control_button("Update every 10th row", "update", AppState::update),
                control_button("Clear", "clear", AppState::clear),
                control_button("Swap Rows", "swaprows", AppState::swap_rows),
            ))
            .attr("class", "row"))
            .attr("class", "col-md-6"),
        ))
        .attr("class", "row"))
        .attr("class", "jumbotron")
    })
}

fn row(row: &Row, selected: Option<usize>) -> impl View<AppState> + ViewMarker {
    memoize(
        (row.id, row.label.clone(), selected == Some(row.id)),
        |(id, label, selected)| {
            let id = *id;
            t(tr((
                td(id.to_string()).attr("class", "col-md-1"),
                td(a(label.clone()).on_click(move |state: &mut AppState, _| state.select(id)))
                    .attr("class", "col-md-4"),
                td(a(span(())
                    .attr("class", "glyphicon glyphicon-remove")
                    .attr("aria-hidden", "true"))
                .on_click(move |state: &mut AppState, _| state.remove(id)))
                .attr("class", "col-md-1"),
                td(()).attr("class", "col-md-6"),
            ))
            .attr("class", selected.then_some("danger")))
        },
    )
}

pub fn main() {
    console_error_panic_hook::set_once();

    App::new(AppState::default(), |state: &mut AppState| {
        let rows: Vec<_> = state.rows.iter().map(|r| row(r, state.selected)).collect();
        div((
            control_buttons(),
            table(tbody(rows)).attr("class", "table table-hover table-striped test-data"),
            span(())
                .attr("class", "preloadicon glyphicon glyphicon-remove")
                .attr("aria-hidden", "true"),
        ))
        .attr("class", "container")
    })
    .run(&get_element_by_id("main"));
}
