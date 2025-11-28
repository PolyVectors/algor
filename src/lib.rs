pub mod frontend {
    pub mod font;
    pub mod style;
    pub mod theme;
    pub mod widgets;
}

pub mod backend {
    pub mod compiler {
        pub mod generator;
        pub mod lexer;
        pub mod parser;

        #[cfg(test)]
        pub mod tests;

        pub mod utils;
        pub use utils::compile;
    }

    pub mod config;
    pub mod virtual_machine;
}
