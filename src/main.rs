use bybe_backend::InitializeLogResponsibility;

fn main() -> std::io::Result<()> {
    bybe_backend::start(None, None, InitializeLogResponsibility::Personal)
}
