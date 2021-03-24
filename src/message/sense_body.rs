pub struct SenseBody {
    pub time: u8,
    pub view_mode: ViewMode,
    pub stamina: Stamina,
    pub speed: Speed,
    pub head_angle: f32,

    pub kick_count: i32,
    pub dash_count: i32,
    pub turn_count: i32,
    pub say_count: i32,
    pub turn_neck_count: i32,
    pub catch_count: i32,
    pub move_count: i32,
    pub change_view_count: i32,

    pub arm: Arm,
    pub focus: Focus,
    pub tackle: Tackle,
    pub collision: Collision,
    pub foul: Foul,
}

pub struct ViewMode {
    pub view_quality: ViewQuality,
    pub view_width: ViewWidth,
}

pub enum ViewQuality {
    High,
    Low,
}

pub enum ViewWidth {
    Narrow,
    Normal,
    Wide,
}

pub struct Stamina {
    pub stamina: i32,
    pub effort: f64,
    pub capacity: i32,
}

pub struct Speed {
    pub amount: f64,
    pub direction: f64,
}

pub struct Arm {
    pub movable_cycles: i32,
    pub expires_cycles: i32,
    pub target: Target,
    pub count: i32,
}

pub struct Target {
    pub value: String,
}

pub struct Focus {
    pub target: Target,
    pub count: i32,
}

pub struct Tackle {
    pub expires_cycles: i32,
    pub count: i32,
}

pub struct Collision {
    pub ball: bool,
    pub player: bool,
    pub post: bool,
}

pub struct Foul {
    pub charged: i32,
    pub card: Card,
}

pub enum Card {
    None,
    Red,
    Yellow
}

impl SenseBody {
    pub fn from(sexpr: lexpr::Value) -> SenseBody {
        SenseBody {
            time: 0,
            view_mode: ViewMode {
                view_quality: ViewQuality::High,
                view_width: ViewWidth::Narrow
            },
            stamina: Stamina {
                stamina: 0,
                effort: 0.0,
                capacity: 0
            },
            speed: Speed {
                amount: 0.0,
                direction: 0.0
            },
            head_angle: 0.0,
            kick_count: 0,
            dash_count: 0,
            turn_count: 0,
            say_count: 0,
            turn_neck_count: 0,
            catch_count: 0,
            move_count: 0,
            change_view_count: 0,
            arm: Arm {
                movable_cycles: 0,
                expires_cycles: 0,
                target: Target {
                    value: "".to_string()
                },
                count: 0
            },
            focus: Focus {
                target: Target {
                    value: "".to_string()
                },
                count: 0
            },
            tackle: Tackle {
                expires_cycles: 0,
                count: 0
            },
            collision: Collision {
                ball: false,
                player: false,
                post: false
            },
            foul: Foul {
                charged: 0,
                card: Card::None
            }
        }
    }
}