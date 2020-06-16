use criterion::black_box;
use std::mem::MaybeUninit;
use yarte::TemplateFixed;

pub fn big_table(b: &mut criterion::Bencher<'_>, size: &usize) {
    let mut table = Vec::with_capacity(*size);
    for _ in 0..*size {
        let mut inner = Vec::with_capacity(*size);
        for i in 0..*size {
            inner.push(i);
        }
        table.push(inner);
    }
    let t = BigTable { table };
    b.iter(|| {
        black_box(t.call(&mut [MaybeUninit::uninit(); 109915]).unwrap());
    });
}

#[derive(TemplateFixed)]
#[template(path = "big-table")]
struct BigTable {
    table: Vec<Vec<usize>>,
}

pub fn teams(b: &mut criterion::Bencher<'_>) {
    let t = Teams {
        year: 2015,
        teams: vec![
            Team {
                name: "Jiangsu".into(),

                score: 43,
            },
            Team {
                name: "Beijing".into(),
                score: 27,
            },
            Team {
                name: "Guangzhou".into(),
                score: 22,
            },
            Team {
                name: "Shandong".into(),
                score: 12,
            },
        ],
    };
    b.iter(|| {
        black_box(t.call(&mut [MaybeUninit::uninit(); 239]).unwrap());
    });
}

#[derive(TemplateFixed)]
#[template(path = "teams")]
struct Teams {
    year: u16,
    teams: Vec<Team>,
}

struct Team {
    name: String,
    score: u8,
}
