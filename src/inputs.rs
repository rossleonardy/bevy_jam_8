use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

#[derive(Component, Default, Clone)]
struct InGame;

#[derive(Resource)]
struct FixedRuns(u32);

#[derive(InputAction)]
#[action_output(Vec2)]
struct Move;

#[derive(InputAction)]
#[action_output(bool)]
pub struct SwatLeft;

#[derive(InputAction)]
#[action_output(bool)]
pub struct SwatRight;

#[derive(Resource)]
pub struct AccumulatedMove(pub Vec2);

#[derive(Resource)]
pub enum AccumulatedLR {
    Left,
    Right,
    None
}

fn setup(mut commands: Commands) {
    commands.insert_resource(FixedRuns(0));
    commands.insert_resource(AccumulatedMove(Vec2::ZERO));
    commands.insert_resource(AccumulatedLR::None);

    commands.spawn_scene(bsn !{
        InGame
        Actions<InGame> [
            Action<Move>::new()
            DeadZone
            SmoothNudge
            Bindings [
                {preset::cardinal::wasd_keys()}
                --
                {preset::cardinal::arrows()}
                --
                {preset::axial::left_stick()}
            ]
            --
            Action<SwatLeft>::new()
            Bindings [
                Binding(GamepadButton::LeftTrigger)
                ~Press
                --
                Binding(KeyCode::KeyQ)
                ~Press
            ]
            --
            Action<SwatRight>::new()
            Bindings [
                Binding(GamepadButton::RightTrigger)
                ~Press
                --
                Binding(KeyCode::KeyE)
                ~Press
            ]
        ]
    });
}
fn reset_fixed_run(mut fixed_runs: ResMut<FixedRuns>) {
    fixed_runs.0 = 0;
}

fn inc_fixed_run(mut fixed_runs: ResMut<FixedRuns>) {
    fixed_runs.0 += 1;
}

fn fixed_runs_gt_zero(fixed_runs: Res<FixedRuns>) -> bool {
    fixed_runs.0 > 0
}

fn accumulate_move(
    move_action: Single<&Action<Move>>,
    mut move_dir: ResMut<AccumulatedMove>,
) {
    let dir = ***move_action;
    if dir != Vec2::ZERO {
        move_dir.0 += dir;
    }
}

fn accumulate_lr(
    left_action: Single<&Action<SwatLeft>>,
    right_action: Single<&Action<SwatRight>>,
    mut lr: ResMut<AccumulatedLR>,
) {
    let l: bool = ***left_action;
    let r: bool = ***right_action;
    if l {
        *lr = AccumulatedLR::Left;
    } else if r {
        *lr = AccumulatedLR::Right;
    }
}

fn clear_input(mut move_dir: ResMut<AccumulatedMove>, mut lr: ResMut<AccumulatedLR>) {
    move_dir.0 = Vec2::ZERO;
    *lr = AccumulatedLR::None;
}

pub struct InputsPlugin;
impl Plugin for InputsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EnhancedInputPlugin)
            .add_input_context::<InGame>()
            .add_systems(Startup, setup)
            .add_systems(PreUpdate, reset_fixed_run)
            .add_systems(FixedPreUpdate, inc_fixed_run)
            .add_systems(RunFixedMainLoop,
                         (
                         accumulate_move.in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
                         accumulate_lr.in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
                         clear_input
                             .run_if(fixed_runs_gt_zero)
                             .in_set(RunFixedMainLoopSystems::AfterFixedMainLoop)
                         ));
    }
}
