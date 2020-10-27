use rusty_markov_traffic::{Action, MarkovChain};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum UserAction {
    SignIn,
    SignOut,
    CreateTodo,
    DeleteTodo,
    ListTodos,
}

impl Action for UserAction {}

fn main() {
    let mut chain = MarkovChain::new(1);
    let actions = vec![
        UserAction::SignIn,
        UserAction::ListTodos,
        UserAction::CreateTodo,
        UserAction::CreateTodo,
        UserAction::SignOut,
        UserAction::SignIn,
        UserAction::ListTodos,
        UserAction::DeleteTodo,
        UserAction::CreateTodo,
        UserAction::CreateTodo,
        UserAction::SignOut,
        UserAction::SignIn,
        UserAction::ListTodos,
        UserAction::DeleteTodo,
        UserAction::DeleteTodo,
        UserAction::DeleteTodo,
        UserAction::SignOut,
    ];
    chain.update(&actions);

    for _ in 0..16 {
        let action = chain.generate(true).unwrap();
        if action == UserAction::SignIn {
            println!("## New session ##");
        }
        println!("{:?}", action);
    }
}
