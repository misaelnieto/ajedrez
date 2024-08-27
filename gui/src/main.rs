mod board_widget {
    use iced::{Border, Element, Length, Rectangle, Shadow, Size};
    use iced::advanced::layout::{Layout, Node};
    use iced::advanced::{layout, renderer};
    use iced::advanced::renderer::Style;
    use iced::advanced::widget::{Tree, Widget};
    use iced::Color;
    use iced::mouse::Cursor;

    use ajedrez::{ChessBoard as AjedrezChessBoard};

    pub struct Board<'a> {
        board: &'a AjedrezChessBoard,
        width: f32,
        height: f32,
    }

    impl<'a> Board<'a> {
        pub fn new(board: &'a AjedrezChessBoard) -> Self {
            Board {
                board,
                width: 400.0,
                height: 400.0,
            }
        }
    }

    impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Board<'_>
        where
            Renderer: renderer::Renderer,
    {
        fn size(&self) -> Size<Length> {
            Size {
                width: Length::Fill,
                height: Length::Fill,
            }
        }

        fn layout(
            &self,
            _tree: &mut Tree,
            _renderer: &Renderer,
            _limits: &layout::Limits,
        ) -> Node {
            Node::new(Size::new(self.width, self.height))
        }

        fn draw(&self, _tree: &Tree, renderer: &mut Renderer, _theme: &Theme, _style: &Style, layout: Layout, _cursor: Cursor, _viewport: &Rectangle) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border:  Border::default(),
                    shadow:  Shadow::default(),
                },
                Color::BLACK
            );
        }
    }

    impl <'a, Message, Theme, Renderer> From<Board<'a>> for Element<'a, Message, Theme, Renderer> where Renderer: renderer::Renderer {
        fn from(board: Board) -> Element<Message, Theme, Renderer> {
            Element::new(board)
        }

    }
}

use iced::{Element, Error, Length, Sandbox, Settings};
use iced::Alignment;
use iced::widget::{column, container, text};
use ajedrez::ChessBoard as AjedrezChessBoard;

#[derive(Debug, Clone, Copy)]
enum Message {}

struct Chess {
    board: AjedrezChessBoard,
}

impl Chess {
    pub fn new() -> Self {
        Chess {
            board: AjedrezChessBoard::new(),
        }
    }
}

impl Sandbox for Chess {
    type Message = Message;

    fn new() -> Self {
        Chess::new()
    }

    fn title(&self) -> String {
        String::from("Ajedrez")
    }

    fn update(&mut self, _message: Self::Message) {
        println!("update")
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            board_widget::Board::new(&self.board),
            text(format!("The chess status here ...")),
        ]
            .padding(20)
            .spacing(20)
            .max_width(500)
            .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

pub fn main() -> iced::Result {
    Chess::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
