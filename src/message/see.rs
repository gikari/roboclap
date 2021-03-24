pub struct See {
    pub time: u8,
    pub objects: Vec<ObjectInfo>,
}

pub struct ObjectInfo {
    pub message_type: MessageType,
    pub visual_object: VisualObject,
    pub distance: f64,
    pub direction: f64,
    pub distance_change: f64,
    pub direction_change: f64,
    pub body_facing_direction: f64,
    pub head_facing_direction: f64,
    pub point_direction: f64,
    pub is_tackling: bool,
    pub kicked_the_ball: bool,
}

pub enum MessageType {
    Undefined,
    WithFacingDirs,
    WithChanges,
    DirectionAndDistance,
    DirectionOnly,
}

pub enum VisualObject {
    Player(Player),
    Ball(Ball),
    Flag(Flag),
    Goal(Goal),
    Line(Line),
}

pub struct Player {
    pub team_name: Option<String>,
    pub uniform_number: Option<u8>,
    pub is_goalie: Option<bool>,
}

pub struct Ball {}

pub struct Flag {
    pub hash: String,
}

pub struct Goal {
    pub side: GoalSide,
}

pub enum GoalSide {
    Left,
    Right,
}

pub struct Line {
    pub side: LineSide,
}

pub enum LineSide {
    Top,
    Right,
    Bottom,
    Left,
}

impl See {
    pub fn from(sexpr: lexpr::Value) -> See {
        See {
            time: 0,
            objects: vec![]
        }
    }
}