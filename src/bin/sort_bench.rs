use plotters::prelude::*;
use safe_dsa::sorting::{bubble_sort, merge_sort, quick_sort, selection_sort};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Instant;

type SortFn = fn(&mut [i32]);

struct SortSpec {
    name: &'static str,
    func: SortFn,
    max_size: usize,
}

const BASE_SIZES: &[usize] = &[
    16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16_384, 32_768, 65_536, 131_072, 262_144,
    524_288,
];
const REPEATS: usize = 5;

fn main() -> Result<(), Box<dyn Error>> {
    let specs = [
        SortSpec {
            name: "Bubble Sort",
            func: bubble_sort::sort::<i32>,
            max_size: 10_000,
        },
        SortSpec {
            name: "Selection Sort",
            func: selection_sort::sort::<i32>,
            max_size: 10_000,
        },
        SortSpec {
            name: "Merge Sort",
            func: merge_sort::sort::<i32>,
            max_size: 1_000_000,
        },
        SortSpec {
            name: "Quick Sort",
            func: quick_sort::sort::<i32>,
            max_size: 1_000_000,
        },
    ];

    let series: Vec<_> = specs
        .iter()
        .enumerate()
        .map(|(idx, spec)| {
            (
                spec.name,
                idx,
                benchmark_series(spec, 1_000 + idx as u64 * 97),
            )
        })
        .collect();

    let max_size = series
        .iter()
        .flat_map(|(_, _, data)| data.iter().map(|(n, _)| *n))
        .max()
        .unwrap_or(0);
    let max_time = series
        .iter()
        .flat_map(|(_, _, data)| data.iter().map(|(_, t)| *t))
        .fold(0.0_f64, f64::max)
        .max(1.0);

    let output_path = Path::new("benchmarks/sort_bench.png");
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let root = BitMapBackend::new(output_path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Sorting Performance", ("sans-serif", 36))
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(70)
        .build_cartesian_2d(0usize..max_size, 0f64..max_time)?;

    chart
        .configure_mesh()
        .x_desc("Input Size (elements)")
        .y_desc("Average Time (ms)")
        .label_style(("sans-serif", 20))
        .x_labels(10)
        .y_labels(10)
        .light_line_style(&WHITE.mix(0.15))
        .draw()?;

    for (name, palette_idx, data) in &series {
        let idx = *palette_idx;
        chart
            .draw_series(LineSeries::new(data.iter().copied(), Palette99::pick(idx)))?
            .label(*name)
            .legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 20, y)], Palette99::pick(idx))
            });
        chart.draw_series(
            data.iter()
                .map(|(n, t)| Circle::new((*n, *t), 4, Palette99::pick(idx).filled())),
        )?;
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .label_font(("sans-serif", 22))
        .draw()?;

    root.present()?;

    println!(
        "Benchmark complete. Plot saved to {}",
        output_path.display()
    );

    Ok(())
}

fn benchmark_series(spec: &SortSpec, seed: u64) -> Vec<(usize, f64)> {
    let sizes = BASE_SIZES
        .iter()
        .copied()
        .filter(|n| *n <= spec.max_size)
        .collect::<Vec<_>>();

    sizes
        .into_iter()
        .map(|n| {
            let mut total = 0.0_f64;
            for run in 0..REPEATS {
                let mut data = pseudo_random_vec(n, seed + run as u64 * 1_313);
                let start = Instant::now();
                (spec.func)(&mut data);
                total += start.elapsed().as_secs_f64() * 1_000.0;
            }
            let avg = total / REPEATS as f64;
            (n, avg)
        })
        .collect()
}

fn pseudo_random_vec(len: usize, mut seed: u64) -> Vec<i32> {
    let mut out = Vec::with_capacity(len);
    for _ in 0..len {
        seed ^= seed >> 12;
        seed ^= seed << 25;
        seed ^= seed >> 27;
        seed = seed.wrapping_mul(2_685_821_657_736_338_717);
        let value = (seed % 200_000) as i64 - 100_000;
        out.push(value as i32);
    }
    out
}
