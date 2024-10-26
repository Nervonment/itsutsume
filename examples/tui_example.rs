use std::{
    io::{stdout, Write},
    sync::mpsc::channel,
    thread,
};

use anyhow::Ok;
use crossterm::{
    cursor::{Hide, MoveToNextLine, MoveToRow},
    event::{Event, KeyCode, KeyEventKind},
    style::{Print, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use itsutsume::{
    agent::{Agent, MinimaxSearch},
    board::{complicated_eval::ComplicatedEvalBoard, Board, Side, HEIGHT, WIDTH},
    game::{DropResult, Game},
};

enum EventKind {
    Move((isize, isize)),
    Drop,
    Quit,
    Restart,
}

fn main() -> anyhow::Result<()> {
    let mut game = Game::<ComplicatedEvalBoard>::new();
    let mut player_pos = (WIDTH / 2, HEIGHT / 2);
    let mut agent_pos = None;
    let mut total_step = 0;
    let mut finished = false;
    let (tx, rx) = channel();

    thread::spawn(move || -> anyhow::Result<()> {
        loop {
            match crossterm::event::read()? {
                Event::Key(event) => {
                    if let KeyEventKind::Press = event.kind {
                        match event.code {
                            KeyCode::Up => tx.send(EventKind::Move((0, -1)))?,
                            KeyCode::Down => tx.send(EventKind::Move((0, 1)))?,
                            KeyCode::Left => tx.send(EventKind::Move((-1, 0)))?,
                            KeyCode::Right => tx.send(EventKind::Move((1, 0)))?,
                            KeyCode::Enter => tx.send(EventKind::Drop)?,
                            KeyCode::Esc => tx.send(EventKind::Quit)?,
                            KeyCode::Char('R') => tx.send(EventKind::Restart)?,
                            KeyCode::Char('r') => tx.send(EventKind::Restart)?,
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    });

    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(Hide)?;
    show_board(game.get_board(), player_pos, agent_pos)?;

    loop {
        match rx.recv()? {
            EventKind::Move((dc, dr)) => {
                let r = (player_pos.0 as isize + dr).max(0).min(HEIGHT as isize - 1) as usize;
                let c = (player_pos.1 as isize + dc).max(0).min(WIDTH as isize - 1) as usize;
                player_pos = (r, c);
                show_board(game.get_board(), player_pos, agent_pos)?;
                continue;
            }
            EventKind::Drop => {}
            EventKind::Quit => {
                stdout().execute(LeaveAlternateScreen)?;
                break;
            }
            EventKind::Restart => {
                game = Game::<ComplicatedEvalBoard>::new();
                player_pos = (WIDTH / 2, HEIGHT / 2);
                agent_pos = None;
                finished = false;
                total_step = 0;
                stdout().execute(Clear(ClearType::All))?;
                show_board(game.get_board(), player_pos, agent_pos)?;
                continue;
            }
        }

        if finished {
            continue;
        }

        let res = match game.drop(player_pos.0, player_pos.1) {
            std::result::Result::Ok(res) => res,
            Err(_) => continue,
        };
        total_step += 1;
        show_board(game.get_board(), player_pos, agent_pos)?;
        if show_res(&res)? {
            finished = true;
            continue;
        }

        let mut agent = MinimaxSearch::with_max_depth(if total_step > 50 { 3 } else { 4 });
        let (r, c) = agent.action(game.get_board(), itsutsume::board::Side::White);
        agent_pos = Some((r, c));
        let res = game.drop(r, c).unwrap();
        total_step += 1;
        show_board(game.get_board(), player_pos, agent_pos)?;
        if show_res(&res)? {
            finished = true;
        }
    }

    Ok(())
}

fn show_board<B: Board>(
    board: &B,
    player_pos: (usize, usize),
    agent_pos: Option<(usize, usize)>,
) -> anyhow::Result<()> {
    stdout().queue(MoveToRow(0))?;
    for r in 0..HEIGHT {
        for c in 0..WIDTH {
            let ch = match board.get_cell(r, c) {
                Side::Black => "⚫",
                Side::White => "⚪",
                Side::None => "➕",
            };
            if (r, c) == player_pos {
                stdout().queue(PrintStyledContent(ch.on_dark_cyan()))?;
            } else if agent_pos.is_some() && (r, c) == agent_pos.unwrap() {
                stdout().queue(PrintStyledContent(ch.on_dark_red()))?;
            } else {
                stdout().queue(Print(ch))?;
            }
        }
        stdout().queue(MoveToNextLine(1))?;
    }
    stdout().flush()?;
    Ok(())
}

fn show_res(res: &DropResult) -> anyhow::Result<bool> {
    match res {
        DropResult::BlackWins => {
            println!();
            println!("⚫ wins.");
            println!("Press R to restart or Esc to quit.");
            Ok(true)
        }
        DropResult::WhiteWins => {
            println!();
            println!("⚪ wins.");
            println!("Press R to restart or Esc to quit.");
            Ok(true)
        }
        _ => Ok(false),
    }
}
