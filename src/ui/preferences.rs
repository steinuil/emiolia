use adw::prelude::{
    AdwDialogExt as _, PreferencesDialogExt as _, PreferencesGroupExt as _, PreferencesPageExt as _,
};
use relm4::{
    AsyncComponentSender,
    prelude::{AsyncComponentParts, SimpleAsyncComponent},
};

#[derive(Debug)]
pub struct Preferences {
    dialog: adw::PreferencesDialog,
    parent: adw::ApplicationWindow,
}

#[derive(Debug)]
pub enum Input {
    Present,
}

#[relm4::component(pub async)]
impl SimpleAsyncComponent for Preferences {
    type Init = adw::ApplicationWindow;
    type Input = Input;
    type Output = ();

    view! {
        adw::PreferencesDialog {
            add = &adw::PreferencesPage {
                set_title: "General",
                set_icon_name: Some("preferences-system-symbolic"),

                add = &adw::PreferencesGroup {
                    set_title: "Server",
                },
            },
        }
    }

    async fn init(
        parent: Self::Init,
        dialog: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = Preferences {
            dialog: dialog.clone(),
            parent,
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        match msg {
            Input::Present => self.dialog.present(Some(&self.parent)),
        }
    }
}
