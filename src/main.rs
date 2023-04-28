const TOP_CHAR: u8 = 72;
const BOTTOM_CHAR: u8 = 65;
const TOP_NUMBER: i8 = 8;
const BOTTOM_NUMBER: i8 = 1;
type FourDirections = (
    Position,
    bool,
    Position,
    bool,
    Position,
    bool,
    Position,
    bool,
);
type CheckedCoordinates = (u8, u8, i8, i8, bool, bool, bool, bool);
fn merge_vec<T: Clone>(vec_1: &[T], vec_2: &[T]) -> Vec<T> {
    let mut vec = Vec::from(vec_1);

    vec.extend_from_slice(vec_2);
    vec
}
trait HasMoves {
    fn moves(
        &self,
        enemy_positions: Vec<Position>,
        ally_positions: Vec<Position>,
        enemy_atacks: Vec<Position>,
    ) -> Vec<Position>;
    fn atacks(
        &self,
        enemy_positions: Vec<Position>,
        ally_positions: Vec<Position>,
        for_take: bool,
    ) -> Vec<Position>;
    fn possible_actions(
        &self,
        allies_positions: Vec<Position>,
        enemy_positions: Vec<Position>,
        enemy_attacks: Vec<Position>,
    ) -> Vec<Position>;
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position(char, i8);
impl Position {
    fn letter(&self) -> char {
        self.0
    }
    fn number(&self) -> i8 {
        self.1
    }
    fn check_letter(letter: u8) -> bool {
        (BOTTOM_CHAR..=TOP_CHAR).contains(&letter)
    }
    fn lineal_moves(&self, occupied: Vec<Position>, is_attacking: bool) -> Vec<Position> {
        let mut positions = Vec::new();

        let mut up_block = false;
        let mut down_block = false;
        let mut left_block = false;
        let mut right_block = false;

        for i in BOTTOM_NUMBER..TOP_NUMBER {
            let (right, can_right, left, can_left, down, can_down, up, can_up) =
                &self.lineals(i as u8);
            if *can_right && !right_block {
                right_block = right.check_occupied_blocks(&occupied);
                Position::push_if_not_blocked(&mut positions, right_block, *right, is_attacking);
            }
            if *can_left && !left_block {
                left_block = left.check_occupied_blocks(&occupied);
                Position::push_if_not_blocked(&mut positions, left_block, *left, is_attacking);
            }
            if *can_down && !down_block {
                down_block = down.check_occupied_blocks(&occupied);
                Position::push_if_not_blocked(&mut positions, down_block, *down, is_attacking);
            }
            if *can_up && !up_block {
                up_block = up.check_occupied_blocks(&occupied);
                Position::push_if_not_blocked(&mut positions, up_block, *up, is_attacking);
            }
        }
        positions
    }

