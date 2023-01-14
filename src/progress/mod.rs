pub static BLOCKS: &str = "\x20\u{258f}\u{258e}\u{258d}\u{258c}\u{258b}\u{258a}\u{2589}\u{2588}";
pub static GRADIENT: &str = "\x20\u{2591}\u{2592}\u{2593}\u{2588}";
pub static DROP_BLOCKS: &str = "\x20\u{2598}\u{2596}\u{259E}\u{2584}\u{2599}\u{2588}";

#[derive(Debug)]
pub struct ProgressStyle {
    chars: Vec<char>,
}
impl ProgressStyle {
    pub fn new(chars: &str) -> Self {
        return Self {
            chars: chars.chars().collect(),
        };
    }

    pub fn draw(&self, progress: f64, len: usize) -> String {
        let progress_capped = progress.min(1.).max(0.);
        let ratio_per_segment = 1. / (len as f64);

        (0..len)
            .into_iter()
            .map(|pos| {
                (progress_capped - ratio_per_segment * pos as f64).min(ratio_per_segment)
                    / ratio_per_segment
            })
            .map(|segment_ratio| (segment_ratio * (self.chars.len() - 1) as f64).round() as usize)
            .map(|idx| self.chars[idx])
            .fold(String::new(), |mut s, c| {
                s.push(c);
                s
            })
    }
}
