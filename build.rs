fn main() {
    const proto2: [&str; 7] = [
        "messages_robocup_ssl_detection.proto",
        "messages_robocup_ssl_geometry.proto",
        "messages_robocup_ssl_wrapper.proto",
        "messages_robocup_ssl_wrapper_tracked.proto",
        "messages_robocup_ssl_detection_tracked.proto",
        "messages_robocup_ssl_geometry_legacy.proto",
        "messages_robocup_ssl_wrapper_legacy.proto"
    ];

    let root: String = "src/proto2/".to_owned();

    let result = proto2.iter().map(|x| format!("{}{}", root, x))
    .collect::<Vec<String>>();
    
    protobuf_codegen::Codegen::new()
    .cargo_out_dir("proto2")
    .inputs(&result)
    .include("src/proto2")
    .run_from_script();
}
