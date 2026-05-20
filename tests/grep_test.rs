use std::fs;

#[test]
fn test_file_search() {
    fs::write("temp.txt", "hello world\nrust lang").unwrap();

    let output = std::process::Command::new("cargo")
        .args(["run", "--", "hello", "temp.txt"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("hello world"));

    fs::remove_file("temp.txt").unwrap();
}


#[test]
fn test_parallel_does_not_crash() {
    let dir = "parallel_test_data";
    fs::create_dir_all(dir).unwrap();

    for i in 0..200 {
        fs::write(
            format!("{}/file{}.txt", dir, i),
            "hello world\n".repeat(100),
        )
        .unwrap();
    }

    let output = std::process::Command::new("cargo")
        .args(["run", "--release", "--", "hello", dir, "-r"])
        .output()
        .unwrap();

    assert!(output.status.success());

    fs::remove_dir_all(dir).unwrap();
}