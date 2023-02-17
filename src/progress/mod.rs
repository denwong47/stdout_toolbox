use super::modifiers::*;

#[allow(dead_code)]
pub enum ProgressStyle {
    Arrow,
    Blocks,
    Density,
    DropBlocks,
    GrayscaleToWhite, // This one is very slow
    Numbers,
}
impl ProgressStyle {
    #[allow(dead_code)]
    pub fn bar(&self, length: usize, blur: Option<usize>) -> ProgressBar {
        match self {
            Self::Arrow => ProgressBar::new("\x20>=", length, None),
            Self::Blocks => ProgressBar::new(
                "\x20\u{258f}\u{258e}\u{258d}\u{258c}\u{258b}\u{258a}\u{2589}\u{2588}",
                length,
                None,
            ),
            Self::Density => ProgressBar::new("\x20\u{2591}\u{2592}\u{2593}\u{2588}", length, blur),
            Self::DropBlocks => ProgressBar::new(
                "\x20\u{2598}\u{2596}\u{259E}\u{2584}\u{2599}\u{2588}",
                length,
                blur,
            ),
            Self::GrayscaleToWhite => ProgressBar::from_iter(
                BackgroundColours::iter_grayscale().map(|colour| colour.wraps(&" ".to_string())),
                length,
                blur,
            ),
            Self::Numbers => ProgressBar::new("\x20123456789.", length, None),
        }
    }
}

#[derive(Debug)]
pub struct ProgressBar {
    chars: Vec<String>,
    pub length: usize,
    blur: usize,
}
#[allow(dead_code)]
impl ProgressBar {
    pub fn new(chars: &str, length: usize, blur: Option<usize>) -> Self {
        return Self {
            chars: chars.chars().map(|c| c.to_string()).collect(),
            length: length,
            blur: blur.unwrap_or(0),
        };
    }

    pub fn from_iter<T>(iter: T, length: usize, blur: Option<usize>) -> Self
    where
        T: Iterator<Item = String>,
    {
        return Self {
            chars: iter.collect(),
            length: length,
            blur: blur.unwrap_or(0),
        };
    }

    pub fn draw(&self, progress: f64) -> String {
        let ratio_per_segment = 1. / (self.length as f64);
        let blur_ratio = (1 + self.blur * 2) as f64;

        let progress_capped = { progress.min(1.).max(0.) }
            * (1. + ratio_per_segment * 2. * self.blur as f64)
            - ratio_per_segment * self.blur as f64;
        (0..self.length)
            .into_iter()
            .map(|pos| {
                (progress_capped - ratio_per_segment * (pos as f64 - self.blur as f64))
                    .min(ratio_per_segment * blur_ratio)
                    .max(0.)
                    / ratio_per_segment
                    / blur_ratio
            })
            .map(|segment_ratio| (segment_ratio * (self.chars.len() - 1) as f64).round() as usize)
            .map(|idx| &self.chars[idx])
            .fold(String::new(), |mut s, c| {
                s.push_str(c);
                s
            })
    }
}

pub struct ProgressedIterator<'a, T, I>
where
    I: Iterator<Item = T>,
{
    iter: I,
    bar: &'a ProgressBar,
    total: usize,
    prefix: String,
    suffix: String,

    counter: usize,
}
#[allow(dead_code)]
impl<'a, T, I> ProgressedIterator<'a, T, I>
where
    I: Iterator<Item = T>,
{
    pub fn new(
        iter: I,
        bar: &'a ProgressBar,
        total: usize,
        prefix: Option<&str>,
        suffix: Option<&str>,
    ) -> Self {
        return Self {
            iter: iter,
            bar: bar,
            total: total,
            prefix: prefix.unwrap_or("").to_string(),
            suffix: suffix.unwrap_or("").to_string(),
            counter: 0,
        };
    }

    pub fn display(&self) {
        let mut len_count: usize = 0;
        let mut to_print: String = String::new();
        let progress = self.counter as f64 / self.total as f64;

        len_count += self.prefix.len();
        to_print.push_str(&self.prefix);

        len_count += 2 + self.bar.length;
        to_print.push('|');
        to_print.push_str(&self.bar.draw(progress));
        to_print.push('|');

        len_count += 8;
        to_print.push_str(&format!(" {:>6.2}%", (progress * 100.).min(100.)));

        len_count += self.suffix.len();
        to_print.push_str(&self.suffix);

        to_print.push_str(&MoveCursor::Left(len_count as i16).value());

        print!("{}", to_print);
    }

    pub fn reset(&mut self) {
        self.counter = 0;
    }
}
impl<'a, T, I> Iterator for ProgressedIterator<'a, T, I>
where
    I: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == 0 {
            self.display();
        }
        self.counter += 1;

        let _return = self.iter.next();

        self.display();

        return _return;
    }
}

pub trait Progressed<T, I>
where
    I: Iterator<Item = T>,
{
    fn progressed<'a>(
        self,
        bar: &'a ProgressBar,
        total: usize,
        prefix: Option<&str>,
        suffix: Option<&str>,
    ) -> ProgressedIterator<'a, T, I>;
}

impl<T, I> Progressed<T, I> for I
where
    I: Iterator<Item = T>,
{
    fn progressed<'a>(
        self,
        bar: &'a ProgressBar,
        total: usize,
        prefix: Option<&str>,
        suffix: Option<&str>,
    ) -> ProgressedIterator<'a, T, I> {
        return ProgressedIterator::new(self, bar, total, prefix, suffix);
    }
}
