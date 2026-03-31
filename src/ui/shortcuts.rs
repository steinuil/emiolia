use adw::prelude::AdwDialogExt as _;
use relm4::{ComponentParts, ComponentSender, SimpleComponent};

#[derive(Debug)]
pub struct Shortcuts {
    dialog: adw::ShortcutsDialog,
    parent: adw::ApplicationWindow,
}

#[derive(Debug)]
pub struct Init {
    pub parent: adw::ApplicationWindow,
    pub items: Vec<(String, Vec<(String, String)>)>,
}

#[derive(Debug)]
pub enum Input {
    Present,
}

impl SimpleComponent for Shortcuts {
    type Init = Init;
    type Input = Input;
    type Output = ();
    type Root = adw::ShortcutsDialog;
    type Widgets = adw::ShortcutsDialog;

    fn init_root() -> Self::Root {
        adw::ShortcutsDialog::new()
    }

    fn init(
        init: Self::Init,
        dialog: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Shortcuts {
            parent: init.parent,
            dialog: dialog.clone(),
        };

        let widgets = dialog;

        for (section_name, items) in init.items {
            let section = adw::ShortcutsSection::new(Some(&section_name));

            for (name, shortcut) in items {
                section.add(adw::ShortcutsItem::new(&name, &shortcut));
            }

            widgets.add(section);
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Input::Present => {
                self.dialog.present(Some(&self.parent));
                self.dialog.focus();
            }
        }
    }
}
