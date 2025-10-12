use std::{cell::RefCell, rc::Rc};

use pancurses::{
    endwin, has_colors, init_pair, initscr, noecho, start_color, ColorPair, Input, Window,
    COLOR_BLACK, COLOR_RED, COLOR_WHITE,
};

struct UIScreen<'a> {
    title: &'a str,
    window: Rc<RefCell<Window>>,
    border_color_pair: Option<ColorPair>,
}

impl<'a> UIScreen<'a> {
    fn get_border_color_pair(&self) -> Option<ColorPair> {
        self.border_color_pair
    }

    fn get_window(&self) -> Rc<RefCell<Window>> {
        return self.window.clone();
    }

    fn draw(&self) {
        let title = &self.title;
        self.draw_border(title);
    }

    fn draw_border(&self, title: &str) {
        match self.border_color_pair {
            Some(ColorPair(a)) => {
                self.window.borrow_mut().attron(ColorPair(a));
            }
            None => {}
        }
        // Use default border characters
        self.window.borrow_mut().border(0, 0, 0, 0, 0, 0, 0, 0);
        // Title on the top border
        self.window.borrow_mut().mvaddstr(0, 2, title);
        self.window.borrow_mut().refresh();
        match self.border_color_pair {
            Some(ColorPair(a)) => {
                self.window.borrow_mut().attroff(ColorPair(a));
            }
            None => {}
        }
    }
}

struct UIInputScreen<'a> {
    screen: UIScreen<'a>,
}

impl<'a> UIInputScreen<'a> {
    fn new(win: &Window) -> Self {
        let input_win = win
            .subwin(3, win.get_max_x(), win.get_max_y() - 3, 0)
            .unwrap();
        input_win.mv(1, 2); // after " Input "
        input_win.refresh();
        Self {
            screen: UIScreen {
                title: "Input",
                window: Rc::new(RefCell::new(input_win)),
                border_color_pair: None,
            },
        }
    }

    fn draw(&mut self, inp_text: Option<&str>) {
        self.screen.draw_border("Input");
        match inp_text {
            Some(s) => {
                let rc_win = self.screen.get_window();
                let win = rc_win.borrow();
                win.mv(1, 3);
                win.printw(s);
            }
            None => {}
        }
    }

    fn getch(&mut self) -> Option<Input> {
        let c = self.screen.get_window().borrow_mut().getch();
        // let screen = self.screen.get_window().borrow_mut();
        // match c {
        //     Some(Input::Character('\n')) => {
        //         // Show typed text in main window
        //         self.buf += &'\n'.to_string();

        //         // Clear input line
        //         let input_win: std::cell::Ref<'_, Window> = self.screen.get_window().borrow();
        //         input_win.clear();
        //         // draw_border(&input_win, " Input ");
        //         self.draw();
        //         input_win.mv(0, 9);
        //         input_win.refresh();
        //     }
        //     Some(Input::Character(c)) => {
        //         self.screen.get_window().borrow_mut().addch(c);
        //         self.screen.get_window().borrow_mut().refresh();
        //         self.buf += &(c.to_string());
        //     }
        //     Some(Input::KeyResize) => {
        //         // Simple exit on resize; you could recompute layout instead
        //         self.draw();
        //     }
        //     _ => {}
        c
        // }
    }
}
pub struct GameWindow<'a> {
    screen: Window,
    max_x: i32,
    max_y: i32,
    player_1_screen: UIScreen<'a>,
    player_2_screen: UIScreen<'a>,
    text_screen: UIScreen<'a>,
    input_screen: UIInputScreen<'a>,
    input_buffer: String,
}

pub enum InputResult {
    StringResult(String),
    NoneResult,
}

