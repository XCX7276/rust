//! Тестовая сцена: пол, ряд кубов и «солнце».
//! Нужна, чтобы было на что смотреть, пока нет настоящего мира из чанков.
//! Позже (этап 3 ТЗ) этот файл заменит мир из блоков.

use bevy::prelude::*;

/// Плагин тестовой сцены. `pub` — чтобы main.rs мог его подключить.
pub struct TestScenePlugin;

impl Plugin for TestScenePlugin {
    // build вызывается один раз при сборке приложения: регистрируем свои системы.
    fn build(&self, app: &mut App) {
        // Startup — система выполнится один раз при запуске игры.
        app.add_systems(Startup, spawn_test_scene);
    }
}

/// Система: создаёт пол, кубы и свет.
///
/// Параметры функции Bevy подставляет сам:
/// - `commands` — очередь «создать/удалить сущности»;
/// - `meshes` — хранилище 3D-форм (мешей), `ResMut` = ресурс, который можно менять;
/// - `materials` — хранилище материалов (цвет, текстура).
fn spawn_test_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Пол 20 × 20 метров. Plane3d — плоскость, Vec3::Y — «смотрит вверх».
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)))),
        MeshMaterial3d(materials.add(Color::srgb(0.35, 0.55, 0.30))), // зелёный
    ));

    // Один меш куба 1 × 1 × 1 м и два материала — их можно использовать много раз.
    // .clone() у Handle копирует только «ссылку», а не сам куб — это дёшево.
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let stone = materials.add(Color::srgb(0.55, 0.55, 0.60));
    let wood = materials.add(Color::srgb(0.55, 0.38, 0.20));

    // Ряд из 5 кубов перед камерой. x идёт от -2 до 2 включительно (..=).
    for x in -2..=2 {
        // чётные — камень, нечётные — дерево
        let material = if x % 2 == 0 {
            stone.clone()
        } else {
            wood.clone()
        };
        commands.spawn((
            Mesh3d(cube.clone()),
            MeshMaterial3d(material),
            // y = 0.5 — куб стоит НА полу (центр куба на высоте половины его размера)
            Transform::from_xyz(x as f32 * 2.0, 0.5, -3.0),
        ));
    }

    // Столбик из 3 кубов — чтобы проверить тени и высоту.
    for y in 0..3 {
        commands.spawn((
            Mesh3d(cube.clone()),
            MeshMaterial3d(stone.clone()),
            Transform::from_xyz(4.0, 0.5 + y as f32, 1.0),
        ));
    }

    // «Солнце» — направленный свет. Положение не важно, важно направление:
    // looking_at(Vec3::ZERO, ...) — светит из точки (4, 10, 3) в центр сцены.
    commands.spawn((
        DirectionalLight {
            illuminance: 8_000.0,      // яркость (люкс)
            shadow_maps_enabled: true, // включить тени
            ..default()
        },
        Transform::from_xyz(4.0, 10.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
