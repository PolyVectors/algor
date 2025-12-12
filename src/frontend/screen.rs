#[derive(Clone, Debug, Default, PartialEq)]
pub enum Screen {
    #[default]
    Menu,
    LessonSelect,
    LessonView,
    Sandbox,
    Settings,
}
