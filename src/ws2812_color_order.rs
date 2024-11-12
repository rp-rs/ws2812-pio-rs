#[derive(Copy, Clone)]
pub enum Ws2812ColorOrder {
    RGB,
    GRB,
    RBG,
    BGR,
    GBR,
    BRG,
}

impl Ws2812ColorOrder {
    pub fn _get_color_value(&self, color: &smart_leds_trait::RGB8) -> u32 {
        match self {
            Ws2812ColorOrder::RGB => {
                return (u32::from(color.r) << 24)
                    | (u32::from(color.g) << 16)
                    | (u32::from(color.b) << 8)
            }
            Ws2812ColorOrder::GRB => {
                return (u32::from(color.g) << 24)
                    | (u32::from(color.r) << 16)
                    | (u32::from(color.b) << 8)
            }
            Ws2812ColorOrder::RBG => {
                return (u32::from(color.r) << 24)
                    | (u32::from(color.b) << 16)
                    | (u32::from(color.g) << 8)
            }
            Ws2812ColorOrder::BGR => {
                return (u32::from(color.b) << 24)
                    | (u32::from(color.g) << 16)
                    | (u32::from(color.r) << 8)
            }
            Ws2812ColorOrder::GBR => {
                return (u32::from(color.g) << 24)
                    | (u32::from(color.b) << 16)
                    | (u32::from(color.r) << 8)
            }
            Ws2812ColorOrder::BRG => {
                return (u32::from(color.b) << 24)
                    | (u32::from(color.r) << 16)
                    | (u32::from(color.g) << 8)
            }
        };
    }
}
