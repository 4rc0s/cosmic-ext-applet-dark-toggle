#[macro_export]
macro_rules! icon_handle {
    ($name:literal) => {{
        let bytes = include_bytes!(concat!("../res/icons/", $name, ".svg"));
        cosmic::widget::icon::from_svg_bytes(bytes).symbolic(true)
    }};
}

pub fn icon_display() -> cosmic::widget::icon::Handle {
    icon_handle!("display-symbolic")
}
