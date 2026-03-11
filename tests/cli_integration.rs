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

#[test]
fn add_without_init_fails_with_guidance() {
    let home = make_temp_home("add-no-init");
    let sample_path = home.join("sample.txt");
    fs::write(&sample_path, "hello").expect("failed to create sample file");

    let add = run(
        &home,
        &["add", sample_path.to_str().expect("utf-8 path expected")],
    );
    assert!(!add.status.success(), "add should fail before init");
    assert!(output_text(&add).contains("ConfigSync not initialized"));
}

#[test]
fn secrets_add_without_key_fails_with_guidance() {
    let home = make_temp_home("secret-no-key");

    let init = run(&home, &["init"]);
    assert!(init.status.success(), "init failed: {}", output_text(&init));

    let secret_path = home.join("secret.env");
    fs::write(&secret_path, "TOKEN=abc").expect("failed to create secret file");
    let add_secret = run(
        &home,
        &[
            "secrets",
            "add",
            secret_path.to_str().expect("utf-8 path expected"),
        ],
    );
    assert!(
        !add_secret.status.success(),
        "secrets add should fail without key initialization"
    );
    assert!(output_text(&add_secret).contains("Have you run `configsync secrets init`?"));
}

#[test]
fn add_detects_repository_name_collision() {
    let home = make_temp_home("add-collision");

    let init = run(&home, &["init"]);
    assert!(init.status.success(), "init failed: {}", output_text(&init));

    let first_dir = home.join("one");
    let second_dir = home.join("two");
    fs::create_dir_all(&first_dir).expect("failed to create first dir");
    fs::create_dir_all(&second_dir).expect("failed to create second dir");

    let first_path = first_dir.join("dup.txt");
    let second_path = second_dir.join("dup.txt");
    fs::write(&first_path, "one").expect("failed to create first file");
    fs::write(&second_path, "two").expect("failed to create second file");

    let first_add = run(
        &home,
        &["add", first_path.to_str().expect("utf-8 path expected")],
    );
    assert!(
        first_add.status.success(),
        "first add failed: {}",
        output_text(&first_add)
    );

    let second_add = run(
        &home,
        &["add", second_path.to_str().expect("utf-8 path expected")],
    );
    assert!(
        !second_add.status.success(),
        "second add should fail due to basename collision"
    );
    assert!(output_text(&second_add).contains("already exists in repository"));
}

#[test]
fn apply_reports_conflict_for_existing_regular_file_destination() {
    let home = make_temp_home("apply-conflict");

    let init = run(&home, &["init"]);
    assert!(init.status.success(), "init failed: {}", output_text(&init));

    let sample_path = home.join("conflict.txt");
    fs::write(&sample_path, "original").expect("failed to create sample file");

    let add = run(
        &home,
        &["add", sample_path.to_str().expect("utf-8 path expected")],
    );
    assert!(add.status.success(), "add failed: {}", output_text(&add));

    // Replace symlink destination with a normal file to emulate user drift.
    fs::remove_file(&sample_path).expect("failed to remove existing symlink");
    fs::write(&sample_path, "local drift").expect("failed to create conflict file");

    let apply = run(&home, &["apply"]);
    assert!(
        apply.status.success(),
        "apply failed: {}",
        output_text(&apply)
    );
    assert!(output_text(&apply).contains("Destination exists and is not a symlink."));
}
