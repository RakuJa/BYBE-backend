use bybe_backend::{InitializeLogResponsibility, StartOptions};

fn main() -> std::io::Result<()> {
    bybe_backend::start(StartOptions {
        init_log_resp: InitializeLogResponsibility::Personal,
        ..Default::default()
    })
}
