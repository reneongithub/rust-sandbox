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
        Self::perform_on_sandbox(TestErrorHandling);

        let mut player = Player::new();
        Self::init(&mut player);
        Self::perform(&mut player);
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
        // player.add_game(Box::new(TestErrorHandling));
        // player.add_game(Box::new(TestMisc));
        player.add_game(Box::new(TestFlags));
        // player.add_game(Box::new(TestStringPointer));
        // player.add_game(Box::new(TestOrdering));
        // player.add_game(Box::new(TestGenericsConceptTry));
        // player.add_game(Box::new(TestGenericsConceptCollection));
    }
}
