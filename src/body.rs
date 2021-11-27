#[cfg(feature = "fltkform")]
use fltk::{prelude::*, *};
#[cfg(feature = "fltkform")]
use fltk_form_derive::*;
#[cfg(feature = "fltkform")]
use fltk_form::{FltkForm, HasProps};
#[cfg(feature = "fltkform")]
use std::collections::HashMap;
#[cfg(feature = "fltkform")]
use std::mem::transmute;

use std::fmt;
use serde::{Deserialize, Serialize};

use crate::random::*;
use crate::class::Advanced as Class;

#[cfg(feature = "makesvg")]
use svg::Document;
#[cfg(feature = "makesvg")]
use svg::node::element::{Ellipse, Filter, Rectangle, path::Data, Path};
#[cfg(feature = "makesvg")]
use svg::node::element::Group as SvgGroup;

/*
# Expression
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
pub enum Expression {
    Smile,
    Hurt,
    Angry,
    None,
}
#[cfg(feature = "makesvg")]
impl VectorView for Expression{
    fn make_image(&self, x:i32, y:i32, w:i32, h:i32, color:&str, opacity:f64) -> SvgGroup {
        match *self {
            Expression::Smile =>self.make_face_default(x,y,w,h,color.clone(),"green","orange", "pink"),
            _=> self.make_face_default(x,y,w,h,color.clone(),"brown","purple", "green"),
        }
    }
    //fn make_mouth(&self, x:i32, y:i32, w:i32, h:i32, color:&str) -> SvgGroup {}
}
/*
This trait builds vector graphics for characters and creatures
*/
#[cfg(feature = "makesvg")]
pub trait VectorView {
/*
This function is the one you define what is drawn
Choose from any of the functions in this trait to compose your object visually
*/
    fn make_image(&self, x:i32, y:i32, w:i32, h:i32, color:&str, opacity:f64) -> SvgGroup;
/*
Draw an eye
*/
    fn make_eye(&self, x:i32, y:i32, w:i32, h:i32, color:&str) -> SvgGroup {
        self.make_eye_default(x,y,w,h,color.clone())
    }

//TODO
    fn make_face_default(&self, x:i32, y:i32, w:i32, h:i32, color:&str, eye_color:&str, nose_color:&str, hair_color:&str) -> SvgGroup {
        let face = self.make_ellipse(x,y,w,h,color);
        let eye_w:i32 = w / 5;
        let y = y + (h / 3);
        let face_x:i32 = (w / 2) - (eye_w / 2);
        let nose = self.make_ellipse(face_x, y, eye_w, eye_w * 2, nose_color);
        let eye_l = self.make_eye(face_x - eye_w, y, eye_w, eye_w, eye_color);
        let eye_r = self.make_eye(face_x + eye_w, y, eye_w, eye_w, eye_color);
        let ellipse_y:i32 = y + (eye_w * 2);
        let mouth = self.make_mouth((x / 8)*7 , ellipse_y + (h/12), w, 0, "white");
        //let hair = self.make_hair(x,y,w,h,hair_color);
        SvgGroup::new()
                    .add(face)
                    .add(nose)
                    .add(eye_r)
                    .add(eye_l)
                    //.add(hair)
                    .add(mouth)
    }
/*
Override for different teeth
*/
    fn make_teeth(&self, x:i32, y:i32, w:i32, h:i32, color:&str) -> SvgGroup {
        let tooth = self.make_rectangle(x, y, w, h, color);
        SvgGroup::new()
                 .add(tooth)
    }
/*
Make the default "smile" mouth
*/
    fn make_smile_mouth(&self, x:i32, y:i32, w:i32, h:i32, color:&str) -> SvgGroup {
        let mut path = self.make_down_part(x,y,w,"black", 1.0);
        let height:i32 = y / 4;
        let mut x:i32 = x;
        x += w / 4;
        let half:i32 = w / 2;
        let spacer = w / 12;
        let tooth_w:i32 = half - (2 * spacer);
        let tooth_h:i32 = height / 3;
        x += spacer;
        let tooth = self.make_teeth(x, y, tooth_w, tooth_h, color);
        SvgGroup::new()
                 .add(path)
                 .add(tooth)
    }
    fn make_mouth(&self, x:i32, y:i32, w:i32, h:i32, color:&str) -> SvgGroup {
        self.make_smile_mouth(x,y,w,h,color.clone())
    }
    
/*
Built in function makes the half circle down portion of a mouth
*/
    fn make_down_part(&self, x:i32, y:i32, w:i32, color:&str, opacity:f64) -> Path {
        let height:i32 = y / 4;
        let mut x:i32 = x;
        x += w / 4;
        let half:i32 = w / 2;
        let mouth = Data::new()
                        .move_to((x, y))
                        .cubic_curve_by((0, height, half, height, half, 0))
                        .close();
        Path::new()
             .set("fill", color)
             .set("opacity", opacity.to_string().as_str())
             .set("d", mouth)
    }
/*
Make an upside-down bowl shape
*/
    fn make_up_half(&self, x:i32, y:i32, w:i32, h:i32, color:&str, opacity:f64) -> Path {
        let mouth = Data::new()
                        .move_to((x, y))
                        .cubic_curve_by((0, -h, w, -h, w, 0))
                        .close();
        Path::new()
            .set("fill", color)
             .set("opacity", opacity.to_string().as_str())
             .set("d", mouth)
    }
/*
Make an ellipse
*/
    fn make_ellipse(&self, x:i32, y:i32, w:i32, h:i32, color:&str) -> Ellipse {
        let cx:i32 = (w / 2) + x;
        let cy:i32 = (h / 2) + y;
        let rx:i32 = w / 2;
        let ry:i32 = h / 2;
        let mut style:String = "opacity:1;fill:".to_string();
        style.push_str(color);
        Ellipse::new()
                .set("style", style.as_str())
                .set("cx", cx)
                .set("cy", cy)
                .set("rx", rx)
                .set("ry", ry)
    }
/*
Make an ellipse that is semi-transparent for a shadow effect
*/
    fn make_ellipse_shadow(&self, x:i32, y:i32, w:i32, h:i32, color:&str) -> Ellipse {
        let cx:i32 = (w / 2) + x;
        let cy:i32 = (h / 2) + y;
        let rx:i32 = w / 2;
        let ry:i32 = h / 2;
        let mut style:String = "opacity:0.5;fill:".to_string();
        style.push_str(color);
        Ellipse::new()
                .set("style", style.as_str())
                .set("cx", cx)
                .set("cy", cy)
                .set("rx", rx)
                .set("ry", ry)
    }
/*
Make the default eye
*/
    fn make_eye_default(&self, x:i32, y:i32, w:i32, h:i32, color:&str) -> SvgGroup {
        let white = self.make_ellipse(x,y,w,h,"white");
        let black = self.make_ellipse_shadow(x,y + 1,w,h,"black");
        let iris_x:i32 = x + (w / 4) + (w / 10);
        let iris_y:i32 = y + (h /4) -  (h / 5);
        let iris_w:i32 = w / 2;
        let iris_h:i32 = h / 2;
        let iris = self.make_ellipse(iris_x, iris_y, iris_w, iris_w, color);
        let pupil = self.make_ellipse(iris_x + (iris_w / 4), iris_y + (iris_h / 4), w / 4, h / 4, "black");
        let g = SvgGroup::new()
                        .add(black)
                        .add(white)
                        .add(iris)
                        .add(pupil);
        g
    }
/*
Make a tooth-like triangle
*/
    fn make_sharp(&self, x:i32, y:i32, w:i32, color:&str, opacity:f64) -> Path {
        let tooth = Data::new()
                        .move_to((x, y))
                        .line_to((x + w, y))
                        .line_to((x + (w / 2), y + w))
                        .close();
        Path::new()
             .set("fill", color)
             .set("opacity", opacity.to_string().as_str())
            .set("d", tooth)
    }
/*
Make a rectangle (like teeth)
*/
    fn make_rectangle(&self, x:i32, y:i32, w:i32, h:i32, color:&str) -> Rectangle {
        Rectangle::new()
                  .set("fill", color)
                  .set("x", x.to_string().as_str())
                  .set("y", y.to_string().as_str())
                  .set("width", w.to_string().as_str())
                  .set("height", h.to_string().as_str())
                  .set("rx","1")
    }
}


#[derive(Clone, Debug)]
#[cfg(feature = "makesvg")]
pub struct Body {
    pub head:SvgGroup,
    pub hair:SvgGroup,
    pub face:SvgGroup,
    pub torso:SvgGroup,
    pub arms:SvgGroup,
    pub legs:SvgGroup,
}
#[cfg(feature = "makesvg")]
impl Body {
    pub fn new()-> Self {
        Body {
            head:SvgGroup::new(),
            hair:SvgGroup::new(),
            face:SvgGroup::new(),
            torso:SvgGroup::new(),
            arms:SvgGroup::new(),
            legs:SvgGroup::new(),
        }
    }
}
