//! Окно отладки: кадры в секунду, время кадра, координаты камеры.
//! F3 — показать / скрыть.

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, // замер FPS
    input::common_conditions::input_just_pressed,               // условие «клавишу нажали»
    prelude::*, window::PresentMode,
}; 
/// Компонент-метка: «это текст окна отладки».
/// Нужен, чтобы найти именно этот текст среди всех сущностей.
#[derive(Component)]
struct DebugText;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default()) // начинает считать FPS
            .add_systems(Startup, spawn_debug_text)
            .add_systems(
                Update,
                (
                    update_debug_text,
                    // run_if — запускать систему, только когда нажали F3
                    toggle_debug.run_if(input_just_pressed(KeyCode::F3)),
                ),
            );
    }
}

/// Создать текст в левом верхнем углу.
fn spawn_debug_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new("..."),
        TextFont {
            // Встроенный шрифт Bevy НЕ знает русских букв — грузим свой.
            // Путь — от папки assets: assets/fonts/DejaVuSans.ttf
            font: asset_server.load("fonts/DejaVuSans.ttf").into(),
            font_size: FontSize::Px(16.0),
            ..default()
        },
        // Node — «блок разметки»: где стоит текст на экране.
        Node {
            position_type: PositionType::Absolute,
            top: px(8),
            left: px(8),
            ..default()
        },
        DebugText,
    ));
}

/// Каждый кадр — обновить строку.
fn update_debug_text(
    diagnostics: Res<DiagnosticsStore>,
    camera: Single<&Transform, With<Camera3d>>,
    mut text: Single<&mut Text, With<DebugText>>,
) {
    // .get(...) может ничего не вернуть в первые кадры — тогда показываем 0.
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed())
        .unwrap_or(0.0);
    let ms = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|d| d.smoothed())
        .unwrap_or(0.0);
    let p = camera.translation;

    // text.0 — сама строка внутри Text.
    text.0 = format!(
        "Кадров в секунду: {fps:.0}\nВремя кадра: {ms:.1} мс\nКоординаты: X {:.1}  Y {:.1}  Z {:.1}\nF3 — скрыть",
        p.x, p.y, p.z
    );
}

/// Показать / скрыть окно отладки.
fn toggle_debug(mut visibility: Single<&mut Visibility, With<DebugText>>) {
    // ** — «достать значение» из Single и из ссылки на компонент.
    **visibility = match **visibility {
        Visibility::Hidden => Visibility::Visible,
        _ => Visibility::Hidden,
    };
}
