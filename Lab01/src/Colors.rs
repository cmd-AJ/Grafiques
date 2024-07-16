struct Color { //Objeto
    r: u8,
    g: u8,
    b: u8
}

impl Color { //La implementacion del objeto color

    fn new( r: u8, g: u8, b: u8 ) -> Color {
        Color{
            r,
            g, 
            b,
        }
    }

    fn from_hex( hex: u32) -> Color {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Color {
            r,
            g,
            b
        }
    }

    fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8 ) | (self.b as u32)
    }


}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, other_color: Color) -> Color {
        let r = self.r.saturating_add(other_color.r);
        let g = self.g.saturating_add(other_color.g);
        let b = self.b.saturating_add(other_color.b);
        Color { r, g, b }
        
    }
    
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, constant: f32) -> Color {
        let r = (self.r as f32 * constant).clamp(0.0, 255.0) as u8;
        let g = (self.g as f32 * constant).clamp(0.0, 255.0) as u8;
        let b = (self.b as f32 * constant).clamp(0.0, 255.0) as u8;
        Color { r,g,b}    
    }
    
}



fn main(){
    let objeto_color = Color::new( u8::from(255), u8::from(165), u8::from(0) );

    println!( "El codigo del color  es {}, {}, {} ", objeto_color.r, objeto_color.g, objeto_color.b );
    println!( "En hexadecimal es {}", objeto_color.to_hex());
    let hex_color = Color::from_hex(0xFFA500);
    println!( "El codigo del color  es {}, {}, {} ", hex_color.r, hex_color.g, hex_color.b );

}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_color() {
        let color1 = Color { r: 100, g: 150, b: 200 };
        let color2 = Color { r: 50, g: 75, b: 100 };

        let result = color1 + color2;

        assert_eq!(result.r, 150);
        assert_eq!(result.g, 225);
        assert_eq!(result.b, 255);
    }

    #[test]
    fn test_mul_color() {
        let color = Color { r: 100, g: 150, b: 200 };
        let constant = 1.5;

        let result = color * constant;

        assert_eq!(result.r, 150); 
        assert_eq!(result.g, 225); 
        assert_eq!(result.b, 255); 
    }

    #[test]
    fn test_mul_color_with_negative() {
        let color = Color { r: 100, g: 150, b: 200 };
        let constant = -1.5;

        let result = color * constant;

        assert_eq!(result.r, 0); 
        assert_eq!(result.g, 0); 
        assert_eq!(result.b, 0); 
    }
}