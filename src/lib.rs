pub mod frontend {
    pub mod font;
    pub mod message;
    pub mod screen;
    pub mod style;
    pub mod theme;
    pub mod widgets;
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
