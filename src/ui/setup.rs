use std::path::{Path, PathBuf};

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
    parent: adw::ApplicationWindow,
    library_directory: Option<PathBuf>,
}

#[derive(Debug)]
pub enum Input {
    SelectLibraryDirectory,
    CreateLibrary,
}

#[derive(Debug)]
pub enum Output {
    CreateLibrary(PathBuf),
}

fn initial_folder() -> PathBuf {
    glib::user_special_dir(glib::UserDirectory::Documents).unwrap_or_else(|| glib::home_dir())
}

fn default_library_dir() -> Option<PathBuf> {
    let mut path = glib::user_special_dir(glib::UserDirectory::Documents)?;
    path.push("Sheet music");
    Some(path)
}

fn get_dir_name(dir: &Path) -> Option<String> {
    Some(dir.file_name()?.to_string_lossy().to_string())
}

#[relm4::component(pub async)]
impl SimpleAsyncComponent for Setup {
    type Init = adw::ApplicationWindow;
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
                                #[watch]
                                set_label: &model.library_directory.as_ref().and_then(|p| get_dir_name(p)).unwrap_or_else(|| "No directory selected".to_string()),
                            },
                        },
                    },

                    adw::ButtonRow {
                        set_title: "Create library",
                        add_css_class: "suggested-action",
                        set_end_icon_name: Some("go-next-symbolic"),
                        set_hexpand: false,
                        #[watch]
                        set_sensitive: model.library_directory.is_some(),
                        connect_activated => Input::CreateLibrary,
                    },
                },
            },
        }
    }

    async fn init(
        parent: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = Setup {
            library_directory: default_library_dir(),
            parent,
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            Input::SelectLibraryDirectory => {
                let documents_directory = gio::File::for_path(initial_folder());

                let selected_dir = unwrap_or_return!(
                    gtk::FileDialog::builder()
                        .modal(true)
                        .initial_folder(&documents_directory)
                        .build()
                        .select_folder_future(Some(&self.parent))
                        .await
                );
                let selected_dir = match selected_dir.path() {
                    Some(path) => path,
                    None => return,
                };

                self.library_directory = Some(selected_dir);
            }

            Input::CreateLibrary => {
                if let Some(directory) = &self.library_directory {
                    let _ = sender.output(Output::CreateLibrary(directory.clone()));
                }
            }
        }
    }
}
