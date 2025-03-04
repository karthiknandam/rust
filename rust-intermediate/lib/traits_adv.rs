#[derive(Debug, Clone, Copy)]
enum VideoGame {
    Xbox,
    Ps5,
}

impl Game for VideoGame {
    fn print_game(&self) {
        println!("{:?}", self)
    }
}
#[derive(Debug, Clone, Copy)]
enum BoardGame {
    Chess,
    Carroms,
}

impl Game for BoardGame {
    fn print_game(&self) {
        println!("{:?}", self)
    }
}

trait Game {
    fn print_game(&self);
}

struct PlayingGame<T: Game> {
    game: T,
}

impl<T> PlayingGame<T>
where
    T: Game,
{
    fn play(&self) {
        self.game.print_game();
    }
}

pub fn traits_adv() {
    let video_gameing = PlayingGame {
        game: VideoGame::Ps5,
    };
    let board_game = PlayingGame {
        game: BoardGame::Carroms,
    };
    video_gameing.play();
    board_game.play();
}

// enum AdvTicket {
//     FrontRow,
//     MiddleRow,
//     BackRow,
// }
// trait Seat {
//     fn book(&self);
// }

// impl Seat for AdvTicket {
//     fn book(&self) {
//         println!("{:?}", self)
//     }
// }

// struct Ticket<T: Seat> {
//     location: T,
// }

// fn print_ticket<T: Seat>(ticket: Ticket<T>) {
//     ticket.location.book();
// }
