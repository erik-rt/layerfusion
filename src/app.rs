use clap::{crate_version, AppSettings, Arg, ColorChoice, Command};

pub fn build_app() -> Command<'static> {
    let clap_color_choice = if std::env::var_os("NO_COLOR").is_none() {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    };

    let app = Command::new("generate")
        .version(crate_version!())
        .color(clap_color_choice)
        .setting(AppSettings::DeriveDisplayOrder)
        .dont_collapse_args_in_usage(true)
        .after_help("This is displayed after help.")
        .arg(
            Arg::new("batch-size")
                .long("batch-size")
                .short('b')
                .value_name("size")
                .help("Size of the collection to be generated")
                .long_help(
                    "The total number of assets to generate. This assumes that the user would \
                         would like to generate brand new assets from scratch.",
                ),
        );
    app
}

#[test]
fn verify_app() {
    build_app().debug_assert()
}
