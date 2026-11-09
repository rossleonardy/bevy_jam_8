use crate::physics::Cooldown;
use crate::player::{BlueSkill, ColorSkills, GreenSkill, RedSkill};
use bevy::color::palettes::basic::*;
use bevy::prelude::*;

pub struct UiPlugin;

const CIRCLE_RADIUS: f32 = 100.;
const CIRCLE_ITEM_SIZE: f32 = 50.;

const DECAY_RATE: f32 = 46.0;

fn rotate_sys(
    time: Res<Time>,
    color_skills: Single<&ColorSkills>,
    mut ui_parent: Single<&mut UiTransform, With<CircleParent>>,
) {
    let f: f32 = color_skills.0 as f32;
    let target = Rot2::turn_fraction(-f / 3.);
    if ui_parent.rotation == target {
        return;
    }
    if ui_parent.rotation.angle_to(target).abs() < 0.001 {
        ui_parent.rotation = target;
    } else {
        ui_parent
            .rotation
            .smooth_nudge(&target, DECAY_RATE, time.delta_secs());
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

fn circle_item_position() -> impl Scene {
    bsn! {
        Node {
            position_type: PositionType::Absolute,
            top: px(-CIRCLE_RADIUS),
            left: px(-CIRCLE_ITEM_SIZE /2. ),
            width: px(CIRCLE_ITEM_SIZE),
            height: px(CIRCLE_ITEM_SIZE),
        }
    }
}

#[derive(Component, Default, Clone)]
pub struct HotbarItem;

fn circle_item_red() -> impl Scene {
    bsn! {
        @circle_item_position()
        RedSkill
        HotbarItem
        BackgroundColor(RED)
        Outline {
            width: px(1.),
            color: Color::NONE,
        }
    }
}

fn circle_item_green() -> impl Scene {
    bsn! {
        @circle_item_position()
        GreenSkill
        HotbarItem
        BackgroundColor(GREEN)
        Outline {
            width: px(1.),
            color: Color::NONE,
        }
    }
}

fn circle_item_blue() -> impl Scene {
    bsn! {
        @circle_item_position()
        BlueSkill
        HotbarItem
        BackgroundColor(BLUE)
        Outline {
            width: px(1.),
            color: Color::NONE,
        }
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
            @circle_layout(0., circle_item_red())
            --
            @circle_layout(1., circle_item_green())
            --
            @circle_layout(2., circle_item_blue())]
    }
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

fn red_skill_update(
    red_cooldown: Single<&Cooldown, With<RedSkill>>,
    red_ui: Single<(&mut BackgroundColor, &mut Outline), With<RedSkill>>,
) {
    let (mut background, mut outline) = red_ui.into_inner();
    let f = if red_cooldown.0.fraction() == 1.0 {
        1.0
    } else {
        red_cooldown.0.fraction() * 0.5
    };
    background.0 = Color::srgb(f, 0.0, 0.0);
    outline.color = if red_cooldown.0.is_finished() {
        Color::WHITE
    } else {
        Color::NONE
    };
}

fn green_skill_update(
    green_cooldown: Single<&Cooldown, With<GreenSkill>>,
    green_ui: Single<(&mut BackgroundColor, &mut Outline), With<GreenSkill>>,
) {
    let (mut background, mut outline) = green_ui.into_inner();
    let f = if green_cooldown.0.fraction() == 1.0 {
        1.0
    } else {
        green_cooldown.0.fraction() * 0.5
    };
    background.0 = Color::srgb(0.0, f, 0.0);
    outline.color = if green_cooldown.0.is_finished() {
        Color::WHITE
    } else {
        Color::NONE
    };
}

fn blue_skill_update(
    blue_cooldown: Single<&Cooldown, With<BlueSkill>>,
    blue_ui: Single<(&mut BackgroundColor, &mut Outline), With<BlueSkill>>,
) {
    let (mut background, mut outline) = blue_ui.into_inner();
    let f = if blue_cooldown.0.fraction() == 1.0 {
        1.0
    } else {
        blue_cooldown.0.fraction() * 0.5
    };
    background.0 = Color::srgb(0.0, 0.0, f);
    outline.color = if blue_cooldown.0.is_finished() {
        Color::WHITE
    } else {
        Color::NONE
    };
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui_scene.spawn());
        app.add_systems(
            Update,
            (red_skill_update, green_skill_update, blue_skill_update),
        );
        app.add_systems(Update, rotate_sys);
    }
}