impl<'a> GameWindow<'a> {
    pub fn new() -> Self {
        let tmp_win = initscr();
        noecho();
        if has_colors() {
            start_color();
            init_pair(1, COLOR_WHITE, COLOR_BLACK);
            init_pair(2, COLOR_RED, COLOR_BLACK); // red text on black background
        }
        let (max_y, max_x) = tmp_win.get_max_yx();
        // Layout
        let top_height = max_y / 3;
        let input_height = 3; // single-line input
        let middle_height = max_y - top_height - input_height; // remaining space

        let half_width = max_x / 2;

        // Windows
        let player_1_window = tmp_win.subwin(top_height, half_width, 0, 0).unwrap();
        let player_2_window = tmp_win
            .subwin(top_height, max_x - half_width, 0, half_width)
            .unwrap();
        let text_win = tmp_win.subwin(middle_height, max_x, top_height, 0).unwrap();

        let player_1_screen = UIScreen {
            title: "Player 1",
            window: Rc::new(RefCell::new(player_1_window)),
            border_color_pair: None,
        };
        let player_2_screen = UIScreen {
            title: "Player 2",
            window: Rc::new(RefCell::new(player_2_window)),
            border_color_pair: None,
        };
        let text_screen = UIScreen {
            title: "Log",
            window: Rc::new(RefCell::new(text_win)),
            border_color_pair: None,
        };
        let input_screen = UIInputScreen::new(&tmp_win);

        Self {
            screen: tmp_win,
            max_x: max_x,
            max_y: max_y,
            player_1_screen: player_1_screen,
            player_2_screen: player_2_screen,
            text_screen: text_screen,
            input_screen: input_screen,
            input_buffer: String::new(),
        }
    }

    fn process_input(&mut self) -> InputResult {
        self.getch()
    }

    pub fn draw(&mut self) {
        let tmp_win = &self.screen;
        let (max_y, max_x) = tmp_win.get_max_yx();
        // Layout
        let top_height = max_y / 3;
        let input_height = 3; // single-line input
        let middle_height = max_y - top_height - input_height; // remaining space

        let half_width = max_x / 2;

        // Windows
        let player_1_window = tmp_win.subwin(top_height, half_width, 0, 0).unwrap();
        let player_2_window = tmp_win
            .subwin(top_height, max_x - half_width, 0, half_width)
            .unwrap();
        let text_win = tmp_win.subwin(middle_height, max_x, top_height, 0).unwrap();

        self.player_1_screen = UIScreen {
            title: "Player 1",
            window: Rc::new(RefCell::new(player_1_window)),
            border_color_pair: None,
        };
        self.player_2_screen = UIScreen {
            title: "Player 2",
            window: Rc::new(RefCell::new(player_2_window)),
            border_color_pair: None,
        };
        self.text_screen = UIScreen {
            title: "Log",
            window: Rc::new(RefCell::new(text_win)),
            border_color_pair: None,
        };
        self.input_screen = UIInputScreen::new(&tmp_win);

        self.player_1_screen.draw();
        self.player_2_screen.draw();
        self.text_screen.draw();
        self.input_screen.draw(Some(&self.input_buffer));
    }

    pub fn getch(&mut self) -> InputResult {
        let c = self.input_screen.getch();
        match c {
            Some(Input::Character('\n')) => {
                let res = InputResult::StringResult(self.input_buffer.clone());
                self.input_buffer = String::new();
                res
            }
            Some(Input::Character(e)) => {
                self.input_buffer += &e.to_string();
                InputResult::NoneResult
            }
            Some(Input::KeyResize) => {
                self.draw();
                InputResult::NoneResult
            }
            _ => InputResult::NoneResult,
        }
    }
}

impl Drop for GameWindow<'_> {
    fn drop(&mut self) {
        endwin();
    }
}

// pub fn init_window() {
//     let screen = GameWindow::new();

// screen.keypad(true);

// Layout
// let top_height = max_y / 3;
// let input_height = 3; // single-line input
// let middle_height = max_y - top_height - input_height; // remaining space

// let half_width = max_x / 2;

// // Windows
// let win1 = screen.subwin(top_height, half_width, 0, 0).unwrap();
// let win2 = screen
//     .subwin(top_height, max_x - half_width, 0, half_width)
//     .unwrap();
// let main_win = screen.subwin(middle_height, max_x, top_height, 0).unwrap();
// let input_win = screen
//     .subwin(input_height, max_x, max_y - input_height, 0)
//     .unwrap();

// screen.
// Borders + labels
// draw_border(&win1, " Window 1 ");
// draw_border(&win2, " Window 2 ");
// draw_border(&main_win, " Main ");
// draw_border(&input_win, " Input ");

// Input loop: type on the bottom line, Enter to print in main window, 'q' to quit
// }
