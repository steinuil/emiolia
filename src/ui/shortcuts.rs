use adw::prelude::AdwDialogExt as _;
use relm4::{ComponentParts, ComponentSender, SimpleComponent};

#[derive(Debug)]
pub struct Shortcuts {
    dialog: adw::ShortcutsDialog,
    parent: adw::ApplicationWindow,
}

#[derive(Debug)]
pub enum Input {
    Present,
}

#[relm4::component(pub)]
impl SimpleComponent for Shortcuts {
    type Init = adw::ApplicationWindow;
    type Input = Input;
    type Output = ();

    view! {
        adw::ShortcutsDialog {
            add = adw::ShortcutsSection::new(Some("Basic")) {
                add = adw::ShortcutsItem::new("About", "F1") { },
                add = adw::ShortcutsItem::new("Preferences", "<Primary>comma") { },
            }
        }
    }

    fn init(
        parent: Self::Init,
        dialog: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Shortcuts {
            parent,
            dialog: dialog.clone(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Input::Present => self.dialog.present(Some(&self.parent)),
        }
    }
}
