pub mod frontend {
    pub mod app;

    pub mod components {
        pub mod menu;
        pub mod sandbox;
        pub mod settings;
    }

    pub mod handlers {
        pub mod messages;
        pub mod subscriptions;
    }

    pub mod utils {
        pub mod font;
        pub mod style;
        pub mod theme;
        pub mod widgets;
    }
}

pub mod backend {
    pub mod compiler;
    pub mod config;
}

pub mod shared {
    pub mod runtime;
    pub mod vm;
}
