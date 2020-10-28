use rusty_markov_traffic::MarkovChain;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum UserAction {
    SignIn,
    SignOut,
    CreateTodo,
    DeleteTodo,
    ListTodos,
}

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
        UserAction::DeleteTodo,
        UserAction::CreateTodo,
        UserAction::CreateTodo,
        UserAction::SignOut,
        UserAction::SignIn,
        UserAction::ListTodos,
        UserAction::CreateTodo,
        UserAction::DeleteTodo,
        UserAction::CreateTodo,
        UserAction::DeleteTodo,
        UserAction::ListTodos,
        UserAction::DeleteTodo,
        UserAction::SignOut,
    ];
    chain.update(&actions);

    // Generate N events
    for action in chain.iter().take(16) {
        if action == UserAction::SignIn {
            println!("## New session ##");
        }
        println!("{:?}", action);
    }

    // Generate until SignOut
    // for action in chain
    //     .iter()
    //     .take_while(|action| *action != UserAction::SignOut)
    // {
    //     println!("{:?}", action);
    // }
}
