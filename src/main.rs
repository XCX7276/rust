//! ParodiaR — точка входа.
//!
//! Здесь ТОЛЬКО сборка игры из плагинов. Логики в этом файле нет:
//! каждая тема живёт в своём файле (модуле) и подключается строкой `mod имя;`.

mod camera; // файл src/camera.rs     — летающая камера (WASD + мышь)
mod debug; // файл src/debug.rs      — окно отладки F3
mod test_scene; // файл src/test_scene.rs — тестовая сцена: пол, кубы, солнце

use bevy::{prelude::*, window::PresentMode}; // всё нужное из Bevy + PresentMode (режим VSync)

fn main() {
    App::new()
        // DefaultPlugins — стандартный набор: окно, рендер, ввод, звук, ассеты.
        // .set(...) — донастроить один из плагинов набора.
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "ParodiaR".into(), // заголовок окна
                        present_mode: PresentMode::AutoNoVsync, // без VSync — только для замера FPS; для игры вернуть AutoVsync
                        ..default()               // остальные настройки — по умолчанию
                    }),
                    ..default()
                })
                // текстуры без размытия — для пиксель-арта
                .set(ImagePlugin::default_nearest()),
        )
        // Наши плагины. Формат: имя_файла::ИмяПлагина
        .add_plugins((
            test_scene::TestScenePlugin,
            camera::CameraPlugin,
            debug::DebugPlugin,
        ))
        .run();
}
