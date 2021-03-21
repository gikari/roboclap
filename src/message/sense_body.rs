pub struct SenseBody {
    time: u8,
    view_mode: ViewMode,
    stamina: Stamina,
    speed: Speed,
    head_angle: f32,

    kick_count: i32,
    dash_count: i32,
    turn_count: i32,
    say_count: i32,
    turn_neck_count: i32,
    catch_count: i32,
    move_count: i32,
    change_view_count: i32,

    arm: Arm,
    focus: Focus,
    tackle: Tackle,
    collision: Collision,
    foul: Foul,
}

struct ViewMode {
    view_quality: ViewQuality,
    view_width: ViewWidth,
}

enum ViewQuality {
    High,
    Low,
}

enum ViewWidth {
    Narrow,
    Normal,
    Wide,
}

struct Stamina {
    stamina: i32,
    effort: f64,
    capacity: i32,
}

struct Speed {
    amount: f64,
    direction: f64,
}

struct Arm {
    movable_cycles: i32,
    expires_cycles: i32,
    target: Target,
    count: i32,
}

struct Target {
    value: [u8; 20],
}

struct Focus {
    target: Target,
    count: i32,
}

struct Tackle {
    expires_cycles: i32,
    count: i32,
}

struct Collision {
    ball: bool,
    player: bool,
    post: bool,
}

struct Foul {
    charged: i32,
    card: Card,
}

enum Card {
    None,
    Red,
    Yellow
}
