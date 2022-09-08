// Based on:
//   https://salsa.debian.org/benchmarksgame-team/benchmarksgame/-/blob/master/public/download/benchmarksgame-sourcecode.zip
//   regexredux.rust-2.rust
pub fn run_regexredux_impl<F>(clock_ms: F) -> f32
    where F: Fn() -> i64 + Send + Sync + 'static
{
    static INPUT: &[u8] = include_bytes!("regexredux-input1000.txt");
    macro_rules! regex {
        ($re:expr) => {
            regex::bytes::Regex::new($re).unwrap()
        }
    }

    let timestamp = clock_ms();

    for _ in 0..10 {
        let sequence = regex!(">[^\n]*\n|\n").replace_all(&INPUT, &b""[..]).into_owned();
        let sequence_c = sequence.clone();

        let mut total_count = 0;

        total_count += vec![
            ("tHa[Nt]", &b"<4>"[..]),
            ("aND|caN|Ha[DS]|WaS", &b"<3>"[..]),
            ("a[NSt]|BY", &b"<2>"[..]),
            ("<[^>]*>", &b"|"[..]),
            ("\\|[^|][^|]*\\|", &b"-"[..]),
        ].into_iter().fold(sequence_c, |mut buffer, (pattern, replacement)| {
            regex!(pattern).replace_all(&mut buffer, replacement).into_owned()
        }).len();

        total_count += vec![
            "agggtaaa|tttaccct",
            "[cgt]gggtaaa|tttaccc[acg]",
            "a[act]ggtaaa|tttacc[agt]t",
            "ag[act]gtaaa|tttac[agt]ct",
            "agg[act]taaa|ttta[agt]cct",
            "aggg[acg]aaa|ttt[cgt]ccct",
            "agggt[cgt]aa|tt[acg]accct",
            "agggta[cgt]a|t[acg]taccct",
            "agggtaa[cgt]|[acg]ttaccct",
        ].into_iter().map(|variant| {
            regex!(variant).find_iter(&sequence).count()
        }).sum::<usize>();

        assert_eq!(total_count, 5266);
    }

    let elapsed = clock_ms() - timestamp;
    elapsed as f32
}

#[cfg(target_arch = "wasm32")]
extern {
    fn clock_ms() -> i64;
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub fn run_regexredux() -> f32 {
    run_regexredux_impl(|| unsafe { clock_ms() })
}
