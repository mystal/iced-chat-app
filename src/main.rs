use iced::{scrollable, text_input, Button, Column, Container, Element, Length, Row, Rule, Sandbox, Scrollable, Settings, Text, TextInput};

struct ChatLine {
    text: String,
}

impl ChatLine {
    fn new(text: String) -> Self {
        Self {
            text,
        }
    }
}

struct ChatLog {
    lines: Vec<ChatLine>,
    scroll_state: scrollable::State,
}

impl ChatLog {
    fn new() -> Self {
        Self {
            lines: Vec::new(),
            scroll_state: scrollable::State::new(),
        }
    }
}

struct ChatInput {
    value: String,
    state: text_input::State,
}

impl ChatInput {
    fn new() -> Self {
        Self {
            value: String::new(),
            state: text_input::State::new(),
        }
    }
}

#[derive(Clone, Debug)]
enum ChatAppMessage {
    ChatTextChanged(String),
    ChatTextSubmitted,
}

struct ChatApp {
    chat_log: ChatLog,
    chat_input: ChatInput,
}

impl Sandbox for ChatApp {
    type Message = ChatAppMessage;

    fn new() -> Self {
        let mut chat_input = ChatInput::new();
        chat_input.state.focus();

        Self {
            chat_log: ChatLog::new(),
            chat_input,
        }
    }

    fn title(&self) -> String {
        "iced Chat App".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            ChatAppMessage::ChatTextChanged(v) => self.chat_input.value = v,
            ChatAppMessage::ChatTextSubmitted => {
                if !self.chat_input.value.trim().is_empty() {
                    self.chat_log.lines.push(ChatLine::new(self.chat_input.value.clone()));
                    self.chat_input.value.clear();
                }
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        // Display scrollable chat log history.
        let log = Column::with_children(
            self.chat_log.lines.iter()
                .map(|line| Text::new(&line.text).into())
                .collect()
        )
            .spacing(4);
        let log = Scrollable::new(&mut self.chat_log.scroll_state)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(log);

        // Text input to fill the chat log.
        let input = TextInput::new(&mut self.chat_input.state, "", &self.chat_input.value, ChatAppMessage::ChatTextChanged)
            .on_submit(ChatAppMessage::ChatTextSubmitted)
            .padding(4);
        // let input_button = Button::new
        let input_row = Row::new()
            .push(input);
            // .push(input_button);

        let content = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(log)
            .push(Rule::horizontal(20))
            .push(input_row);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(8)
            .center_x()
            .center_y()
            .into()
    }
}

fn main() -> iced::Result {
    ChatApp::run(Settings::default())
}
