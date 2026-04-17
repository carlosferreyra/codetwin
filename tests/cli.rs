//! CLI argument parsing (no subprocess — uses `clap::Parser::try_parse_from`).

use clap::Parser;
use codetwin::cli::{Cli, Command};

#[test]
fn bare_invocation_has_no_subcommand() {
    let cli = Cli::try_parse_from(["codetwin"]).unwrap();
    assert!(cli.command.is_none());
}

#[test]
fn gen_layout_flag_parses() {
    let cli = Cli::try_parse_from(["codetwin", "gen", "--layout", "architecture-map"]).unwrap();
    match cli.command.expect("subcommand") {
        Command::Gen(args) => assert_eq!(args.layout.as_deref(), Some("architecture-map")),
        other => panic!("unexpected command: {other:?}"),
    }
}

#[test]
fn global_verbose_is_available_on_subcommands() {
    let cli = Cli::try_parse_from(["codetwin", "--verbose", "list"]).unwrap();
    assert!(cli.verbose);
    matches!(cli.command, Some(Command::List(_)));
}

#[test]
fn diff_accepts_two_positional_refs() {
    let cli = Cli::try_parse_from(["codetwin", "diff", "HEAD~1", "HEAD"]).unwrap();
    match cli.command.expect("subcommand") {
        Command::Diff(args) => {
            assert_eq!(args.ref_a.as_deref(), Some("HEAD~1"));
            assert_eq!(args.ref_b.as_deref(), Some("HEAD"));
        }
        other => panic!("unexpected command: {other:?}"),
    }
}

#[test]
fn unknown_subcommand_is_rejected() {
    let err = Cli::try_parse_from(["codetwin", "bogus"]).unwrap_err();
    assert!(err.to_string().to_lowercase().contains("unrecognized"));
}

// TODO(Phase 1.e): integration test for `--save` persistence once implemented.
