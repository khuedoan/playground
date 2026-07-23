fn main() {
    topcoat::icon::iconify::BuildConfig::new()
        .icon_set("feather")
        .stage()
        .unwrap();

    topcoat::tailwind::BuildConfig::new()
        .input("styles.css")
        .render()
        .unwrap();
}
