//! Летающая камера (режим наблюдателя).
//!
//! Управление:
//! - ЛКМ — захватить мышь (курсор пропадёт), Esc — отпустить;
//! - мышь — осмотреться;
//! - W A S D — вперёд / влево / назад / вправо (работает на любой раскладке);
//! - Пробел — вверх, левый Shift — вниз, левый Ctrl — быстрее.

use std::f32::consts::FRAC_PI_2; // число π/2 (90° в радианах)

use bevy::{
    input::mouse::AccumulatedMouseMotion, // насколько сдвинулась мышь за кадр
    prelude::*, window::PresentMode,
    window::{CursorGrabMode, CursorOptions}, // захват курсора
};

/// Компонент: «эта сущность — летающая камера» + её настройки.
/// `#[derive(Component)]` — разрешает прикрепить структуру к сущности.
#[derive(Component)]
pub struct FlyCamera {
    pub speed: f32,        // метров в секунду
    pub sensitivity: Vec2, // чувствительность мыши по X (влево-вправо) и Y (вверх-вниз)
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            // Update — каждый кадр. .chain() — строго по порядку:
            // сначала захват мыши, потом поворот, потом движение.
            .add_systems(Update, (grab_mouse, look_around, fly).chain());
    }
}

/// Создать камеру: стоит в точке (0, 2, 6) и смотрит на ряд кубов.
fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 6.0).looking_at(Vec3::new(0.0, 0.5, -3.0), Vec3::Y),
        FlyCamera {
            speed: 6.0,
            sensitivity: Vec2::new(0.003, 0.002),
        },
    ));
}

/// ЛКМ — захватить курсор, Esc — отпустить.
///
/// `Single<&mut CursorOptions>` — «дай ровно один такой компонент» (он у окна игры).
/// `Res<ButtonInput<...>>` — состояние кнопок мыши и клавиш.
fn grab_mouse(
    mut cursor: Single<&mut CursorOptions>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        cursor.visible = false;
        cursor.grab_mode = CursorGrabMode::Locked; // курсор заперт в окне
    }
    if keys.just_pressed(KeyCode::Escape) {
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None; // отпустить
    }
}

/// Поворот камеры мышью.
fn look_around(
    motion: Res<AccumulatedMouseMotion>,
    cursor: Single<&CursorOptions>,
    camera: Single<(&mut Transform, &FlyCamera)>,
) {
    // Мышь не захвачена или не двигалась — ничего не делаем.
    if cursor.grab_mode == CursorGrabMode::None || motion.delta == Vec2::ZERO {
        return;
    }

    // into_inner() — достать из Single сами компоненты.
    let (mut transform, fly) = camera.into_inner();

    // Текущий поворот → три угла: рыскание (влево-вправо), тангаж (вверх-вниз), крен.
    let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

    // Мышь НЕ умножаем на время кадра: delta — уже весь сдвиг за кадр.
    let yaw = yaw - motion.delta.x * fly.sensitivity.x;

    // Не даём смотреть ровно вверх или вниз — там поворот «переворачивается».
    const LIMIT: f32 = FRAC_PI_2 - 0.01;
    let pitch = (pitch - motion.delta.y * fly.sensitivity.y).clamp(-LIMIT, LIMIT);

    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
}

/// Движение камеры клавишами.
fn fly(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    camera: Single<(&mut Transform, &FlyCamera)>,
) {
    let (mut transform, fly) = camera.into_inner();

    // «Вперёд» и «вправо» без наклона: летим горизонтально, куда смотрим.
    let forward = Vec3::new(transform.forward().x, 0.0, transform.forward().z).normalize_or_zero();
    let right = Vec3::new(transform.right().x, 0.0, transform.right().z).normalize_or_zero();

    // Складываем направления от всех нажатых клавиш.
    // KeyCode — МЕСТО клавиши: KeyW = «Ц» на русской раскладке.
    let mut direction = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        direction += forward;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction -= forward;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += right;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction -= right;
    }
    if keys.pressed(KeyCode::Space) {
        direction += Vec3::Y;
    }
    if keys.pressed(KeyCode::ShiftLeft) {
        direction -= Vec3::Y;
    }

    let boost = if keys.pressed(KeyCode::ControlLeft) {
        4.0
    } else {
        1.0
    };

    // normalize_or_zero — чтобы по диагонали (W + D) не лететь быстрее.
    // * time.delta_secs() — скорость одинаковая при любом FPS.
    transform.translation += direction.normalize_or_zero() * fly.speed * boost * time.delta_secs();
}
