use pyo3::prelude::*;
use std::path::PathBuf;

use rnapkin::draw::{self, colors::ColorTheme, Mirror};
use rnapkin::forest;
use rnapkin::rnamanip::{self, Nucleotide};
use rnapkin::utils::ParsedInput;

struct Args {
    input: Option<String>,
    theme: String,
    angle: f64,
    bgopacity: Option<f64>,
    my: bool,
    mx: bool,
    height: u32,
}

impl Args {
    fn new(input: String, theme: String, angle: f64, bgopacity: f64, my: bool, mx: bool) -> Self {
        Args {
            input: Some(input),
            theme,
            angle,
            bgopacity: Some(bgopacity),
            my,
            mx,
            height: 900,
        }
    }
}

/// Creates SVG from newline separated fold and (optionally) RNA seq and highlights
/// SVG code is returned as a string, no file is created; You're free to do that on your own
/// themes: dark, bright, white, black
/// angle: rotation of the drawing in degrees
/// bgopacity: background opacity
/// my: y mirror or horizontal flip
/// mx: x mirror or vertical flip
#[pyfunction]
fn rna2svg(
    rna: String,
    theme: String,
    angle: f64,
    bgopacity: f64,
    my: bool,
    mx: bool,
) -> PyResult<String> {
    let args = Args::new(rna, theme, angle, bgopacity, my, mx);
    let text = args.input.unwrap();
    let mut lineiter = text.split("\n").map(|x| x.to_string());
    let pi = ParsedInput::parse(&mut lineiter).unwrap();

    let filename = PathBuf::from("o.x");

    let mut theme = match args.theme.as_ref() {
        "dark" => ColorTheme::dark(),
        "white" | "w" => ColorTheme::white(),
        "black" | "b" => ColorTheme::black(),
        "bright" => ColorTheme::bright(),
        _ => {
            eprintln!(
                "theme: \"{}\" not recognized!\nfalling back to default",
                args.theme
            );
            ColorTheme::default()
        }
    };

    if let Some(bgopacity) = args.bgopacity {
        theme.bg.3 = bgopacity;
    }

    let (pairlist, sequence) = match (pi.secondary_structure, pi.sequence) {
        (Some(sst), Some(seq)) => {
            let pl = rnamanip::get_pair_list(&sst);
            let seq = rnamanip::read_sequence(&seq);
            assert_eq!(
                pl.len(),
                seq.len(),
                "sequence and structure have differents lenghts!"
            );
            (pl, seq)
        }
        (Some(sst), None) => {
            let pairlist = rnamanip::get_pair_list(&sst);
            let seq = vec![Nucleotide::X; pairlist.len()]; // TODO del XSequence if am not gonna use it
            (pairlist, seq)
        }
        (None, Some(_)) => unimplemented!(
            "Calling external soft e.g. RNAFold to get secondary_structure not yet implemented"
        ),
        (None, None) => panic!("Neither sequence nor secondary structure found in the input file!"),
    };

    let tree = forest::grow_tree(&pairlist);

    let bblradius = 0.5;
    let bubbles = draw::gather_bubbles(&tree, &sequence, bblradius, args.angle.to_radians());
    let mirror = Mirror::new(args.mx, args.my);

    // TODO highlight is implementation is rushed
    // I need the functionality but haven't got the time to do it nicely :c
    let highlights = match pi.highlight {
        Some(hls) => draw::colors::user_input_to_highlight_indices(&hls),
        None => vec![None; sequence.len()],
    };

    let svgout = draw::plot(
        &bubbles,
        bblradius,
        &filename,
        &theme,
        args.height,
        mirror,
        &highlights,
    )
    .unwrap();

    return Ok(svgout.unwrap());
}

/// A Python module implemented in Rust.
#[pymodule]
fn rnapykin(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rna2svg, m)?)?;
    Ok(())
}
