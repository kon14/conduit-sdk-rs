use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

const CONDUIT_TAG: &str = "v0.16.8";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    prepare_dirs();
    pull_protos().await;
    build_protos();
    Ok(())
}

fn get_proto_url(rel_path: &str) -> String {
    format!("https://raw.githubusercontent.com/ConduitPlatform/Conduit/{CONDUIT_TAG}/{rel_path}")
}

fn prepare_dirs() {
    if Path::new("protos/conduit").is_dir() {
        fs::remove_dir_all("protos/conduit").unwrap();
    }
    if Path::new("proto_gen").is_dir() {
        fs::remove_dir_all("proto_gen").unwrap();
    }
    fs::create_dir("protos/conduit").unwrap();
    fs::create_dir("proto_gen").unwrap();
}

async fn pull_protos() {
    let mut proto_src_paths: HashMap<&str, &str> = HashMap::new();
    proto_src_paths.insert("conduit", "packages/core/src/core.proto");
    proto_src_paths.insert(
        "conduit.module.v1",
        "libraries/module-tools/src/module.proto",
    );
    proto_src_paths.insert(
        "authentication",
        "modules/authentication/src/authentication.proto",
    );
    proto_src_paths.insert(
        "authorization",
        "modules/authorization/src/authorization.proto",
    );
    proto_src_paths.insert("chat", "modules/chat/src/chat.proto");
    proto_src_paths.insert("database", "modules/database/src/database.proto");
    proto_src_paths.insert("email", "modules/email/src/email.proto");
    proto_src_paths.insert(
        "push-notifications",
        "modules/push-notifications/src/push-notifications.proto",
    );
    proto_src_paths.insert("router", "modules/router/src/router.proto");
    proto_src_paths.insert("sms", "modules/sms/src/sms.proto");
    proto_src_paths.insert("storage", "modules/storage/src/storage.proto");
    for (key, rel_path) in proto_src_paths {
        let res = reqwest::get(get_proto_url(rel_path))
            .await
            .unwrap_or_else(|_| panic!("Failed to retrieve {key} proto"));
        let body = res.text().await.unwrap();
        let mut out = fs::File::create(format!("protos/conduit/{key}.proto")).unwrap();
        io::copy(&mut body.as_bytes(), &mut out).unwrap();
    }
}

fn build_protos() {
    let proto_files = [
        "protos/grpc/grpc.health.v1.proto",
        "protos/conduit/conduit.module.v1.proto",
        "protos/conduit/conduit.proto",
        "protos/conduit/authentication.proto",
        "protos/conduit/authorization.proto",
        "protos/conduit/chat.proto",
        "protos/conduit/database.proto",
        "protos/conduit/email.proto",
        "protos/conduit/push-notifications.proto",
        "protos/conduit/router.proto",
        "protos/conduit/sms.proto",
        "protos/conduit/storage.proto",
    ];
    tonic_build::configure()
        .build_server(false)
        .out_dir("proto_gen")
        .compile(&proto_files, &["protos"])
        .unwrap();
}
