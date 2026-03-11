use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn bin_path() -> &'static str {
    env!("CARGO_BIN_EXE_configsync")
}

fn make_temp_home(prefix: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "configsync-{}-{}-{}",
        prefix,
        std::process::id(),
        stamp
    ));
    fs::create_dir_all(&path).expect("failed to create temporary HOME");
    path
}

fn run(home: &Path, args: &[&str]) -> Output {
    Command::new(bin_path())
        .args(args)
        .env("HOME", home)
        .env("XDG_CONFIG_HOME", home.join(".config"))
        .env("XDG_DATA_HOME", home.join(".local/share"))
        .output()
        .expect("failed to execute configsync")
}

fn output_text(output: &Output) -> String {
    let mut s = String::new();
    s.push_str(&String::from_utf8_lossy(&output.stdout));
    s.push_str(&String::from_utf8_lossy(&output.stderr));
    s
}

#[test]
fn init_creates_initial_commit_and_history_works() {
    let home = make_temp_home("init-history");

    let init = run(&home, &["init"]);
    assert!(init.status.success(), "init failed: {}", output_text(&init));

    let history = run(&home, &["history"]);
    assert!(
        history.status.success(),
        "history failed: {}",
        output_text(&history)
    );
    let text = output_text(&history);
    assert!(text.contains("Initialize ConfigSync repository"));
}

#[test]
fn undo_refuses_root_commit() {
    let home = make_temp_home("undo-root");

    let init = run(&home, &["init"]);
    assert!(init.status.success(), "init failed: {}", output_text(&init));

    let undo = run(&home, &["undo"]);
    assert!(!undo.status.success(), "undo should fail for root commit");
    let text = output_text(&undo);
    assert!(text.contains("Refusing to undo the initial repository commit"));
}

#[test]
fn push_without_git_identity_still_commits_locally() {
    let home = make_temp_home("push-signature");

    let init = run(&home, &["init"]);
    assert!(init.status.success(), "init failed: {}", output_text(&init));

    let sample_path = home.join("sample.txt");
    fs::write(&sample_path, "hello").expect("failed to create sample file");

    let add = run(
        &home,
        &["add", sample_path.to_str().expect("utf-8 path expected")],
    );
    assert!(add.status.success(), "add failed: {}", output_text(&add));

    let push = run(&home, &["push"]);
    assert!(
        push.status.success(),
        "push should succeed even without origin: {}",
        output_text(&push)
    );

    let history = run(&home, &["history"]);
    assert!(
        history.status.success(),
        "history failed: {}",
        output_text(&history)
    );
    let text = output_text(&history);
    assert!(text.contains("Update configurations (configsync)"));
}

#[test]
fn apply_reports_already_linked_when_symlink_is_correct() {
    let home = make_temp_home("apply-linked");

    let init = run(&home, &["init"]);
    assert!(init.status.success(), "init failed: {}", output_text(&init));

    let sample_path = home.join("linked.txt");
    fs::write(&sample_path, "data").expect("failed to create sample file");

    let add = run(
        &home,
        &["add", sample_path.to_str().expect("utf-8 path expected")],
    );
    assert!(add.status.success(), "add failed: {}", output_text(&add));

    let apply = run(&home, &["apply"]);
    assert!(
        apply.status.success(),
        "apply failed: {}",
        output_text(&apply)
    );
    assert!(output_text(&apply).contains("Already linked. Skipping."));
}

#[test]
fn pull_without_origin_shows_guidance_and_succeeds() {
    let home = make_temp_home("pull-no-origin");

    let init = run(&home, &["init"]);
    assert!(init.status.success(), "init failed: {}", output_text(&init));

    let pull = run(&home, &["pull"]);
    assert!(
        pull.status.success(),
        "pull should continue locally when origin is missing: {}",
        output_text(&pull)
    );
    assert!(output_text(&pull).contains("No git remote named 'origin'"));
}
