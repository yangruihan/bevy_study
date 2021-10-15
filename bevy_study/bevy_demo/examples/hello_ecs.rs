use bevy_study::bevy::{
    app::{AppExit, ScheduleRunnerPlugin, ScheduleRunnerSettings},
    ecs::schedule::ReportExecutionOrderAmbiguities,
    log::LogPlugin,
    prelude::*,
    utils::Duration,
};
use rand::random;

#[derive(Component)]
struct Player {
    name: String,
}

#[derive(Component)]
struct Score {
    value: usize,
}

#[derive(Default)]
struct GameState {
    current_round: usize,
    total_players: usize,
    winning_player: Option<String>,
}

struct GameRules {
    winning_score: usize,
    max_rounds: usize,
    max_players: usize,
}

fn print_message_system() {
    println!("This game is fun!");
}

fn new_round_system(game_rules: Res<GameRules>, mut game_state: ResMut<GameState>) {
    game_state.current_round += 1;
    println!(
        "Begin round {} of {}",
        game_state.current_round,
        game_rules.max_rounds
    );
}

fn score_system(mut query: Query<(&Player, &mut Score)>) {
    for (player, mut score) in query.iter_mut() {
        let scored_a_point = random::<bool>();
        if scored_a_point {
            score.value += 1;
            println!(
                "{} scored a point! Their score is: {}",
                player.name,
                score.value
            );
        } else {
            println!(
                "{} did not score a point! Their score is: {}",
                player.name,
                score.value
            );
        }
    }
}

fn score_check_system(
    game_rules: Res<GameRules>,
    mut game_state: ResMut<GameState>,
    query: Query<(&Player, &Score)>,
) {
    for (player, score) in query.iter() {
        if score.value == game_rules.winning_score {
            game_state.winning_player = Some(player.name.clone());
        }
    }
}

fn game_over_system(
    game_rules: Res<GameRules>,
    game_state: Res<GameState>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if let Some(ref player) = game_state.winning_player {
        println!("{} won the game!", player);
        app_exit_events.send(AppExit);
    } else if game_state.current_round == game_rules.max_rounds {
        println!("Ran out of rounds. Nobody wins!");
        app_exit_events.send(AppExit);
    }

    println!();
}

fn startup_system(mut commands: Commands, mut game_state: ResMut<GameState>) {
    commands.insert_resource(GameRules {
        max_rounds: 10,
        winning_score: 4,
        max_players: 4,
    });

    commands.spawn_batch(vec![
        (
            Player {
                name: "Alice".to_string(),
            },
            Score { value: 0 },
        ),
        (
            Player {
                name: "Bob".to_string(),
            },
            Score { value: 0 },
        ),
    ]);

    game_state.total_players = 2;
}

fn new_player_system(
    mut commands: Commands,
    game_rules: Res<GameRules>,
    mut game_state: ResMut<GameState>,
) {
    let add_new_player = random::<bool>();
    if add_new_player && game_state.total_players < game_rules.max_players {
        game_state.total_players += 1;
        commands.spawn_bundle((
            Player {
                name: format!("Player {}", game_state.total_players),
            },
            Score { value: 0 },
        ));

        println!("Player {} joined the game!", game_state.total_players);
    }
}

#[derive(Default)]
struct State {
    counter: usize,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
enum MyStage {
    BeforeRound,
    AfterRound,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum MyLabels {
    ScoreCheck,
}

fn main() {
    App::new()
        .insert_resource(State { counter: 0 })
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs(1)))
        .add_plugin(ScheduleRunnerPlugin::default())
        .init_resource::<GameState>()
        .add_startup_system(startup_system)
        .add_system(print_message_system)
        .add_system_to_stage(CoreStage::Update, score_system)
        .add_stage_before(
            CoreStage::Update,
            MyStage::BeforeRound,
            SystemStage::parallel(),
        )
        .add_stage_after(
            CoreStage::Update,
            MyStage::AfterRound,
            SystemStage::parallel(),
        )
        .add_system_to_stage(MyStage::BeforeRound, new_round_system)
        .add_system_to_stage(MyStage::BeforeRound, new_player_system)
        .add_system_to_stage(
            MyStage::AfterRound,
            score_check_system.label(MyLabels::ScoreCheck),
        )
        .add_system_to_stage(
            MyStage::AfterRound,
            game_over_system.after(MyLabels::ScoreCheck),
        )
        .add_plugin(LogPlugin::default())
        .insert_resource(ReportExecutionOrderAmbiguities)
        .run();
}