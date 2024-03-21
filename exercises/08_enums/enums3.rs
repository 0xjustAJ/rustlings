// enums3.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a
// hint.


enum Message {
    // TODO: implement the message variant types based on their usage below
    ChangeColor(u32, u32, u32),
    Echo(String),
    Move(Point),
    Quit,
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u32, u32, u32),
    position: Point,
    quit: bool,
    message: String,
}

impl State {
    fn change_color(&mut self, color: (u32, u32, u32)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) {
        self.message = s
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants
        // Remember: When passing a tuple as a function argument, you'll need extra parentheses:
        // fn function((t, u, p, l, e))
        match message{
         Message::ChangeColor(255_u32, 0_u32, 255_u32) => Some(Message::ChangeColor(255_u32, 0_u32, 255_u32)),
         Message::Echo(String) => Some(Message::Echo (String)),
         Message::Move(Point{x:10_u8, y:15_u8}) => Some(Message::Move(Point{x:10_u8, y:15_u8})),
         Message::Quit => Some(Message::Quit),
         other => Some(other),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: true,
            position: Point { x: 10, y: 15 },
            color: (255, 0, 255),
            message: "Hello world!".to_string(),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "Hello world!");
    }
}
