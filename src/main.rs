mod app;
mod library;
mod ui;

use gio::prelude::ApplicationExt as _;
use gtk::prelude::{GtkApplicationExt, GtkWindowExt as _, WidgetExt as _};
use relm4::{
    Component as _, ComponentController as _, Controller,
    prelude::{AsyncComponent as _, AsyncComponentController as _, AsyncController},
};

#[derive(Debug)]
enum AppMsg {
    CreateLibrary(std::path::PathBuf),
    OpenPreferences,
    OpenAbout,
    OpenShortcuts,
    Quit,
}

#[derive(Debug)]
struct AppModel {
    root: adw::ApplicationWindow,
    preferences: AsyncController<ui::preferences::Preferences>,
    about: Controller<ui::about::About>,
    shortcuts: Controller<ui::shortcuts::Shortcuts>,
    setup: AsyncController<ui::setup::Setup>,
}

relm4::new_action_group!(pub WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");
relm4::new_stateless_action!(ShortcutsAction, WindowActionGroup, "shortcuts");
relm4::new_stateless_action!(QuitAction, WindowActionGroup, "quit");

#[relm4::component(async)]
impl relm4::component::SimpleAsyncComponent for AppModel {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    menu! {
        primary_menu: {
            section! {
                "Preferences" => PreferencesAction,
                "Keyboard Shortcuts" => ShortcutsAction,
                "About Emiolia " => AboutAction,
            }
        }
    }

    view! {
        adw::ApplicationWindow {
            set_title: Some(app::NAME),
            set_default_width: 300,
            set_default_height: 100,
            // add_css_class: "devel",

            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::Quit);
                glib::Propagation::Stop
            },

            #[name = "main"]
            adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    pack_end = &gtk::MenuButton {
                        set_tooltip_text: Some("Menu"),
                        set_icon_name: "open-menu-symbolic",
                        set_primary: true,
                        set_menu_model: Some(&primary_menu),
                    },
                },
            }
        }
    }

    async fn init(
        (): Self::Init,
        root: Self::Root,
        sender: relm4::AsyncComponentSender<Self>,
    ) -> relm4::component::AsyncComponentParts<Self> {
        let preferences = ui::preferences::Preferences::builder()
            .launch(root.clone())
            .detach();
        let about = ui::about::About::builder().launch(root.clone()).detach();

        let widgets = view_output!();

        let app = relm4::main_adw_application();
        app.set_menubar(Some(&primary_menu));

        let accels = ui::macros::actions!(app, &root, sender, WindowActionGroup, {
            PreferencesAction["Preferences", "<Primary>comma"] => AppMsg::OpenPreferences,
            AboutAction => AppMsg::OpenAbout,
            ShortcutsAction["Show Shortcuts", "<Primary>question"] => AppMsg::OpenShortcuts,
            QuitAction["Quit", "<Primary>q"] => AppMsg::Quit,
        });

        let shortcuts = ui::shortcuts::Shortcuts::builder()
            .launch(ui::shortcuts::Init {
                parent: root.clone(),
                items: vec![("Basic".to_string(), accels)],
            })
            .detach();

        let setup = ui::setup::Setup::builder().launch(root.clone()).forward(
            sender.input_sender(),
            |msg| match msg {
                ui::setup::Output::CreateLibrary(path) => AppMsg::CreateLibrary(path),
            },
        );

        widgets.main.set_content(Some(setup.widget()));

        let model = AppModel {
            root,
            preferences,
            about,
            shortcuts,
            setup,
        };

        relm4::component::AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: relm4::AsyncComponentSender<Self>) {
        match msg {
            AppMsg::CreateLibrary(path) => {
                dbg!(path);
            }
            AppMsg::OpenPreferences => self.preferences.emit(ui::preferences::Input::Present),
            AppMsg::OpenAbout => self.about.emit(ui::about::Input::Present),
            AppMsg::OpenShortcuts => self.shortcuts.emit(ui::shortcuts::Input::Present),
            AppMsg::Quit => relm4::main_adw_application().quit(),
        }
    }
}

fn main() {
    let app = relm4::main_adw_application();
    app.set_application_id(Some(app::ID));

    let app = relm4::RelmApp::from_app(app);
    app.run_async::<AppModel>(());
}
