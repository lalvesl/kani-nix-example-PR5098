// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Adapted from Kani's tutorial: `docs/src/tutorial/first-steps-v2`.

pub fn estimate_size(x: u32) -> u32 {
    assert!(x < 4096);

    if x < 256 {
        if x < 128 {
            return 1;
        } else {
            return 3;
        }
    } else if x < 1024 {
        if x > 1022 {
            return 4;
        } else {
            return 5;
        }
    } else {
        if x < 2048 {
            return 7;
        } else {
            return 9;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(estimate_size(1024), 7);
    }
}

#[cfg(kani)]
mod proofs {
    use super::*;

    /// Holds for every input that satisfies `estimate_size`'s precondition.
    #[kani::proof]
    fn verify_success() {
        let x: u32 = kani::any();
        kani::assume(x < 4096);
        let y = estimate_size(x);
        assert!(y < 10);
    }

    /// Without the precondition, Kani finds the input that fails the `assert!`.
    #[kani::proof]
    #[kani::should_panic]
    fn finds_precondition_violation() {
        let x: u32 = kani::any();
        estimate_size(x);
    }
}
