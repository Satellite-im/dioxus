use dioxus_core::prelude::*;

fn main() {
    let mut s = Context { props: &() };
    let g = Component(&mut s);
}

struct CompState {
    tasks: Vec<()>,
}
enum Actions {
    Add,
    MoveUp,
    MoveDown,
    Remvoe,
}

static Component: FC<()> = |ctx| {
    let (tasks, dispatch) = use_reducer(
        ctx,
        || CompState { tasks: Vec::new() },
        |state, action: Actions| match action {
            Actions::Add => state,
            Actions::MoveUp => state,
            Actions::MoveDown => state,
            Actions::Remvoe => state,
        },
    );

    let tasklist = { (0..10).map(|f| html! { <li></li> }) }.collect::<Vec<_>>();

    html! {
        <div>
            <div>
                <h1>"Tasks: "</h1>
                <ul>
                    {tasklist}
                </ul>
            </div>
            <div>
                <button onclick=|_| dispatch(Action::Add)>{"Add"}</button>
                <button onclick=|_| dispatch(Action::MoveUp)>{"MoveUp"}</button>
                <button onclick=|_| dispatch(Action::MoveDown)>{"MoveDown"}</button>
                <button onclick=|_| dispatch(Action::Remvoe)>{"Remvoe"}</button>
            </div>
        </div>
    }
};

fn use_reducer<Props, State, Action>(
    ctx: &mut Context<Props>,
    init: fn() -> State,
    reducer: fn(State, Action) -> State,
) -> (State, impl Fn(Action)) {
    let ii = init();
    (ii, |_| {})
}