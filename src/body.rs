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
    fn make_image(&self, x:f64, y:f64, w:f64, h:f64, color:&str, opacity:f64) -> SvgGroup {
        match *self {
            Expression::Smile =>self.make_simple_body(x,y,w,h,color.clone(),"green","orange", "pink"),
            _=> self.make_simple_body(x,y,w,h,color.clone(),"brown","purple", "green"),
        }
    }
    //fn make_mouth(&self, x:f64, y:f64, w:f64, h:f64, color:&str) -> SvgGroup {}
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
    fn make_image(&self, x:f64, y:f64, w:f64, h:f64, color:&str, opacity:f64) -> SvgGroup;
/*
Draw an eye
*/
    fn make_eye(&self, x:f64, y:f64, w:f64, h:f64, color:&str) -> SvgGroup {
        self.make_eye_default(x,y,w,h,color.clone())
    }

/*
The four color body (teeth are always white)
*/
    fn make_simple_body(&self, x:f64, y:f64, w:f64, h:f64, color1:&str, color2:&str, color3:&str, color4:&str) -> SvgGroup {
        let skin = color1;
        let nose = color4;//TODO tint color
        let eye = color2;
        let hair = color3;
        let teeth = "white";
        let clothes1 = color4;
        let clothes2 = color2;
        self.make_full_body(x, y, w, h, skin, eye, nose, hair, teeth, clothes1, clothes2)
    }

/*
The fully configurable body.
*/
    fn make_full_body(&self, x:f64, y:f64, w:f64, h:f64, skin_color:&str, eye_color:&str, nose_color:&str, hair_color:&str, teeth_color:&str, clothes_color1:&str, clothes_color2:&str) -> SvgGroup {
        let initial_x:f64 = x;
        let initial_y:f64 = y;
        let initial_w:f64 = w;
        let initial_h:f64 = h;
        let half_w:f64 = w / 2.0;
        let quarter_w:f64 = w / 4.0;
        let head_h:f64 = h / 6.5;
        let head_w:f64 = quarter_w;
        let spacer:f64 = head_h / 4.0;
        // center in image based on widths
        let head_x:f64 = ((x + w) / 2.0) - (head_w / 2.0);
        let torso_h:f64 = head_h * 2.0;

        // make the groups
        let head = self.make_face_full(head_x, y, head_w, head_h, skin_color, eye_color, nose_color, hair_color, teeth_color);
        let hand = skin_color;
        let y = (y + head_h) - spacer;
        
        let clothes1 = self.make_ellipse(x + quarter_w, y, half_w, torso_h,clothes_color1);
        let l_arm = self.make_ellipse(x, y, quarter_w / 2.0, torso_h,clothes_color1);
        let l_arm_u = self.make_ellipse(x, y, torso_h, quarter_w / 2.0, clothes_color1);
        let clothes2 = self.make_ellipse(x + quarter_w, y + torso_h - spacer, half_w, torso_h,clothes_color2);
        SvgGroup::new()
                 .add(clothes2)
                 .add(clothes1)
                 .add(l_arm_u)
                 .add(l_arm)
                 .add(head)
    }

