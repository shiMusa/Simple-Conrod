

use elements::*;
use conrod;




/*
d8888b. d888888b .88b  d88.
88  `8D   `88'   88'YbdP`88
88   88    88    88  88  88
88   88    88    88  88  88
88  .8D   .88.   88  88  88
Y8888D' Y888888P YP  YP  YP


*/



#[derive(Debug, Copy, Clone)]
pub enum Dim {
    Absolute(i32),
    Relative(f64),
}





/*
d88888b  .d88b.  d8b   db d888888b
88'     .8P  Y8. 888o  88 `~~88~~'
88ooo   88    88 88V8o 88    88
88~~~   88    88 88 V8o88    88
88      `8b  d8' 88  V888    88
YP       `Y88P'  VP   V8P    YP


*/
#[derive(Debug, Clone)]
pub enum FontJustification {
    Left, Center, Right
}
#[derive(Debug, Clone)]
pub enum FontWrapping {
    Word, Character
}


#[derive(Debug, Clone)]
pub struct Font {
    text: String,
    size: Dim,
    font_id: String,
    color: conrod::Color,
    justi: FontJustification,
    wrap: FontWrapping,
}

impl Font {
    pub fn new(font_id: String) -> Self {
        Font {
            text: "".to_string(),
            size: Dim::Absolute(12),
            font_id,
            color: conrod::color::BLACK,
            justi: FontJustification::Center,
            wrap: FontWrapping::Word
        }
    }

    pub fn with_size(mut self, size: Dim) -> Self {
        self.size = size;
        self
    }

    pub fn with_color(mut self, color: conrod::Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_justification(mut self, justification: FontJustification) -> Self {
        self.justi = justification;
        self
    }

    pub fn with_wrapping(mut self, wrapping: FontWrapping) -> Self {
        self.wrap = wrapping;
        self
    }

    pub fn write(&self, text: String) -> Self {
        let mut res = self.clone();
        res.text = text;
        res
    }

    pub fn set_size(&mut self, size: Dim) {
        self.size = size;
    }

    pub fn set_color(&mut self, color: conrod::Color) {
        self.color = color;
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn get_size(&self) -> Dim {
        self.size
    }

    pub fn get_font_id(&self) -> String {
        self.font_id.clone()
    }

    pub fn get_color(&self) -> conrod::Color {
        self.color
    }

    pub fn set_justification(&mut self, justi: FontJustification) {
        self.justi = justi;
    }

    pub fn get_justification(&self) -> FontJustification {
        self.justi.clone()
    }

    pub fn set_wrapping(&mut self, wrapping: FontWrapping) {
        self.wrap = wrapping;
    }

    pub fn get_wrapping(&self) -> FontWrapping {
        self.wrap.clone()
    }
}




/*
 d888b  d8888b.  .d8b.  d8888b. db   db d888888b  .o88b.
88' Y8b 88  `8D d8' `8b 88  `8D 88   88   `88'   d8P  Y8
88      88oobY' 88ooo88 88oodD' 88ooo88    88    8P
88  ooo 88`8b   88~~~88 88~~~   88~~~88    88    8b
88. ~8~ 88 `88. 88   88 88      88   88   .88.   Y8b  d8
 Y888P  88   YD YP   YP 88      YP   YP Y888888P  `Y88P'


*/


#[derive(Debug, Clone)]
pub enum Graphic {
    Color(conrod::Color),
    Texture(Texture),
    None,
}









/*
d888888b d88888b db    db d888888b db    db d8888b. d88888b
`~~88~~' 88'     `8b  d8' `~~88~~' 88    88 88  `8D 88'
   88    88ooooo  `8bd8'     88    88    88 88oobY' 88ooooo
   88    88~~~~~  .dPYb.     88    88    88 88`8b   88~~~~~
   88    88.     .8P  Y8.    88    88b  d88 88 `88. 88.
   YP    Y88888P YP    YP    YP    ~Y8888P' 88   YD Y88888P


*/

#[derive(Debug, Copy, Clone)]
pub enum TextureMode {
    Stretch,
    FitWidth,
    FitHeight,
    FitMin,
    FitMax,
    Tile,
}

#[derive(Debug, Clone)]
pub struct Texture {
    id: String,
    cut: Option<Frame<u32>>,
    mode: TextureMode,
}

impl Texture {
    pub fn new(id: String) -> Self {
        Texture {
            id,
            cut: None,
            mode: TextureMode::Stretch,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn set_cut(&mut self, cut: Frame<u32>) {
        self.cut = Some(cut);
    }

    pub fn with_cut(mut self, cut: Frame<u32>) -> Self {
        self.cut = Some(cut);
        self
    }

    pub fn set_mode(&mut self, mode: TextureMode) {
        self.mode = mode;
    }

    pub fn with_mode(mut self, mode: TextureMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn get_cut(&self, w: u32,h: u32, img_w: u32, img_h: u32) -> Rect {
        //println!("get_cut() {} {} {} {}", w, h, img_w, img_h);

        // TODO implement pre-defined texture cut //////////////////////////////////////////////////
        //let mut tex_cut = match self.cut {
        //    Some(c) => c,
        //    None => Frame::new()
        //};
//
        let ratio = w as f64 / h as f64;
        //let (min, max) = if w > h {(h,w)} else {(w,h)};
//
        //let img_ratio = img_w as f64 / img_h as f64;
        //let (img_min, img_max) = if img_w > img_h {(img_h, img_w)} else {(img_w, img_h)};

        match self.mode {
            TextureMode::FitHeight => {
                Rect::from_corners([0.0,0.0], [ratio * img_w as f64, img_h as f64])
            },
            TextureMode::FitWidth => {
                Rect::from_corners([0.0,0.0], [img_w as f64, img_h as f64 / ratio])
            },
            TextureMode::FitMax => {
                if ratio > 1.0 {
                    Rect::from_corners([0.0,0.0], [img_w as f64, img_h as f64 / ratio])
                } else {
                    Rect::from_corners([0.0,0.0], [ratio * img_w as f64, img_h as f64])
                }
            },
            TextureMode::FitMin => {
                if ratio < 1.0 {
                    Rect::from_corners([0.0,0.0], [img_w as f64, img_h as f64 / ratio])
                } else {
                    Rect::from_corners([0.0,0.0], [ratio * img_w as f64, img_h as f64])
                }
            },
            _ => {
                println!("get_cut() Stretch");
                Rect::from_corners([0.0, 0.0], [img_w as f64, img_h as f64])
            },
        }
    }
}


