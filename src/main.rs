extern crate matrix_display;
use matrix_display::*;
use std::io::stdin;

#[derive(Copy, Clone)]
enum Figure {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    Empty,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Color {
    White,
    Black,
    Empty,
}

#[derive(Copy, Clone)]
enum MoveType {
    NormalMove,
    Capture,
    Castling,
    DoublePawn,
}

enum GameState {
    Mate,
    ChessMate,
    Normal,
    Draw,
}

struct State {
    table: [[(Color, Figure); 8]; 8],
    turn: Color,
    white_points: u32,
    black_points: u32,
    game_state: GameState,
}

struct FigureMove {
    move_type: MoveType,
    old_position: (usize, usize),
    new_position: (usize, usize),
}

fn other_color(color: Color) -> Color {
    match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
        Color::Empty => Color::Empty,
    }
}

fn init_table() -> State {
    let mut table = [[(Color::Empty, Figure::Empty); 8]; 8];
    let figure_order = [Figure::Rook, Figure::Knight, Figure::Bishop, Figure::Queen, Figure::King, Figure::Bishop, Figure::Knight, Figure::Rook];
    
    for (i, figure) in figure_order.iter().enumerate() {
        table[0][i] = (Color::Black, *figure);
        table[7][i] = (Color::White, *figure);
    } 

    table[1] = [(Color::Black, Figure::Pawn); 8];
    table[6] = [(Color::White, Figure::Pawn); 8];
    
    State {table, turn: Color::White, white_points: 0, black_points: 0, game_state: GameState::Normal}
}

fn get_points(state: &State, points: u32) -> (u32, u32) {
    let current_player = state.turn;
    match current_player {
        Color::White => {
            (state.white_points + points, state.black_points)
        },
        
        Color::Black => {
            (state.white_points, state.black_points + points)
        },
        
        Color::Empty => {
            (0, 0)
            // TODO error
        },
    }
}

fn play(state: State, figure_move: FigureMove) -> State {
    let move_type = figure_move.move_type;
    let mut table = state.table;
    
    let (old_x, old_y) = figure_move.old_position;
    let (new_x, new_y) = figure_move.new_position;
    let points = 0;

    match move_type {
        MoveType::NormalMove => {
            table[new_x][new_y] = table[old_x][old_y];
            table[old_x][old_y] = (Color::Empty, Figure::Empty);
            println!("{} {}, {} {}", old_x, old_y, new_x, new_y);
        },
        
        MoveType::Capture => {
            let captured_figure = table[new_x][new_y].1;
            table[new_x][new_y] = table[old_x][old_y];
            table[old_x][old_y] = (Color::Empty, Figure::Empty);
        },
        
        MoveType::Castling => {
            // TODO find rook in that direction
            // TODO find new king position
            // TODO fin new Rook position
        },

        MoveType::DoublePawn => {
            // TODO move pawn on new position
            // TODO set state for pawn that is in position of double step
        },
    };

    let (white_points, black_points) = get_points(&state, points);

    State {
        table,
        turn: other_color(state.turn),
        white_points,
        black_points,
        game_state: state.game_state //TODO
    }
}

fn possible_moves(state: &State) -> Vec<u32> {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec
}


fn get_symbol(cell: &(Color, Figure)) -> char {
    let figure = cell.1;
    let mut base = match figure {
        Figure::King => '♔', 
        Figure::Queen => '♕',
        Figure::Rook => '♖',
        Figure::Bishop => '♗',
        Figure::Knight => '♘',
        Figure::Pawn => '♙',
        Figure::Empty => ' ',
    } as u32;
    
    if cell.0 == Color::Black {
        base = base + 6;
    }

    std::char::from_u32(base).unwrap()
}


fn print_table(state: &State) {  
    // print!("\x1B[2J\x1B[1;1H");
    let format = Format::new(7, 3);
    let table = state.table;
    let ansi_fg = 22;
    let mut board = vec![];
    for (i, row) in table.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let symbol = get_symbol(cell);
            let mut ansi_bg = 0;
            if (i+j) % 2 == 0 {
                ansi_bg = 7;
            }
            let cell = cell::Cell::new(symbol, ansi_fg, ansi_bg);
            board.push(cell);
        }
    }
    let mut data = matrix::Matrix::new(8, board);
    let mut display = MatrixDisplay::new(&format, &mut data);
    display.print(&mut std::io::stdout(), &style::BordersStyle::None);
}

fn filed_converter(field: &str) -> Option<(usize, usize)> {
    println!("Tu sam: {}", field);
    if field.len() != 2 {
        println!("Praznina me zove: {}", field);    
        return None;
    }
    let mut chars = field.chars();
    let letter = (chars.next().unwrap() as u32) - 65;
    let number = 7 - ((chars.next().unwrap() as u32) - 49);

    if letter < 0 || letter >= 8 {
        return None;
    }

    if number < 0 || number >= 8 {
        return None;
    }

    Some((number as usize, letter as usize))
}


fn get_figure_move(input: &Vec<&str>) -> Option<FigureMove> {
    if input.len() != 2 {
        return None;
    }
    let from = filed_converter(input[0]);
    let to = filed_converter(input[1]);
    if let None = from {
        return None;
    }
    if let None = to {
        return None;
    }
    Some(FigureMove{move_type: MoveType::NormalMove, old_position: from.unwrap(), new_position: to.unwrap()})
    // check_move(from, to);
}


fn main() {
    let ai_color = Color::White; // TODO ask user for this
    let mut state = init_table();
    print_table(&state);
    
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed read");
        
        if input.contains("exit") {
            break;
        }
        
        let mut commands = input.trim().split("->");
        let vec: Vec<&str> = commands.collect();
        match get_figure_move(&vec) {
            Some(figure_move) => {
                state = play(state, figure_move);
            }
            None => {
                println!("Invalid input, expected FIELD->FIELD, eg. B1->C4 and valid move must be performed!");
                continue;
            }
        }

        print_table(&state);

        // println!("{} {}", vec[0], vec[1]);
        // print_table(&state);
        // println!("{} {}", from.unwrap(), to.unwrap());
    }

    // for figure_move in possible_moves(&state) {
    //     println!("{}", figure_move);
    // }

    
    // loop {
    //     state = play(state, ai_color);
    //     if state.end == true {
    //         break;
    //     }
    // }
}
