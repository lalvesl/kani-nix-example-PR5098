// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Adapted from Kani's tutorial: `docs/src/tutorial/loops-unwinding`, with the tutorial's
//! off-by-one bug (`0..=length`) fixed and the loop unwound enough.

pub fn initialize_prefix(length: usize, buffer: &mut [u8]) {
    // Let's just ignore invalid calls
    if length > buffer.len() {
        return;
    }

    for i in 0..length {
        buffer[i] = 0;
    }
}

#[cfg(kani)]
mod proofs {
    use super::*;

    const LIMIT: usize = 10;

    /// The loop runs at most `LIMIT` times, so it needs `LIMIT + 1` unwindings.
    #[kani::proof]
    #[kani::unwind(11)]
    fn check_initialize_prefix() {
        let mut buffer: [u8; LIMIT] = [1; LIMIT];

        let length = kani::any();
        kani::assume(length <= LIMIT);

        initialize_prefix(length, &mut buffer);

        for i in 0..LIMIT {
            assert_eq!(buffer[i], if i < length { 0 } else { 1 });
        }
    }
}
