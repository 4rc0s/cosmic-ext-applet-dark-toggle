use crate::app::{AppMsg, AppState};
use crate::fl;
use cosmic::Element;
use cosmic::applet::padded_control;
use cosmic::iced::Alignment;
use cosmic::widget::{column, horizontal_space, mouse_area, row, text, toggler};

impl AppState {
    pub fn applet_button_view(&self) -> Element<'_, AppMsg> {
        self.core
            .applet
            .icon_button_from_handle(crate::icon::icon_display())
            .on_press(AppMsg::TogglePopup)
            .into()
    }

    pub fn popup_view(&self) -> Element<'_, AppMsg> {
        column()
            .padding(10)
            .push(self.dark_mode_view())
            .into()
    }

    fn dark_mode_view(&self) -> Element<'_, AppMsg> {
        padded_control(
            mouse_area(
                row()
                    .align_y(Alignment::Center)
                    .push(text(fl!("dark_mode")))
                    .push(horizontal_space())
                    .push(toggler(self.theme_mode_config.is_dark).on_toggle(AppMsg::SetDarkMode)),
            )
            .on_press(AppMsg::SetDarkMode(!self.theme_mode_config.is_dark)),
        )
        .into()
    }
}
