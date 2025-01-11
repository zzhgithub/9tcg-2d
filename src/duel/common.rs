use crate::duel::main_duel::DuelMainAction;
use crate::utils::{BACKGROUND_COLOR, BORDER_COLOR_ACTIVE};
use bevy::asset::Handle;
use bevy::color::Color;
use bevy::hierarchy::{BuildChildren, ChildBuild, ChildBuilder};
use bevy::prelude::{
    AlignItems, BackgroundColor, BorderColor, Button, Component, FlexDirection, Font,
    JustifyContent, Node, Text, TextColor, TextFont, UiRect, Val, default,
};
use bevy::ui::FocusPolicy;
use bevy_simple_text_input::{
    TextInput, TextInputInactive, TextInputSettings, TextInputTextColor, TextInputTextFont,
    TextInputValue,
};

pub fn create_label_and_input<T>(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    label_text: &str,
    input_component: T,
) where
    T: Component,
{
    parent
        .spawn(
            (Node {
                width: Val::Percent(100.),
                height: Val::Px(50.),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                ..Default::default()
            }),
        )
        .with_children(|parent| {
            parent.spawn((Text::new(label_text), TextFont {
                font: font.clone(),
                font_size: 30.0,
                font_smoothing: Default::default(),
            }));
            //
            parent.spawn((
                Node {
                    width: Val::Px(200.0),
                    border: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                BorderColor(BORDER_COLOR_ACTIVE),
                BackgroundColor(BACKGROUND_COLOR),
                TextInput,
                TextInputTextFont(TextFont {
                    font_size: 20.,
                    ..default()
                }),
                TextInputSettings {
                    retain_on_submit: true,
                    ..default()
                },
                TextInputInactive(true),
                FocusPolicy::Block,
                TextInputValue("".to_string()),
                TextInputTextColor(TextColor(Color::BLACK)),
                input_component,
            ));
        });
}

pub fn spawn_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    background_color: Color,
    text: &str,
    action_component: DuelMainAction,
) {
    builder
        .spawn((
            Button,
            action_component,
            Node {
                width: Val::Percent(25.),
                height: Val::Px(50.),
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center, // 按钮文字居中
                align_items: AlignItems::Center,         // 垂直居中
                border: UiRect::all(Val::Px(2.0)),       // 白色边框
                ..Default::default()
            },
            BorderColor(Color::WHITE),
            BackgroundColor(background_color),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(text),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}
