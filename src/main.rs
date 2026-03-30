mod transmission;
mod ui;

use gio::prelude::ApplicationExt;
use gtk::prelude::{
    BoxExt as _, ButtonExt as _, GtkApplicationExt, GtkWindowExt as _, OrientableExt as _,
    WidgetExt as _,
};
use relm4::{
    Component as _, ComponentController as _, Controller,
    actions::{RelmAction, RelmActionGroup},
    prelude::{AsyncComponent as _, AsyncComponentController as _, AsyncController},
};

#[derive(Debug)]
enum AppMsg {
    FetchVersion,
    OpenPreferences,
    OpenAbout,
    OpenShortcuts,
}

struct AppModel {
    preferences: AsyncController<ui::preferences::Preferences>,
    about: Controller<ui::about::About>,
    shortcuts: Controller<ui::shortcuts::Shortcuts>,
}

relm4::new_action_group!(pub WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");
relm4::new_stateless_action!(ShortcutsAction, WindowActionGroup, "shortcuts");

#[relm4::component(async)]
impl relm4::component::SimpleAsyncComponent for AppModel {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    menu! {
        primary_menu: {
            section! {
                "Preferences" => PreferencesAction,
                "About" => AboutAction,
                "Shortcuts" => ShortcutsAction,
            }
        }
    }

    view! {
        adw::ApplicationWindow {
            set_title: Some("Gush"),
            set_default_width: 300,
            set_default_height: 100,

            // connect_close_request => move |_| { glib::Propagation::Stop },

            adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    pack_end = &gtk::MenuButton {
                        set_tooltip_text: Some("Menu"),
                        set_icon_name: "open-menu-symbolic",
                        set_primary: true,
                        set_menu_model: Some(&primary_menu),
                    },
                },

                adw::StatusPage {
                    set_icon_name: Some("network-transmit"),
                    set_title: "Gush",
                    set_description: Some("Configure a thing"),

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 18,
                        set_halign: gtk::Align::Center,

                        gtk::Button::with_label("Configure") {
                            add_css_class: "pill",
                            connect_clicked => AppMsg::FetchVersion,
                        },

                        gtk::Button::with_label("Settings") {
                            add_css_class: "pill",
                            connect_clicked => AppMsg::OpenPreferences,
                        },

                        gtk::Button::with_label("About") {
                            add_css_class: "pill",
                            connect_clicked => AppMsg::OpenAbout,
                        },
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
        let shortcuts = ui::shortcuts::Shortcuts::builder()
            .launch(root.clone())
            .detach();

        let model = AppModel {
            preferences,
            about,
            shortcuts,
        };

        let widgets = view_output!();

        let mut actions = RelmActionGroup::<WindowActionGroup>::new();
        actions.add_action(RelmAction::<PreferencesAction>::new_stateless({
            let sender = model.preferences.sender().clone();
            move |_| {
                sender.send(ui::preferences::Input::Present).unwrap();
            }
        }));
        actions.add_action(RelmAction::<AboutAction>::new_stateless({
            let sender = model.about.sender().clone();
            move |_| {
                sender.send(ui::about::Input::Present).unwrap();
            }
        }));
        actions.add_action(RelmAction::<ShortcutsAction>::new_stateless({
            let sender = model.shortcuts.sender().clone();
            move |_| {
                sender.send(ui::shortcuts::Input::Present).unwrap();
            }
        }));

        actions.register_for_widget(&root);

        let app = relm4::main_adw_application();
        app.set_menubar(Some(&primary_menu));
        app.set_accels_for_action("win.preferences", &["<Primary>comma"]);
        app.set_accels_for_action("win.about", &["F1"]);
        app.set_accels_for_action("win.shortcuts", &["<Primary>question"]);

        sender.input(AppMsg::FetchVersion);

        relm4::component::AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: relm4::AsyncComponentSender<Self>) {
        match msg {
            AppMsg::FetchVersion => {
                let uri = glib::Uri::parse(
                    "http://192.168.1.25:9091/transmission/rpc",
                    glib::UriFlags::empty(),
                )
                .unwrap();

                let mut client = transmission::TransmissionClient {
                    uri,
                    auth: None,
                    session_id: None,
                };

                let session = soup::Session::new();

                let response = client.request(&session).await;

                let _ = dbg!(response);
            }

            AppMsg::OpenPreferences => {
                self.preferences.emit(ui::preferences::Input::Present);
            }

            AppMsg::OpenAbout => self.about.emit(ui::about::Input::Present),

            AppMsg::OpenShortcuts => self.shortcuts.emit(ui::shortcuts::Input::Present),
        }
    }
}

fn main() {
    let app = relm4::main_adw_application();
    app.set_application_id(Some("work.neets.Gush"));

    let app = relm4::RelmApp::from_app(app);
    app.run_async::<AppModel>(());
}
