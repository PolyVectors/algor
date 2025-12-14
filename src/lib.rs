pub mod frontend {
    pub mod utils {
        pub mod font;
        pub mod style;
        pub mod theme;
        pub mod widgets;
    }

    pub mod screen;
}

pub mod backend {
    pub mod compiler;
    pub mod config;
}

pub mod shared {
    pub mod runtime;
    pub mod vm;
}