/*
Make full face
*/
    fn make_face_full(&self, x:f64, y:f64, w:f64, h:f64, skin_color:&str, eye_color:&str, nose_color:&str, hair_color:&str, teeth_color:&str) -> SvgGroup {
        let face = self.make_ellipse(x, y, w, h, skin_color);
        let eye_w:f64 = w / 5.0;
        let y = y + (h / 3.0);
        let spacer = w / 12.0;
        let face_x:f64 = x + ((w / 2.0) - (eye_w / 2.0));
        let nose = self.make_ellipse(face_x, y, eye_w, eye_w * 2.0, nose_color);
        let eye_l = self.make_eye(face_x - eye_w, y, eye_w, eye_w, eye_color);
        let eye_r = self.make_eye(face_x + eye_w, y, eye_w, eye_w, eye_color);
        let ellipse_y:f64 = y + (eye_w * 2.0);
        let mouth = self.make_mouth(face_x - eye_w, ellipse_y + (h/12.0), (eye_w * 3.0), 0.0, teeth_color);
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
*/
    fn make_face_default(&self, x:f64, y:f64, w:f64, h:f64, skin_color:&str, eye_color:&str, nose_color:&str, hair_color:&str) -> SvgGroup {
        let face = self.make_ellipse(x,y,w,h,skin_color);
        let eye_w:f64 = w / 5.0;
        let y = y + (h / 3.0);
        let face_x:f64 = (w / 2.0) - (eye_w / 2.0);
        let nose = self.make_ellipse(face_x, y, eye_w, eye_w * 2.0, nose_color);
        let eye_l = self.make_eye(face_x - eye_w, y, eye_w, eye_w, eye_color);
        let eye_r = self.make_eye(face_x + eye_w, y, eye_w, eye_w, eye_color);
        let ellipse_y:f64 = y + (eye_w * 2.0);
        let mouth = self.make_mouth((x / 8.0)*7.0 , ellipse_y + (h/12.0), w, 0.0, "white");
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
    fn make_teeth(&self, x:f64, y:f64, w:f64, h:f64, color:&str) -> SvgGroup {
        let tooth = self.make_rectangle(x, y, w, h, color);
        SvgGroup::new()
                 .add(tooth)
    }
/*
Make the default "smile" mouth
*/
    fn make_smile_mouth(&self, x:f64, y:f64, w:f64, h:f64, color:&str) -> SvgGroup {
        let mut path = self.make_down_part(x,y,w,"black", 1.0);
        let height:f64 = y / 4.0;
        let mut x:f64 = x;
        x += w / 4.0;
        let half:f64 = w / 2.0;
        let spacer = w / 12.0;
        let tooth_w:f64 = half - (2.0 * spacer);
        let tooth_h:f64 = height / 3.0;
        x += spacer;
        let tooth = self.make_teeth(x, y, tooth_w, tooth_h, color);
        SvgGroup::new()
                 .add(path)
                 .add(tooth)
    }
    fn make_mouth(&self, x:f64, y:f64, w:f64, h:f64, color:&str) -> SvgGroup {
        self.make_smile_mouth(x,y,w,h,color.clone())
    }
    
/*
Built in function makes the half circle down portion of a mouth
*/
    fn make_down_part(&self, x:f64, y:f64, w:f64, color:&str, opacity:f64) -> Path {
        let height:f64 = y / 4.0;
        let mut x:f64 = x;
        x += w / 4.0;
        let half:f64 = w / 2.0;
        let mouth = Data::new()
                        .move_to((x, y))
                        .cubic_curve_by((0.0, height, half, height, half, 0.0))
                        .close();
        Path::new()
             .set("fill", color)
             .set("opacity", opacity.to_string().as_str())
             .set("d", mouth)
    }
/*
Make an upside-down bowl shape
*/
    fn make_up_half(&self, x:f64, y:f64, w:f64, h:f64, color:&str, opacity:f64) -> Path {
        let mouth = Data::new()
                        .move_to((x, y))
                        .cubic_curve_by((0.0, -h, w, -h, w, 0.0))
                        .close();
        Path::new()
            .set("fill", color)
             .set("opacity", opacity.to_string().as_str())
             .set("d", mouth)
    }
/*
Make an ellipse
*/
    fn make_ellipse(&self, x:f64, y:f64, w:f64, h:f64, color:&str) -> Ellipse {
        let cx:f64 = (w / 2.0) + x;
        let cy:f64 = (h / 2.0) + y;
        let rx:f64 = w / 2.0;
        let ry:f64 = h / 2.0;
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
    fn make_ellipse_shadow(&self, x:f64, y:f64, w:f64, h:f64, color:&str) -> Ellipse {
        let cx:f64 = (w / 2.0) + x;
        let cy:f64 = (h / 2.0) + y;
        let rx:f64 = w / 2.0;
        let ry:f64 = h / 2.0;
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
    fn make_eye_default(&self, x:f64, y:f64, w:f64, h:f64, color:&str) -> SvgGroup {
        let white = self.make_ellipse(x,y,w,h,"white");
        let shadow = self.make_ellipse_shadow(x,y + 1.0,w,h,"black");
        let iris_x:f64 = x + (w / 4.0) + (w / 10.0);
        let iris_y:f64 = y + (h /4.0) -  (h / 5.0);
        let mut iris_w:f64 = w / 2.0;
        let mut iris_h:f64 = h / 2.0;
        if iris_w <= 0.0 {
            iris_w += 0.5;
        }
        if iris_h <= 0.0 {
            iris_h += 0.5;
        }
        let iris = self.make_ellipse(iris_x, iris_y, iris_w, iris_w, color);
        let pupil = self.make_ellipse(iris_x + (iris_w / 4.0), iris_y + (iris_h / 4.0), w / 4.0, h / 4.0, "black");
        let g = SvgGroup::new()
                        .add(shadow)
                        .add(white)
                        .add(iris)
                        .add(pupil);
        g
    }
/*
Make a tooth-like triangle
*/
    fn make_sharp(&self, x:f64, y:f64, w:f64, color:&str, opacity:f64) -> Path {
        let tooth = Data::new()
                        .move_to((x, y))
                        .line_to((x + w, y))
                        .line_to((x + (w / 2.0), y + w))
                        .close();
        Path::new()
             .set("fill", color)
             .set("opacity", opacity.to_string().as_str())
            .set("d", tooth)
    }
/*
Make a rectangle (like teeth)
*/
    fn make_rectangle(&self, x:f64, y:f64, w:f64, h:f64, color:&str) -> Rectangle {
        Rectangle::new()
                  .set("fill", color)
                  .set("x", x.to_string().as_str())
                  .set("y", y.to_string().as_str())
                  .set("width", w.to_string().as_str())
                  .set("height", h.to_string().as_str())
                  .set("rx","0.4")
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
