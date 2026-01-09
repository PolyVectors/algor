pub mod frontend {
    pub mod util {
        pub mod font;
        pub mod theme;
        pub mod widgets;
    }

    pub mod pane;
    pub mod screen;
}

pub mod backend {
    pub mod compiler;
    pub mod config;
    pub mod lessons;
}

pub mod shared {
    pub mod runtime;
    pub mod vm;
}
