enum LayoutMode {
    Center,
    LeftToRight
}

struct LayoutConfig {
    mode: LayoutMode,
    inner_pad: Padding,
    outer_pad: Padding

}

#[derive(Debug, PartialEq)]
struct Padding {
    top: usize,
    right: usize,
    bottom: usize,
    left: usize,
}

impl Padding {
    fn new_all(pad: usize) -> Padding {
        Padding{ top: pad, right: pad, bottom: pad, left: pad }
    }

    fn new_ver_hor(ver_pad: usize, hor_pad: usize) -> Padding {
        Padding{ top: ver_pad, bottom: ver_pad, left: hor_pad, right: hor_pad }

    }
}

impl LayoutConfig {
    fn new() -> LayoutConfig {
        LayoutConfig{ 
            mode: LayoutMode::LeftToRight,
            inner_pad: Padding::new_all(5),
            outer_pad: Padding::new_all(5),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_layout_config() {
        let layout = LayoutConfig::new();
    }

    fn constrct_padding() {
        let first_pad = Padding::new_all(4);
        let second_pad = Padding::new_ver_hor(4, 4);

        assert_eq!(first_pad, second_pad) 
    }
}
