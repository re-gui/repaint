

fn main() {
    println!("Hello from an example!");

    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();

    let out_file = exe_dir.join("hello.txt");
    let out_file_str = out_file.to_str().unwrap().strip_prefix("\\\\?\\").unwrap().replace("\\", "/"); 

    std::fs::write(out_file.clone(), "Hello from an example!").unwrap();

    println!("see output: {}", out_file_str);
}

