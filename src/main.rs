use bybe_backend::InitializeLogResponsibility;

fn main() -> std::io::Result<()> {
    bybe_backend::start(
        None,
        None,
        None,
        None,
        InitializeLogResponsibility::Personal,
    )
}
