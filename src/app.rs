use std::time::{SystemTime, UNIX_EPOCH};

use cosmic::app::{Core, Task};
use cosmic::cosmic_config::CosmicConfigEntry;
use cosmic::cosmic_theme::{THEME_MODE_ID, ThemeMode};
use cosmic::iced::window::Id;
use cosmic::iced::{Limits, Subscription};
use cosmic::iced_runtime::core::window;
use cosmic::iced_winit::commands::popup::{destroy_popup, get_popup};
use cosmic::widget::Space;
use cosmic::{Element, iced_runtime};

pub const APPID: &str = "io.github.4rc0s.cosmic-ext-applet-dark-toggle";

fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub struct AppState {
    pub core: Core,
    popup: Option<window::Id>,
    pub theme_mode_config: ThemeMode,
    last_quit: Option<u128>,
}

#[derive(Clone, Debug)]
pub enum AppMsg {
    TogglePopup,
    ClosePopup,
    ThemeModeConfigChanged(ThemeMode),
    SetDarkMode(bool),
}

impl AppState {
    fn toggle_popup(&mut self) -> Task<AppMsg> {
        if self.popup.is_some() {
            self.close_popup()
        } else {
            self.open_popup()
        }
    }

    fn close_popup(&mut self) -> Task<AppMsg> {
        if let Some(id) = self.popup.take() {
            self.last_quit = Some(now());
            destroy_popup(id)
        } else {
            Task::none()
        }
    }

    fn open_popup(&mut self) -> Task<AppMsg> {
        if self
            .last_quit
            .map(|t| (now() - t) < 200)
            .unwrap_or(false)
        {
            return Task::none();
        }

        let new_id = Id::unique();
        self.popup = Some(new_id);

        let mut popup_settings = self.core.applet.get_popup_settings(
            self.core.main_window_id().unwrap(),
            new_id,
            None,
            None,
            None,
        );

        popup_settings.positioner.size_limits = Limits::NONE
            .min_width(200.0)
            .max_width(300.0)
            .min_height(50.0)
            .max_height(150.0);

        get_popup(popup_settings)
    }
}

impl cosmic::Application for AppState {
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    type Message = AppMsg;
    const APP_ID: &'static str = APPID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let app = AppState {
            core,
            popup: None,
            theme_mode_config: ThemeMode::default(),
            last_quit: None,
        };

        (app, Task::none())
    }

    fn on_close_requested(&self, id: window::Id) -> Option<AppMsg> {
        if self.popup == Some(id) {
            Some(AppMsg::ClosePopup)
        } else {
            None
        }
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            AppMsg::TogglePopup => return self.toggle_popup(),
            AppMsg::ClosePopup => return self.close_popup(),
            AppMsg::ThemeModeConfigChanged(config) => {
                self.theme_mode_config = config;
            }
            AppMsg::SetDarkMode(dark) => {
                self.theme_mode_config.is_dark = dark;
                if let Ok(helper) = ThemeMode::config() {
                    if let Err(e) = self.theme_mode_config.write_entry(&helper) {
                        error!("can't write theme mode: {e}");
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        self.applet_button_view()
    }

    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        if self.popup.is_none() {
            return Space::new(0, 0).into();
        }

        self.core.applet.popup_container(self.popup_view()).into()
    }

    fn style(&self) -> Option<iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        self.core
            .watch_config(THEME_MODE_ID)
            .map(|u| AppMsg::ThemeModeConfigChanged(u.config))
    }
}