    fn diagonal_moves(&self, occupied: Vec<Position>, is_attacking: bool) -> Vec<Position> {
        let mut positions = Vec::new();

        let mut right_up_block = false;
        let mut right_down_block = false;
        let mut left_up_block = false;
        let mut left_down_block = false;

        for i in BOTTOM_NUMBER..TOP_NUMBER {
            let (
                right_up,
                can_right_up,
                left_up,
                can_left_up,
                left_down,
                can_left_down,
                right_down,
                can_right_down,
            ) = &self.diagonals(i as u8);
            if *can_right_up && !right_up_block {
                right_up_block = right_up.check_occupied_blocks(&occupied);
                Position::push_if_not_blocked(
                    &mut positions,
                    right_up_block,
                    *right_up,
                    is_attacking,
                );
            }
            if *can_left_up && !left_up_block {
                left_up_block = left_up.check_occupied_blocks(&occupied);
                Position::push_if_not_blocked(
                    &mut positions,
                    left_up_block,
                    *left_up,
                    is_attacking,
                );
            }
            if *can_left_down && !left_down_block {
                left_down_block = left_down.check_occupied_blocks(&occupied);
                Position::push_if_not_blocked(
                    &mut positions,
                    left_down_block,
                    *left_down,
                    is_attacking,
                );
            }
            if *can_right_down && !right_down_block {
                right_down_block = right_down.check_occupied_blocks(&occupied);
                Position::push_if_not_blocked(
                    &mut positions,
                    right_down_block,
                    *right_down,
                    is_attacking,
                );
            }
        }
        positions
    }
    fn horse_moves(&self, allies_positions: Option<Vec<Position>>) -> Vec<Position> {
        let (
            up_letter_2,
            down_letter_2,
            up_number_2,
            down_number_2,
            can_right_2,
            can_left_2,
            can_up_2,
            can_down_2,
        ) = self.coordinates(2);
        let (up_letter, down_letter, up_number, down_number, can_right, can_left, can_up, can_down) =
            self.coordinates(1);
        let mut positions = Vec::new();
        if can_up_2 {
            if can_left {
                positions.push(Position(down_letter as char, up_number_2))
            }
            if can_right {
                positions.push(Position(up_letter as char, up_number_2))
            }
        }

        if can_down_2 {
            if can_left {
                positions.push(Position(down_letter as char, down_number_2))
            }
            if can_right {
                positions.push(Position(up_letter as char, down_number_2))
            }
        }

        if can_right_2 {
            if can_up {
                positions.push(Position(up_letter_2 as char, up_number))
            }
            if can_down {
                positions.push(Position(up_letter_2 as char, down_number))
            }
        }

        if can_left_2 {
            if can_up {
                positions.push(Position(down_letter_2 as char, up_number))
            }
            if can_down {
                positions.push(Position(down_letter_2 as char, down_number))
            }
        }
        match allies_positions {
            Some(allies) => positions
                .into_iter()
                .filter(|position| !allies.contains(position))
                .collect(),
            None => positions,
        }
    }
    fn king_moves(&self, allies_positions: Vec<Position>) -> Vec<Position> {
        let (up_letter, down_letter, up_number, down_number, can_right, can_left, can_up, can_down) =
            self.coordinates(1);
        let mut positions = Vec::new();
        if can_up {
            positions.push(Position(self.letter(), up_number));
            if can_left {
                positions.push(Position(down_letter as char, up_number));
            }
            if can_right {
                positions.push(Position(up_letter as char, up_number));
            }
        }

        if can_down {
            positions.push(Position(self.letter(), down_number));
            if can_left {
                positions.push(Position(down_letter as char, down_number));
            }
            if can_right {
                positions.push(Position(up_letter as char, down_number));
            }
        }

        if can_left {
            positions.push(Position(down_letter as char, self.number()));
        }
        if can_right {
            positions.push(Position(up_letter as char, self.number()));
        }
        positions
            .into_iter()
            .filter(|position| !allies_positions.contains(position))
            .collect()
    }
    fn queen_moves(
        &self,
        enemy_positions: Vec<Position>,
        allies_positions: Vec<Position>,
        is_attacking: bool,
    ) -> Vec<Position> {
        let mut positions =
            self.diagonal_moves(merge_vec(&enemy_positions, &allies_positions), false);
        positions.extend(self.lineal_moves(merge_vec(&enemy_positions, &allies_positions), false));
        positions
    }
    fn check_occupied_blocks(&self, occupied: &[Position]) -> bool {
        self.blocked(occupied)
    }
    fn push_if_not_blocked(
        positions: &mut Vec<Position>,
        its_blocked: bool,
        position: Position,
        is_attacking: bool,
    ) {
        if !its_blocked || is_attacking {
            positions.push(position);
        }
    }
    fn lineals(&self, span: u8) -> FourDirections {
        let (
            up_letter,
            down_letter,
            up_number,
            down_number,
            can_up_letter,
            can_down_letter,
            can_up_number,
            can_down_number,
        ) = self.coordinates(span);
        (
            Position(up_letter as char, self.number()),
            can_up_letter,
            Position(down_letter as char, self.number()),
            can_down_letter,
            Position(self.letter(), down_number),
            can_down_number,
            Position(self.letter(), up_number),
            can_up_number,
        )
    }
    fn diagonals(&self, span: u8) -> FourDirections {
        let (
            up_letter,
            down_letter,
            up_number,
            down_number,
            can_up_letter,
            can_down_letter,
            can_up_number,
            can_down_number,
        ) = self.coordinates(span);
        (
            Position(up_letter as char, up_number),
            can_up_letter && can_up_number,
            Position(down_letter as char, up_number),
            can_down_letter && can_up_number,
            Position(down_letter as char, down_number),
            can_down_letter && can_down_number,
            Position(up_letter as char, down_number),
            can_up_letter && can_down_number,
        )
    }
    fn coordinates(&self, span: u8) -> CheckedCoordinates {
        let up_letter = self.letter() as u8 + span;
        let down_letter = self.letter() as u8 - span;
        let up_number = self.number() + span as i8;
        let down_number = self.number() - span as i8;
        (
            up_letter,
            down_letter,
            up_number,
            down_number,
            up_letter <= TOP_CHAR,
            down_letter >= BOTTOM_CHAR,
            up_number <= TOP_NUMBER,
            down_number >= BOTTOM_NUMBER,
        )
    }
    fn blocked(&self, occupied: &[Position]) -> bool {
        occupied.iter().any(|position| self == position)
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Colour {
    White,
    Black,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceKind {
    Pawn,
    Horse,
    Bishop,
    Rock,
    Queen,
    King,
}

#[derive(Debug, Clone)]
pub struct Piece {
    pinned: bool,
    position: Position,
    kind: PieceKind,
    color: Colour,
    movements: Vec<Position>,
}
impl Piece {
    fn new(position: Position, kind: PieceKind, color: Colour) -> Piece {
        Piece {
            pinned: false,
            position,
            kind,
            color,
            movements: Vec::new(),
        }
    }
    fn can_promote(&self) -> bool {
        match self.kind {
            PieceKind::Pawn => {
                let number = if self.color == Colour::White { 8 } else { 1 };
                self.position.number() == number
            }
            _ => false,
        }
    }
    fn new_position(&mut self, new_position: Position, promoted: Option<PieceKind>) {
        self.movements.push(self.position);
        self.position = new_position;
        if let Some(new_kind) = promoted {
            self.promote(new_kind);
        }
    }
    fn promote(&mut self, kind: PieceKind) {
        if self.can_promote() {
            self.kind = kind;
        }
    }
    fn can_move(&self) -> bool {
        self.pinned || self.can_promote()
    }
    fn moved(&self) -> bool {
        !self.movements.is_empty()
    }
}
impl HasMoves for Piece {
    fn moves(
        &self,
        enemy_positions: Vec<Position>,
        allies_positions: Vec<Position>,
        enemy_atacks: Vec<Position>,
    ) -> Vec<Position> {
        if self.can_move() {
            return Vec::new();
        }
        match self.kind {
            PieceKind::Pawn => {
                let mut positions = Vec::new();
                let direction: i8 = if self.color == Colour::White { 1 } else { -1 };
                let move_pos = Position(self.position.letter(), self.position.number() + direction);
                if !enemy_positions.contains(&move_pos) {
                    positions.push(move_pos);
                }
                if !self.moved() {
                    positions.push(Position(
                        self.position.letter(),
                        self.position.number() + direction + direction,
                    ))
                }
                positions
            }
            PieceKind::Horse => self.position.horse_moves(Some(allies_positions)),
            PieceKind::Bishop => self
                .position
                .diagonal_moves(merge_vec(&enemy_positions, &allies_positions), false),
            PieceKind::Rock => self
                .position
                .lineal_moves(merge_vec(&enemy_positions, &allies_positions), false),
            PieceKind::Queen => self
                .position
                .queen_moves(enemy_positions, allies_positions, false),

            PieceKind::King => self
                .position
                .king_moves(allies_positions)
                .into_iter()
                .filter(|position| !enemy_atacks.contains(position))
                .collect(),
        }
    }
    fn atacks(
        &self,
        enemy_positions: Vec<Position>,
        allies_positions: Vec<Position>,
        for_take: bool,
    ) -> Vec<Position> {
        if self.can_move() {
            return Vec::new();
        }
        match self.kind {
            PieceKind::Pawn => {
                let direction: i8 = if self.color == Colour::White { 1 } else { -1 };
                let mut attacks = Vec::new();
                let left_letter = self.position.letter() as u8 - 1;
                let right_letter = self.position.letter() as u8 + 1;
                let can_left = Position::check_letter(left_letter);
                let can_right = Position::check_letter(right_letter);

                let left_position =
                    Position((left_letter) as char, self.position.number() + direction);
                if can_left && (!for_take || enemy_positions.contains(&left_position)) {
                    attacks.push(left_position);
                }
                let right_position =
                    Position((right_letter) as char, self.position.number() + direction);
                if can_right && (!for_take || enemy_positions.contains(&right_position)) {
                    attacks.push(right_position);
                }
                attacks
            }
            PieceKind::Horse => self.position.horse_moves(if for_take {
                Some(allies_positions)
            } else {
                None
            }),
            PieceKind::Bishop => self
                .position
                .diagonal_moves(merge_vec(&enemy_positions, &allies_positions), true),
            PieceKind::Rock => self
                .position
                .lineal_moves(merge_vec(&enemy_positions, &allies_positions), true),
            PieceKind::Queen => self
                .position
                .queen_moves(enemy_positions, allies_positions, true),
            PieceKind::King => self.position.king_moves(allies_positions),
        }
    }
    fn possible_actions(
        &self,
        allies_positions: Vec<Position>,
        enemy_positions: Vec<Position>,
        enemy_attacks: Vec<Position>,
    ) -> Vec<Position> {
        let moves = self.moves(
            enemy_positions.clone(),
            allies_positions.clone(),
            enemy_attacks,
        );
        let attacks = self
            .atacks(enemy_positions, allies_positions.clone(), true)
            .into_iter()
            .filter(|attack| !allies_positions.contains(attack))
            .collect::<Vec<Position>>();
        merge_vec(&moves, &attacks)
    }
}
pub struct Player {
    pieces: Vec<Piece>,
    dead_pieces: Vec<Piece>,
    color: Colour,
}
impl Player {
    fn new(color: Colour) -> Player {
        let back = match color {
            Colour::White => BOTTOM_NUMBER,
            Colour::Black => TOP_NUMBER,
        };
        let front = match color {
            Colour::White => BOTTOM_NUMBER + 1,
            Colour::Black => TOP_NUMBER - 1,
        };
        let mut letter = 'A' as u8;
        let mut pieces = Vec::with_capacity(16);

        while letter <= TOP_CHAR {
            let char_letter = letter as char;
            let back_position = Position(char_letter, back);
            let back_piece = match char_letter {
                'A' | 'H' => Piece::new(back_position, PieceKind::Rock, color),
                'B' | 'G' => Piece::new(back_position, PieceKind::Horse, color),
                'C' | 'F' => Piece::new(back_position, PieceKind::Bishop, color),
                'D' => Piece::new(back_position, PieceKind::Queen, color),
                _ => Piece::new(back_position, PieceKind::King, color),
            };
            // if color == Colour::White {
            pieces.push(Piece::new(
                Position(char_letter, front),
                PieceKind::Pawn,
                color,
            ));
            // }
            pieces.push(back_piece);
            letter += 1;
        }

        Player {
            pieces,
            color,
            dead_pieces: Vec::new(),
        }
    }
    fn possible_moves(&self, enemy: &Player) -> Vec<Position> {
        let enemy_positions = enemy.positions();
        self.pieces
            .iter()
            .flat_map(move |piece| {
                let allies_positions = self.filtred_positions(piece);
                let enemy_attacks = enemy.attacks(self.positions());
                piece.possible_actions(allies_positions, enemy_positions.clone(), enemy_attacks)
            })
            .collect()
    }
    fn filtred_positions(&self, piece: &Piece) -> Vec<Position> {
        self.pieces
            .iter()
            .filter_map(|piece_2| {
                if piece_2.position == piece.position {
                    None
                } else {
                    Some(piece_2.position)
                }
            })
            .collect()
    }
    fn positions(&self) -> Vec<Position> {
        self.pieces.iter().map(|piece| piece.position).collect()
    }
    fn attacks(&self, enemy_positions: Vec<Position>) -> Vec<Position> {
        self.pieces
            .iter()
            .flat_map(|piece| {
                piece.atacks(
                    enemy_positions.clone(),
                    self.filtred_positions(piece),
                    false,
                )
            })
            .collect()
    }
    fn king(&self) -> &Piece {
        self.pieces
            .iter()
            .find(|piece| piece.kind == PieceKind::King)
            .expect("Player should have always a King")
    }
    fn move_piece(
        &mut self,
        start_position: Position,
        new_position: Position,
        new_kind: Option<PieceKind>,
        enemy: &mut Player,
    ) {
        let self_positions = self.positions();
        let enemy_positions = enemy.positions();
        for piece in &mut self.pieces {
            if piece.position == start_position
                && piece
                    .possible_actions(
                        self_positions.clone(),
                        enemy_positions.clone(),
                        enemy.attacks(self_positions.clone()),
                    )
                    .contains(&new_position)
            {
                enemy.remove_piece(&new_position);
                piece.new_position(new_position, new_kind);
                break;
            }
        }
    }
    fn remove_piece(&mut self, position: &Position) {
        if let Some(piece) = self.pieces.iter().find(|piece| &piece.position == position) {
            self.dead_pieces.push(piece.clone());
            let pieces = &mut self.pieces.clone().into_iter();
            self.pieces = pieces.filter(|piece| &piece.position != position).collect();
        }
    }
    fn piece_by_position(&mut self, position: Position) -> Option<&mut Piece> {
        if let Some(index) = self
            .pieces
            .iter()
            .position(|piece| piece.position == position)
        {
            return Some(&mut self.pieces[index]);
        }
        None
    }
}

fn main() {
    let mut player_1 = Player::new(Colour::White);
    let mut player_2 = Player::new(Colour::Black);

    let moves = player_1.possible_moves(&player_2);
    println!("First options");
    println!("{:?}", moves);
    player_1.move_piece(Position('B', 1), Position('C', 3), None, &mut player_2);
    let moves = &player_1.possible_moves(&player_2);
    println!("After E4 options");
    println!("{:?}", moves);
    player_1.move_piece(Position('B', 2), Position('B', 4), None, &mut player_2);
    player_1.move_piece(Position('B', 4), Position('B', 5), None, &mut player_2);
    player_1.move_piece(Position('B', 5), Position('B', 6), None, &mut player_2);
    let moves = &player_1.possible_moves(&player_2);
    println!("Before take moves");
    println!("{:?}", moves);

    player_1.move_piece(Position('B', 6), Position('C', 7), None, &mut player_2);
    let moves = &player_1.possible_moves(&player_2);
    println!("On edge options");
    println!("{:?}", moves);
    println!("Pieces player 2");
    println!("{:?}", player_2.pieces);
    println!("{:?}", player_2.dead_pieces);
    let moves = &player_2.possible_moves(&player_1);
    println!("Moves player 2");
    println!("{:?}", moves);
}
