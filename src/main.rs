mod transmission;

use gtk::prelude::{
    BoxExt as _, ButtonExt as _, GtkWindowExt as _, OrientableExt as _, WidgetExt as _,
};

#[derive(Debug)]
enum AppMsg {
    FetchVersion,
}

#[derive(Debug)]
struct AppModel {}

#[relm4::component(async)]
impl relm4::component::SimpleAsyncComponent for AppModel {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("Gush"),
            set_default_width: 300,
            set_default_height: 100,

            adw::ToolbarView {
                add_top_bar = &adw::HeaderBar { },

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
                    },
                },
            }
        }
    }

    async fn init(
        _: Self::Init,
        root: Self::Root,
        sender: relm4::AsyncComponentSender<Self>,
    ) -> relm4::component::AsyncComponentParts<Self> {
        let model = AppModel {};

        let widgets = view_output!();

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

                dbg!(response);
            }
        }
    }
}

fn main() {
    let app = relm4::RelmApp::new("work.neets.Gush");
    app.run_async::<AppModel>(());
}
