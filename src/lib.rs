use std::collections::BTreeMap;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

#[derive(Clone)]
pub struct MarkovChain<T>
where
    T: Clone + Ord,
{
    order: usize,
    occurrences: BTreeMap<Vec<T>, BTreeMap<T, usize>>,
    memory: Vec<T>,
    rng: ThreadRng,
}

impl<T> MarkovChain<T>
where
    T: Clone + Ord,
{
    pub fn new(order: usize) -> Self {
        assert!(order > 0);
        MarkovChain {
            order,
            occurrences: BTreeMap::new(),
            memory: Vec::with_capacity(order),
            rng: ThreadRng::default(),
        }
    }

    pub fn update(&mut self, events: &[T]) {
        let events: Vec<_> = events.to_vec();
        for history in events.windows(self.order + 1) {
            // Split window to 0..N-1 and N
            let previous = history[0..self.order].to_vec();
            let current = history.last().cloned().unwrap();

            // Count occurrence for current event
            self.occurrences
                .entry(previous)
                .or_default()
                .entry(current)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        // Update internal memory
        self.memory.reserve(self.order);
        for event in events.into_iter().rev().take(self.order) {
            self.memory.insert(0, event);
        }
        self.memory.truncate(self.order);
    }

    pub fn generate_from(&mut self, memory: &[T]) -> Option<T> {
        assert_eq!(memory.len(), self.order, "invalid memory size");

        if let Some(occurrences) = self.occurrences.get(memory) {
            // Get number of occurrences for each known event. We need a Vec for `SliceRandom::choose_weighted`.
            let occurrence_counts: Vec<_> = occurrences
                .iter()
                .map(|(event, count)| (event.clone(), *count))
                .collect();

            // Chose a random event based on its count
            occurrence_counts
                .choose_weighted(&mut self.rng, |(_, count)| *count)
                .map(|(event, _)| event)
                .ok()
                .cloned()
        } else {
            // No match from memory
            None
        }
    }

    pub fn generate(&mut self, update_memory: bool) -> Option<T> {
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

    pub fn iter(&mut self) -> MarkovChainIter<T> {
        MarkovChainIter { chain: self }
    }
}

pub struct MarkovChainIter<'a, T>
where
    T: Clone + Ord,
{
    chain: &'a mut MarkovChain<T>,
}

impl<'a, T> Iterator for MarkovChainIter<'a, T>
where
    T: Clone + Ord,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.chain.generate(true)
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
