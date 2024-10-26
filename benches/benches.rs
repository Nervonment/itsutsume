use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration,
};
use itsutsume::{
    agent::{Agent, MinimaxSearch},
    board::{complicated_eval::ComplicatedEvalBoard, Board, Side, ToBoardDisplay, HEIGHT, WIDTH},
    game::{DropResult, Game},
};

fn benchmark(c: &mut Criterion) {
    let mut agent = MinimaxSearch::with_max_depth(3);
    let mut game = Game::<ComplicatedEvalBoard>::new();
    for _ in 0..10 {
        let (r, c) = agent.action(game.get_board(), Side::Black);
        match game.drop(r, c).unwrap() {
            DropResult::Continue => {}
            _ => break,
        }

        let (r, c) = agent.action(game.get_board(), Side::White);
        match game.drop(r, c).unwrap() {
            DropResult::Continue => {}
            _ => break,
        }
    }
    println!("{}", game.get_board().display());

    let board = game
        .get_board()
        .board
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(|(c, _)| MinimaxSearch::has_pieces_nearby(&mut game.get_board().clone(), r, c))
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();

    for r in 0..HEIGHT {
        for c in 0..WIDTH {
            print!(
                "{}",
                match board[r][c] {
                    true => "✅",
                    false => "❌",
                }
            );
        }
        println!();
    }

    let (r0, c0) = agent.action(game.get_board(), Side::Black);
    let mut board = game.get_board().clone();
    c.bench_function("set and unset", |b| {
        b.iter(|| {
            board.set_cell(r0, c0, Side::Black);
            board.set_cell(r0, c0, Side::None);
        })
    });

    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group("action");
    group.plot_config(plot_config);

    for max_depth in 1..6 {
        group.bench_with_input(
            BenchmarkId::new("alpha-beta", max_depth),
            &max_depth,
            |b, d| {
                b.iter(|| {
                    let mut agent = MinimaxSearch::with_max_depth(*d);
                    agent.action(game.get_board(), Side::Black);
                })
            },
        );
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
