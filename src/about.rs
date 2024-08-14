use relm4::prelude::*;

use crate::*;

#[derive(Debug)]
pub struct AboutDialog {
    visible: bool,
}

#[derive(Debug)]
pub enum AboutDialogMsg {
    Show,
    Hide,
}

#[relm4::component(pub)]
impl SimpleComponent for AboutDialog {
    type Init = ();
    type Input = AboutDialogMsg;
    type Output = ();

    view! {
        adw::AboutWindow {
            set_application_name: "Video Player",
            set_application_icon: "media-playback-start",
            set_developers: &[
                "Dylan Donnell https://github.com/dy-tea"
            ],
            set_license_type: gtk::License::Gpl30,

            #[watch]
            set_visible: model.visible,

            connect_close_request[sender] => move |_| {
                sender.input(AboutDialogMsg::Hide);
                gtk::glib::Propagation::Proceed
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self { visible: false };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AboutDialogMsg::Show => {
                self.visible = true;
            }
            AboutDialogMsg::Hide => {
                self.visible = false;
            }
        }
    }
}
