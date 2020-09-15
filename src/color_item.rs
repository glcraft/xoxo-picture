#[derive(Eq)]
struct ColorItem {
    color: (u8, u8, u8),
    file: String
}
impl ColorItem {
    fn new(color: (u8, u8, u8), file: String) -> Self {
        ColorItem{color, file}
    }
}
impl Ord for ColorItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.color.0<other.color.0 {
            std::cmp::Ordering::Less
        } else if self.color.0>other.color.0 {
            std::cmp::Ordering::Greater
        } else {
            if self.color.1<other.color.1 {
                std::cmp::Ordering::Less
            } else if self.color.1>other.color.1 {
                std::cmp::Ordering::Greater
            } else {
                if self.color.2<other.color.2 {
                    std::cmp::Ordering::Less
                } else if self.color.2>other.color.2 {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }
        }
    }
}
impl PartialEq for ColorItem {
    fn eq(&self, other: &Self) -> bool {
        self.color.0 == other.color.0 && self.color.1 == other.color.1 && self.color.2 == other.color.2
    }
}
impl PartialOrd for ColorItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}