const LED_ROW_SIZE: usize = 5;
const LED_COL_SIZE: usize = 5;

type LedMatrix = [[u8; LED_ROW_SIZE]; LED_COL_SIZE];

const LED_DIRECTION_N: LedMatrix = [
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

const LED_DIRECTION_E: LedMatrix = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 1, 1],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

const LED_DIRECTION_W: LedMatrix = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [1, 1, 1, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

const LED_DIRECTION_S: LedMatrix = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];

const LED_DIRECTION_NE: LedMatrix = [
    [0, 0, 0, 0, 1],
    [0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

const LED_DIRECTION_NW: LedMatrix = [
    [1, 0, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

const LED_DIRECTION_SW: LedMatrix = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0],
    [1, 0, 0, 0, 0],
];

const LED_DIRECTION_SE: LedMatrix = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 1, 0],
    [0, 0, 0, 0, 1],
];

pub enum Direction {
    North,
    East,
    West,
    South,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

pub fn get_led_matrix(direction: Direction) -> LedMatrix {
    match direction {
        Direction::North => LED_DIRECTION_N,
        Direction::East => LED_DIRECTION_E,
        Direction::West => LED_DIRECTION_W,
        Direction::South => LED_DIRECTION_S,
        Direction::NorthEast => LED_DIRECTION_NE,
        Direction::NorthWest => LED_DIRECTION_NW,
        Direction::SouthWest => LED_DIRECTION_SW,
        Direction::SouthEast => LED_DIRECTION_SE,
    }
}
