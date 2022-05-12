// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use move_cli::package::prover::ProverTest;

// TODO: split this into individual tests once the package system supports this.
#[test]
fn prove_all() {
    ProverTest::create("core").run();
    ProverTest::create("experimental").run();
    ProverTest::create("DPN").run()
}

#[test]
#[ignore]
fn prove_checking_inconsistency() {
    ProverTest::create("DPN")
        .with_options(&["--check-inconsistency"])
        .run()
}

#[test]
#[ignore]
fn prove_using_cvc5() {
    // TODO: This test is disabled because it hangs (> 10 mins in a local test).
    //ProverTest::create("DPN").with_options(&["--use-cvc5"]).run()
}
