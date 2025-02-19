fn main() {
    #[cfg(not(feature = "colors"))]
    println!("Feature color is switched off");

    #[cfg(feature = "colors")]
    {
        use ansi_term::Color;
        use atty::Stream::{Stderr, Stdout};

        for i in 0..=255 {
            println!("{}: {}", i, Color::Fixed(i).paint(i.to_string()));
        }

        println!("");

        if atty::is(Stdout) {
            println!(
                "Stdout is considered a tty - \
                 flexi_logger::AdaptiveFormat will use colors",
            );
        } else {
            println!(
                "Stdout is not considered a tty - \
                 flexi_logger::AdaptiveFormat will NOT use colors"
            );
        }

        if atty::is(Stderr) {
            println!(
                "Stderr is considered a tty - \
                 flexi_logger::AdaptiveFormat will use colors",
            );
        } else {
            println!(
                "Stderr is not considered a tty - \
                 flexi_logger::AdaptiveFormat will NOT use colors!"
            );
        }

        if cfg!(windows) {
            if !ansi_term::enable_ansi_support().is_ok() {
                println!("Unsupported windows console detected, coloring will likely not work");
            }
        }

        println!(
            "\n{}",
            Color::Fixed(196)
                .bold()
                .paint("err! output (red) with default palette")
        );
        println!(
            "{}",
            Color::Fixed(208)
                .bold()
                .paint("warn! output (yellow) with default palette")
        );
        println!("{}", "info! output (normal) with default palette");
        println!(
            "{}",
            Color::Fixed(7).paint("debug! output (normal) with default palette")
        );
        println!(
            "{}",
            Color::Fixed(8).paint("trace! output (grey) with default palette")
        );

        println!(
            "\n{}",
            Color::Red
                .bold()
                .paint("err! output (red) with env_logger-palette")
        );
        println!(
            "{}",
            Color::Yellow.paint("warn! output (yellow) with env_logger-palette")
        );
        println!(
            "{}",
            Color::Green.paint("info! output (green) with env_logger-palette")
        );
        println!(
            "{}",
            Color::Blue.paint("debug! output (blue) with env_logger-palette")
        );
        println!(
            "{}",
            Color::Cyan.paint("trace! output (cyan) with env_logger-palette")
        );
    }
}
