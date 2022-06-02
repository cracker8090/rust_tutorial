pub fn play(name: String) {
    println!("Playing basic test {} :basic-lib", name);
}

#[test]
fn play_test() {
    play("Play testing".to_string());
}

#[test]
fn play_result_test() -> Result<(), String> {
    if 1 + 2 == 3 {
        Ok(())
    } else {
        Err(String::from("play_result_test error"))
    }
}
