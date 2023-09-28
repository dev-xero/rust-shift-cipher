use rust_shift_cipher::Config;

#[test]
fn test_build_config() {
    let args = vec![
        String::from("cmd"),
        String::from("-en"),
        String::from("password"), 
        String::from("8"),
    ];
    
    let args_iter = args.into_iter();
    let config = Config::build(args_iter).unwrap();

    assert_eq!(config, Config {
        text: String::from("password"),
        shift: 8,
        should_decrypt: false,
        should_export: false
    })
}

#[test]
fn test_build_config_with_export() {
    let args = vec![
        String::from("cmd"), 
        String::from("-en"),
        String::from("password"), 
        String::from("8"),
        String::from("-e")
    ];
    
    let args_iter = args.into_iter();
    let config = Config::build(args_iter).unwrap();

    assert_eq!(config, Config {
        text: String::from("password"),
        shift: 8,
        should_decrypt: false,
        should_export: true
    })
}

#[test]
fn test_build_config_with_invalid_export_arg() {
    let args = vec![
        String::from("cmd"), 
        String::from("-en"),
        String::from("password"), 
        String::from("8"),
        String::from("-export")
    ];
    
    let args_iter = args.into_iter();
    let config = Config::build(args_iter).unwrap();

    assert_eq!(config, Config {
        text: String::from("password"),
        shift: 8,
        should_decrypt: false,
        should_export: false
    })
}

#[test]
#[should_panic]
fn test_build_config_with_insufficient_args() {
    let args = vec![
        String::from("cmd"), 
        String::from("password")
    ];
    
    let args_iter = args.into_iter();
    let _ = Config::build(args_iter).unwrap();
}