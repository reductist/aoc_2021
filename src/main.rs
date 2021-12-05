#![feature(array_windows)]
pub mod day1;
pub mod day2;
pub mod day3;

use tabled::{Tabled, Table, Format, Modify, Column, Row, Header, Style, Alignment};

#[derive(Tabled)]
struct Answer {
    part: &'static str,
    result: String,
}

struct Answers {
    day: usize,
    data: Vec<Answer>
}

impl Answers {
    fn new(day: usize) -> Answers {
        Answers{ day, data: Vec::new() }
    }
}

impl Answers {
    fn add(&mut self, ans: Answer) -> &Answers {
        self.data.push(ans);
        return self
    }
}

impl Answers {
    fn out(self) {
        println!("{}", build_ans_tbl(self.day, self.data));
    }
}

fn build_ans_tbl(day: usize, ans: Vec<Answer>) -> String {
    let hdr_fmt = format!("Day {}", day.to_string());
    let tbl = Table::new(&ans)
        .with(Header(hdr_fmt))
        .with(Modify::new(Row(..1)).with(Alignment::center_horizontal()))
        .with(Modify::new(Row(..1)).with(Format(|s| format!("{}", s))))
        .with(Modify::new(Column(3..)).with(Format(|s| s.to_string())))
        .with(Style::pseudo())
        .to_string();

    return tbl;
}

fn main() {
    // -- day 1 --
    let mut d1_ans = Answers::new(1);
    // day1a
    let n_inc = day1::solve_a().to_string();
    d1_ans.add(Answer{ part: "a", result: n_inc, });

    // day1b
    let n_sum_inc = day1::solve_b().to_string();
    d1_ans.add(Answer{ part: "b", result: n_sum_inc, });

    d1_ans.out();

    // -- day 2 --
    let mut d2_ans = Answers::new(2);
    // day2a
    let pos_a = day2::solve_a().to_string();
    d2_ans.add(Answer{ part: "a", result: pos_a, });

    // day2b
    let pos_b = day2::solve_b().to_string();
    d2_ans.add(Answer{ part: "b", result: pos_b, });

    d2_ans.out();

    // -- day 3 --
    let mut d3_ans = Answers::new(3);
    // day 3a
    let pwr = day3::solve_a().to_string();
    d3_ans.add(Answer{ part: "a", result: pwr });

    // day 3b
    let lf_supp = day3::solve_b().to_string();
    d3_ans.add(Answer{ part: "b", result: lf_supp });

    d3_ans.out()
}