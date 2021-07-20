#![windows_subsystem = "windows"]

use bindings::{
    Microsoft,
    Microsoft::UI::Xaml::{
        Application, ApplicationInitializationCallback, Window,LaunchActivatedEventArgs
    },
};

use windows::{implement};

#[implement(extend Microsoft::UI::Xaml::Application, override OnLaunched)]
struct App {
    _window: Option<Window>,
}

#[allow(non_snake_case)]
impl App {
    fn OnLaunched(&mut self, _: &Option<LaunchActivatedEventArgs>) -> windows::Result<()> {
        let window = Window::new()?;
        window.SetTitle("WinUI Desktop, Unpackaged (Rust)")?;
        window.Activate();
        println!("window is {:?}",self._window);
        windows::Result::Ok(())
    }
}

fn main() -> windows::Result<()> {
    windows_app::bootstrap::initialize().and_then(|_| {
        Application::Start(ApplicationInitializationCallback::new(|_| {
            App { _window: None }.new()?;
            Ok(())
        }))
    })
}
