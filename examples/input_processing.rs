use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<Action>::default())
        .add_systems(Startup, spawn_player)
        .add_systems(Update, check_data)
        .run();
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    Move,
    LookAround,
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands) {
    let mut input_map = InputMap::default();
    input_map
        .insert(
            Action::Move,
            VirtualDPad::wasd()
                // You can add a processor to handle axis-like user inputs by using the `with_processor`.
                //
                // This processor is a circular deadzone that normalizes input values
                // by clamping their magnitude to a maximum of 1.0,
                // excluding those with a magnitude less than 0.1,
                // and scaling other values linearly in between.
                .with_processor(CircleDeadZone::new(0.1))
                // Followed by appending Y-axis inversion for the next processing step.
                .with_processor(DualAxisInverted::ONLY_Y),
        )
        .insert(
            Action::Move,
            DualAxis::left_stick()
                // You can replace the currently used processor with another processor.
                .replace_processor(CircleDeadZone::default())
                // Or remove the processor directly, leaving no processor applied.
                .no_processor(),
        )
        .insert(
            Action::LookAround,
            // You can also add a pipeline to handle axis-like user inputs.
            DualAxis::mouse_motion().with_processor(
                DualAxisProcessingPipeline::default()
                    // The first processor is a circular deadzone.
                    .with(CircleDeadZone::new(0.1))
                    // The next processor doubles inputs normalized by the deadzone.
                    .with(DualAxisSensitivity::all(2.0)),
            ),
        );
    commands
        .spawn(InputManagerBundle::with_map(input_map))
        .insert(Player);
}

fn check_data(query: Query<&ActionState<Action>, With<Player>>) {
    let action_state = query.single();
    for action in action_state.get_pressed() {
        println!(
            "Pressed {action:?}! Its data: {:?}",
            action_state.axis_pair(&action)
        );
    }
}
