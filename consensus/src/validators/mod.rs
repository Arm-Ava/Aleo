// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.

use crate::{
    validator::{Round, Score, Stake, Validator},
    Address,
};

use anyhow::{bail, Result};
use core::ops::Deref;
use indexmap::IndexMap;

/// The validator set.
#[derive(Clone)]
pub struct Validators {
    validators: IndexMap<Address, Validator>,
}

impl Validators {
    /// Initializes a new validator set.
    pub fn new() -> Self {
        Self {
            validators: Default::default(),
        }
    }

    /// Returns the number of validators.
    pub fn num_validators(&self) -> usize {
        self.validators.len()
    }

    /// Returns the validator with the given address, if the validator exists.
    pub fn get(&self, address: &Address) -> Option<&Validator> {
        self.validators.get(address)
    }

    /// Returns the total amount staked.
    pub fn get_total_stake(&self) -> Stake {
        // Note: As the total supply cannot exceed 2^64, this is call to `sum` is safe.
        self.validators.values().map(Validator::stake).sum()
    }

    /// Returns the current leader.
    pub fn get_leader(&self) -> Result<Address> {
        // Retrieve the validator with the highest score.
        let leader = self
            .validators
            .iter()
            .map(|(address, validator)| (address, validator.score()))
            .fold((None, Score::MIN), |(a, power_a), (b, power_b)| match power_a > power_b {
                true => (a, power_a),
                false => (Some(*b), power_b),
            })
            .0;

        // Return the leader address.
        match leader {
            Some(leader) => Ok(leader),
            None => bail!("No leader was found"),
        }
    }
}

impl Validators {
    /// Increments
    pub fn increment_stake(&mut self, address: &Address, stake: Stake) {
        if let Some(validator) = self.validators.get_mut(address) {
            validator.increment_earned_by(stake);
        }
    }
}

impl Deref for Validators {
    type Target = IndexMap<Address, Validator>;

    /// Returns the underlying validator map.
    fn deref(&self) -> &Self::Target {
        &self.validators
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_stake() {
        let validators = Validators::new();
        assert_eq!(validators.get_total_stake(), 0);
    }
}