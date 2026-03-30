use adw::prelude::AdwDialogExt as _;
use relm4::{ComponentParts, ComponentSender, SimpleComponent};

#[derive(Debug)]
pub struct About {
    dialog: adw::AboutDialog,
    parent: adw::ApplicationWindow,
}

#[derive(Debug)]
pub enum Input {
    Present,
}

impl SimpleComponent for About {
    type Init = adw::ApplicationWindow;
    type Input = Input;
    type Output = ();
    type Root = adw::AboutDialog;
    type Widgets = adw::AboutDialog;

    fn init_root() -> Self::Root {
        let about = adw::AboutDialog::builder()
            .developer_name("steen")
            .license_type(gtk::License::Gpl30)
            .website("https://github.com/steinuil/gush")
            .issue_url("https://github.com/steinuil/gush/issues")
            .application_name("Gush")
            .version("0.1.0")
            .copyright("Copyright © 2026 steen")
            .can_close(true)
            .build();

        about
    }

    fn init(
        parent: Self::Init,
        dialog: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = About {
            parent,
            dialog: dialog.clone(),
        };

        let widgets = dialog;

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Input::Present => self.dialog.present(Some(&self.parent)),
        }
    }
}
