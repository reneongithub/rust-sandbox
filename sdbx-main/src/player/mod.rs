mod games;

use games::*;
use sdbx_commons::PerformOnSdbx;

pub struct Player {
    pub games: Vec<Box<dyn PerformOnSdbx>>,
}

impl Player {
    fn new() -> Self {
        Self { games: Vec::new() }
    }

    pub fn play() {
        let mut player = Player::new();
        Self::init(&mut player);
        Self::perform(&mut player);

        Self::perform_on_sandbox(TestErrorHandling {
            name: String::from("error handling test"),
        })
    }

    fn perform(player: &mut Player) {
        for game in &player.games {
            game.run();
        }
    }

    fn perform_on_sandbox<G>(game: G)
    where
        G: PerformOnSdbx,
    {
        game.run();
    }

    pub fn add_game(&mut self, game: Box<dyn PerformOnSdbx>) {
        self.games.push(game);
    }

    fn init(player: &mut Player) {
        player.add_game(Box::new(TestErrorHandling {
            name: String::from("error handling test"),
        }));
        player.add_game(Box::new(TestMisc {
            name: String::from("misc testing"),
        }));
        player.add_game(Box::new(TestStringPointer {
            name: String::from("pinter on string testing"),
        }));
        player.add_game(Box::new(TestOrdering {
            name: String::from("ordering testing"),
        }));
    }
}
