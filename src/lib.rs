use std::collections::BTreeMap;
use std::fmt::Debug;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

pub trait Action: Clone + PartialEq + Eq + Ord + Debug {}

#[derive(Debug, Clone, Default)]
pub struct MarkovChain<A: Action> {
    occurrences: BTreeMap<Vec<A>, BTreeMap<A, usize>>,
    order: usize,
    memory: Vec<A>,
    rng: ThreadRng,
}

impl<A: Action> MarkovChain<A> {
    pub fn new(order: usize) -> Self {
        assert!(order > 0);
        MarkovChain {
            occurrences: BTreeMap::new(),
            order,
            memory: Vec::with_capacity(order),
            rng: ThreadRng::default(),
        }
    }

    pub fn update(&mut self, actions: &[A]) {
        let actions: Vec<_> = actions.to_vec();
        for history in actions.windows(self.order + 1) {
            // Split window to 0..N-1 and N
            let previous = history[0..self.order].to_vec();
            let current = history.last().cloned().unwrap();

            // Count occurrence for current action
            let occurrences = self.occurrences.entry(previous).or_default();
            let count = occurrences.entry(current).or_default();
            *count += 1;
        }

        // Update internal memory
        self.memory.reserve(self.order);
        for action in actions.into_iter().rev().take(self.order) {
            self.memory.insert(0, action);
        }
        self.memory.truncate(self.order);
    }

    pub fn generate_from(&mut self, memory: &[A]) -> Option<A> {
        assert_eq!(memory.len(), self.order, "invalid memory size");

        if let Some(occurrences) = self.occurrences.get(memory) {
            // Get number of occurrences for each known action
            let occurrence_counts: Vec<_> = occurrences
                .iter()
                .map(|(action, count)| (action.clone(), *count))
                .collect();

            // Chose a random action based on its count
            occurrence_counts
                .choose_weighted(&mut self.rng, |(_, count)| *count)
                .map(|(action, _)| action)
                .ok()
                .cloned()
        } else {
            // No match from memory
            None
        }
    }

    pub fn generate(&mut self, update_memory: bool) -> Option<A> {
        let last_memory = self.memory.clone();
        if let Some(next) = self.generate_from(&last_memory) {
            if update_memory {
                // Update internal memory
                self.memory.insert(0, next.clone());
                self.memory.truncate(self.order);
            }

            Some(next)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum UserAction {
        SignIn,
        SignOut,
        CreateTodo,
    }

    impl Action for UserAction {}

    #[test]
    fn basic() {
        let mut chain = MarkovChain::new(1);
        let actions = vec![
            UserAction::SignIn,
            UserAction::CreateTodo,
            UserAction::SignOut,
        ];
        chain.update(&actions);

        assert_eq!(
            chain.generate_from(&[UserAction::SignIn]).unwrap(),
            UserAction::CreateTodo
        );
        assert_eq!(
            chain.generate_from(&[UserAction::CreateTodo]).unwrap(),
            UserAction::SignOut
        );
    }
}
