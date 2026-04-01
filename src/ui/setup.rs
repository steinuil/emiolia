use super::macros::unwrap_or_return;
use crate::app;
use adw::prelude::{ActionRowExt as _, PreferencesRowExt as _};
use gio::prelude::FileExt;
use gtk::prelude::{ButtonExt as _, WidgetExt as _};
use relm4::{
    AsyncComponentSender,
    prelude::{AsyncComponentParts, SimpleAsyncComponent},
};

#[derive(Debug)]
pub struct Setup {
    parent: Option<adw::ApplicationWindow>,
    library_directory: glib::GString,
}

#[derive(Debug)]
pub enum Input {
    SelectLibraryDirectory,
    CreateLibrary,
}

#[derive(Debug)]
pub enum Output {
    CreateLibrary(glib::GString),
}

#[derive(Debug)]
pub struct Init {
    pub parent: Option<adw::ApplicationWindow>,
    pub default_library_directory: glib::GString,
}

#[relm4::component(pub async)]
impl SimpleAsyncComponent for Setup {
    type Init = Init;
    type Input = Input;
    type Output = Output;

    view! {
        adw::StatusPage {
            set_icon_name: Some("folder-music-symbolic"),
            set_title: app::NAME,
            set_description: Some("Let's set up your library so you can start managing your collection."),

            adw::Clamp {
                gtk::ListBox {
                    add_css_class: "boxed-list-separate",

                    adw::ActionRow {
                        set_title: "Library Location",
                        set_subtitle: "Where your library will be stored.",
                        set_use_underline: true,
                        set_activatable_widget: Some(&library_directory),

                        add_suffix: library_directory = &gtk::Button {
                            set_valign: gtk::Align::Center,
                            connect_clicked => Input::SelectLibraryDirectory,

                            adw::ButtonContent {
                                set_can_shrink: true,
                                set_icon_name: "folder-symbolic",
                                set_label: "Sheet music",
                            },
                        },
                    },

                    adw::ButtonRow {
                        set_title: "Create library",
                        add_css_class: "suggested-action",
                        set_end_icon_name: Some("go-next-symbolic"),
                        set_hexpand: false,
                        connect_activated => Input::CreateLibrary,
                    },
                },
            },
        }
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = Setup {
            library_directory: init.default_library_directory,
            parent: init.parent,
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            Input::SelectLibraryDirectory => {
                let documents_directory = glib::user_special_dir(glib::UserDirectory::Documents)
                    .unwrap_or_else(|| glib::home_dir());

                let documents_directory = gio::File::for_path(&documents_directory);

                let selected_dir = unwrap_or_return!(
                    gtk::FileDialog::builder()
                        .modal(true)
                        .initial_folder(&documents_directory)
                        .build()
                        .select_folder_future(self.parent.as_ref())
                        .await
                );

                self.library_directory = selected_dir.uri();
            }

            Input::CreateLibrary => {
                let _ = sender.output(Output::CreateLibrary(self.library_directory.clone()));
            }
        }
    }
}
