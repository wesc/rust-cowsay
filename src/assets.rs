use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/cows/"]
pub struct Assets;

pub fn get(path: &str) -> Option<std::borrow::Cow<'static, [u8]>> {
    Assets::get(path).map(|file| file.data)
}

pub fn list() -> Vec<String> {
    Assets::iter().map(|s| s.to_string()).collect()
}
