// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.

use pest::iterators::Pairs;
use super::Rule;

pub struct Config {
    /// The poll/question counter `(is_set: bool, start_from: usize)`
    pub counter: (bool, usize),
}

impl Config {
    /// Create new Config.
    /// Setting:
    /// - `counter` to `(false, 0)`
    pub fn new() -> Config {
        return Config {
            counter: (false, 0)
        };
    }

    pub fn parse(&mut self, config_ast: Pairs<'_, Rule>) {
        config_ast
            .into_iter()
            .filter( |pair| pair.as_rule() == Rule::CONFIG_OPSION)
            .flat_map( |pair| pair.into_inner() )
            .for_each( |inner_pair| {
                // Parse Counter feature.
                if inner_pair.as_rule() == Rule::CONFIG_COUNTER {
                    inner_pair
                        .into_inner()
                        .filter( |counter_pair| counter_pair.as_rule() == Rule::CONFIG_COUNTER_VALUE)
                        .take(1)
                        .for_each( |counter_pair| {
                            if let Ok(counter_value) = counter_pair.as_str().parse::<usize>() {
                                self.counter = (true, counter_value);
                            }
                        });
                }
            });
    }
}