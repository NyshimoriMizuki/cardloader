use iced::widget::{button, column, container, row, text};
use iced::widget::{progress_bar, progress_bar::Style};
use iced::window::Settings;
use iced::{Center, Color, Element, Length::Fill, Size};
use rfd::FileDialog;

use serde::Deserialize;
use std::fs::read_to_string;
use std::path::PathBuf;

use iced::Background::Color as BgColor;

macro_rules! const_color {
    ($name:ident = $($field:ident : $val:expr),+) => {
        const $name: Color = Color {
            $($field: $val,)*
            a: 1.0,
        };
    };
}

/// Unit based on size of a character
const REM: f32 = 16.0;

const_color![ RED = r: 0.85, g: 0.25, b: 0.25 ];
const_color![ AMBER = r: 0.95, g: 0.65, b: 0.15 ];
const_color![ GREEN = r: 0.13, g: 0.65, b: 0.35 ];
const_color![ GREY = r: 0.87, g: 0.87, b: 0.87 ];

#[derive(Clone)]
enum Message {
    SearchFile,
}

#[derive(Deserialize)]
struct Item {
    name: String,
    rate: f32,
    description: String,
}
impl Default for Item {
    fn default() -> Self {
        Self {
            name: "<nome>".to_string(),
            rate: 0.,
            description: "<descrição>".to_string(),
        }
    }
}
impl Item {
    fn get_rate_color(&self) -> Color {
        if self.rate < 2.0 {
            RED
        } else if self.rate < 3.5 {
            AMBER
        } else {
            GREEN
        }
    }
}

struct App {
    loaded: Item,
}
impl App {
    fn boot() -> Self {
        Self {
            loaded: Item::default(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let rate_color = self.loaded.get_rate_color();
        let rate_bar = progress_bar(0.0..=5.0, self.loaded.rate)
            .style(move |_theme| Style {
                background: BgColor(GREY),
                bar: BgColor(rate_color),
                border: iced::Border::default(),
            })
            .length(Fill)
            .girth(1. * REM);

        container(
            column![
                row![
                    button("Abrir")
                        .on_press(Message::SearchFile)
                        .height(42)
                        .padding([1. / 2. * REM, 1. * REM]),
                    column!["Nota", rate_bar].spacing(4).width(Fill)
                ]
                .spacing(1. / 2. * REM)
                .align_y(Center)
                .width(Fill),
                text(&self.loaded.name)
                    .size(2. * REM)
                    .width(Fill)
                    .align_x(Center),
                text(&self.loaded.description).size(1. * REM).width(Fill),
            ]
            .spacing(2. / 3. * REM),
        )
        .padding(1. * REM)
        .align_x(Center)
        .center_x(Fill)
        .into()
    }

    fn update(&mut self, _msg: Message) {
        let samples_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("samples");

        let selected_file = FileDialog::new()
            .set_directory(samples_dir)
            .add_filter("Arquivos JSON", &["json"])
            .pick_file();
        if selected_file.is_none() {
            eprintln!("WARN: no file selected on FileDialog.");
            return;
        };
        self.load_item_from_file(selected_file.unwrap());
    }

    fn load_item_from_file(&mut self, filename: PathBuf) {
        let content = read_to_string(filename).unwrap_or("./samples/std.json".to_string());
        let mut pre_load = match serde_json::from_str(&content) {
            Ok(i) => i,
            Err(e) => {
                eprintln!("ERROR: {e}.");
                Item::default()
            }
        };
        pre_load.rate = pre_load.rate.clamp(0., 5.);

        self.loaded = pre_load;
    }
}

fn main() -> iced::Result {
    iced::application(App::boot, App::update, App::view)
        .title("CardLoader")
        .window(Settings {
            size: Size::new(300., 200.),
            resizable: false,
            ..Default::default()
        })
        .run()
}
