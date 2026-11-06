use bevy::color::palettes::basic::*;
use bevy::math::VectorSpace;
use bevy::prelude::*;

pub struct UiPlugin;

const CIRCLE_RADIUS: f32 = 100.;
const CIRCLE_ITEM_SIZE: f32 = 50.;

const DECAY_RATE: f32 = 46.0;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct UiCircleIndex(pub f32);



fn rotate_sys(
    time: Res<Time>,
    index: Res<UiCircleIndex>,
    mut ui_parent: Single<&mut UiTransform, With<CircleParent>>) {
    let target = Rot2::turn_fraction(index.0 / 3.);
    if ui_parent.rotation == target {
        return
    }
    if ui_parent.rotation.angle_to(target).abs() < 0.001 {
        ui_parent.rotation = target;
    } else {
        ui_parent.rotation.smooth_nudge(&target, DECAY_RATE, time.delta_secs());
    }
}

fn ui_rotate_third(thirds: f32) -> impl Scene {
    bsn! {
        UiTransform::from_rotation(Rot2::turn_fraction(thirds / 3.0))
    }
}

fn circle_layout(thirds: f32, child: impl Scene) -> impl Scene {
    bsn! {
        #CircleLayoutRed
                Node {
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::Center,
                    width: px(0.),
                    height: px(0.),
                }
                @ui_rotate_third(thirds )
                Children [
                    @child
                ]
    }
}

fn circle_item(color: impl Into<Color>) -> impl Scene {
    bsn! {
        Node {
            position_type: PositionType::Absolute,
            top: px(-CIRCLE_RADIUS),
            left: px({-CIRCLE_ITEM_SIZE /2.}),
            width: px(CIRCLE_ITEM_SIZE),
            height: px(CIRCLE_ITEM_SIZE),
        }
        BackgroundColor(color)
    }
}

#[derive(Component, Default, Clone)]
pub struct CircleParent;

fn circle_parent() -> impl Scene {
    bsn! {
        CircleParent
        Node {}
        UiTransform::default()
        Children [
            @circle_layout(0., circle_item(RED))
            --
            @circle_layout(1., circle_item(GREEN))
            --
            @circle_layout(2., circle_item(BLUE))]
    }
}


#[derive(Resource, Deref, DerefMut)]
pub struct UiCooldownPercent(pub f32);
impl Default for UiCooldownPercent {
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Component, Default, Clone)]
pub struct UiCooldownDisplay;

fn cooldown_sys(cooldown_percent: Res<UiCooldownPercent>,
    mut cooldown_display: Single<(&mut BackgroundColor), With<UiCooldownDisplay>>) {
    cooldown_display.0 = Srgba::new(cooldown_percent.0, cooldown_percent.0, cooldown_percent.0, 1.).into();
}

fn ui_scene() -> impl Scene {
    bsn! {
        #RootUi
        Node {
            display: Display::Grid,
            width: percent(100.0),
            height: percent(100.0),
            grid_template_columns: RepeatedGridTrack::fr::<RepeatedGridTrack>(12, 1.0),
            grid_template_rows: RepeatedGridTrack::fr::<RepeatedGridTrack>(12, 1.0),
        }
        Children [
            UiCooldownDisplay
            Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                grid_column: GridPlacement::start_end(6, 8),
                grid_row: GridPlacement::start_end(10, 12),
            }
            BackgroundColor(BLACK)
            Children [
                @circle_parent()
            ]
        ]
    }
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiCircleIndex>();
        app.init_resource::<UiCooldownPercent>();
        app.add_systems(Startup, ui_scene.spawn());
        app.add_systems(Update, rotate_sys);
        app.add_systems(Update, cooldown_sys);
    }
}