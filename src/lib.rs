pub mod items;

use sdl3::pixels::Color;
use sdl3::ttf;

use sdl3::Sdl;
use sdl3::{AudioSubsystem, EventPump, EventSubsystem, VideoSubsystem};

use sdl3::event::Event;
use sdl3::render::*;
use sdl3::video::WindowContext;

use std::fs;
use std::io;
use std::path::*;

/// Extension trait that provides the methods FRect is missing compared to Rect.
/// Covers edge accessors, corner helpers, point/rect containment, intersection,
/// union, and mutation helpers (offset, resize, center_on, set_* edges).
pub trait FRectExt {
    /// Returns the y coordinate of the top edge.
    fn top(&self) -> f32;
    /// Returns the y coordinate of the bottom edge (y + height).
    fn bottom(&self) -> f32;
    /// Returns the x coordinate of the left edge.
    fn left(&self) -> f32;
    /// Returns the x coordinate of the right edge (x + width).
    fn right(&self) -> f32;
    /// Returns the centre point of the rect.
    fn center(&self) -> FPoint;
    /// Returns the top-left corner.
    fn top_left(&self) -> FPoint;
    /// Returns the top-right corner.
    fn top_right(&self) -> FPoint;
    /// Returns the bottom-left corner.
    fn bottom_left(&self) -> FPoint;
    /// Returns the bottom-right corner.
    fn bottom_right(&self) -> FPoint;
    /// Returns true when the point lies inside or on the boundary of this rect.
    fn contains_point<P: Into<FPoint>>(&self, point: P) -> bool;
    /// Returns true when `other` is fully enclosed by this rect.
    fn contains_rect(&self, other: &FRect) -> bool;
    /// Returns true when this rect and `other` overlap (touching edges count as no intersection).
    fn has_intersection(&self, other: &FRect) -> bool;
    /// Returns the overlapping region of the two rects, or `None` if they do not intersect.
    fn intersection(&self, other: &FRect) -> Option<FRect>;
    /// Returns the smallest rect that fully encloses both rects.
    fn union(&self, other: &FRect) -> FRect;
    /// Translates the rect by `(dx, dy)` in place.
    fn offset(&mut self, dx: f32, dy: f32);
    /// Replaces the size of the rect in place, keeping the origin.
    fn resize(&mut self, width: f32, height: f32);
    /// Repositions the rect so that its centre lands on `point`.
    fn center_on<P: Into<FPoint>>(&mut self, point: P);
    /// Moves the left edge to `left`, keeping the width.
    fn set_left(&mut self, left: f32);
    /// Moves the top edge to `top`, keeping the height.
    fn set_top(&mut self, top: f32);
    /// Moves the right edge to `right`, keeping the width (adjusts x).
    fn set_right(&mut self, right: f32);
    /// Moves the bottom edge to `bottom`, keeping the height (adjusts y).
    fn set_bottom(&mut self, bottom: f32);
}

impl FRectExt for FRect {
    #[inline] fn top(&self) -> f32    { self.y }
    #[inline] fn bottom(&self) -> f32 { self.y + self.h }
    #[inline] fn left(&self) -> f32   { self.x }
    #[inline] fn right(&self) -> f32  { self.x + self.w }

    fn center(&self) -> FPoint {
        FPoint::new(self.x + self.w / 2.0, self.y + self.h / 2.0)
    }
    fn top_left(&self) -> FPoint     { FPoint::new(self.x,          self.y)  }
    fn top_right(&self) -> FPoint    { FPoint::new(self.x + self.w, self.y)  }
    fn bottom_left(&self) -> FPoint  { FPoint::new(self.x,  self.y + self.h) }
    fn bottom_right(&self) -> FPoint { FPoint::new(self.x + self.w, self.y + self.h) }

    fn contains_point<P: Into<FPoint>>(&self, point: P) -> bool {
        let p: FPoint = point.into();
        p.x >= self.left() && p.x <= self.right() && p.y >= self.top() && p.y <= self.bottom()
    }

    fn contains_rect(&self, other: &FRect) -> bool {
        other.left() >= self.left()
            && other.top() >= self.top()
            && other.right() <= self.right()
            && other.bottom() <= self.bottom()
    }

    fn has_intersection(&self, other: &FRect) -> bool {
        self.left() < other.right()
            && self.right() > other.left()
            && self.top() < other.bottom()
            && self.bottom() > other.top()
    }

    fn intersection(&self, other: &FRect) -> Option<FRect> {
        if !self.has_intersection(other) {
            return None;
        }
        let x = self.left().max(other.left());
        let y = self.top().max(other.top());
        let r = self.right().min(other.right());
        let b = self.bottom().min(other.bottom());
        Some(FRect::new(x, y, r - x, b - y))
    }

    fn union(&self, other: &FRect) -> FRect {
        let x = self.left().min(other.left());
        let y = self.top().min(other.top());
        let r = self.right().max(other.right());
        let b = self.bottom().max(other.bottom());
        FRect::new(x, y, r - x, b - y)
    }

    fn offset(&mut self, dx: f32, dy: f32) {
        *self = FRect::new(self.x + dx, self.y + dy, self.w, self.h);
    }

    fn resize(&mut self, width: f32, height: f32) {
        *self = FRect::new(self.x, self.y, width, height);
    }

    fn center_on<P: Into<FPoint>>(&mut self, point: P) {
        let p: FPoint = point.into();
        *self = FRect::new(p.x - self.w / 2.0, p.y - self.h / 2.0, self.w, self.h);
    }

    fn set_left(&mut self, left: f32) {
        *self = FRect::new(left, self.y, self.w, self.h);
    }
    fn set_top(&mut self, top: f32) {
        *self = FRect::new(self.x, top, self.w, self.h);
    }
    fn set_right(&mut self, right: f32) {
        *self = FRect::new(right - self.w, self.y, self.w, self.h);
    }
    fn set_bottom(&mut self, bottom: f32) {
        *self = FRect::new(self.x, bottom - self.h, self.w, self.h);
    }
}

pub const DEFAULT_COLOR: Color = Color::RGB(210, 210, 220);
pub const DEFAULT_CLEAR_COLOR: Color = Color::RGB(20, 20, 20);

pub const COLOR_WHITE: Color = Color::RGB(194, 194, 194);
pub const COLOR_GRAY: Color = Color::RGB(131, 131, 131);
pub const COLOR_RED: Color = Color::RGB(212, 74, 74);
pub const COLOR_GREEN: Color = Color::RGB(74, 212, 74);
pub const COLOR_BLUE: Color = Color::RGB(74, 74, 212);

const SLIDER_PIVOT_COLOR: Color = Color::RGB(120, 120, 120);
const SLIDER_PIVOT_SIZE: u32 = 14;
const SLIDER_BAR_DEFAULT_COLOR: Color = Color::RGB(87, 87, 87);
const SLIDER_BAR_SIZE: u32 = 8;

const BUTTON_STATE_TRUE: Color = Color::RGB(30, 165, 30);
const BUTTON_STATE_FALSE: Color = Color::RGB(165, 30, 30);
const BUTTON_DEFAULT_COLOR: Color = Color::RGB(120, 120, 120);
const BUTTON_RECT_SIZE: u32 = 24;
const BUTTON_RECT_STATE_SIZE: u32 = 16;

///Hmm... Primitives....
pub trait PrimitiveNumber: Copy + PartialOrd + std::fmt::Debug + std::ops::Div {
    fn as_f32(self) -> f32;
    fn from_f32(value: f32) -> Self;
}

impl PrimitiveNumber for u8 {
    fn as_f32(self) -> f32 {
        self as f32
    }
    fn from_f32(value: f32) -> u8 {
        value as u8
    }
}
impl PrimitiveNumber for i8 {
    fn as_f32(self) -> f32 {
        self as f32
    }
    fn from_f32(value: f32) -> i8 {
        value as i8
    }
}
impl PrimitiveNumber for u16 {
    fn as_f32(self) -> f32 {
        self as f32
    }
    fn from_f32(value: f32) -> u16 {
        value as u16
    }
}
impl PrimitiveNumber for i16 {
    fn as_f32(self) -> f32 {
        self as f32
    }
    fn from_f32(value: f32) -> i16 {
        value as i16
    }
}
impl PrimitiveNumber for u32 {
    fn as_f32(self) -> f32 {
        self as f32
    }
    fn from_f32(value: f32) -> u32 {
        value as u32
    }
}
impl PrimitiveNumber for i32 {
    fn as_f32(self) -> f32 {
        self as f32
    }
    fn from_f32(value: f32) -> i32 {
        value as i32
    }
}
impl PrimitiveNumber for u64 {
    fn as_f32(self) -> f32 {
        self as f32
    }
    fn from_f32(value: f32) -> u64 {
        value as u64
    }
}
impl PrimitiveNumber for i64 {
    fn as_f32(self) -> f32 {
        self as f32
    }
    fn from_f32(value: f32) -> i64 {
        value as i64
    }
}
impl PrimitiveNumber for f32 {
    fn as_f32(self) -> f32 {
        self
    }
    fn from_f32(value: f32) -> f32 {
        value
    }
}
impl PrimitiveNumber for f64 {
    fn as_f32(self) -> f32 {
        self as f32
    }
    fn from_f32(value: f32) -> f64 {
        value as f64
    }
}
impl PrimitiveNumber for usize {
    fn as_f32(self) -> f32 {
        self as f32
    }
    fn from_f32(value: f32) -> usize {
        value as usize
    }
}
impl PrimitiveNumber for isize {
    fn as_f32(self) -> f32 {
        self as f32
    }
    fn from_f32(value: f32) -> isize {
        value as isize
    }
}

pub trait Draw {
    fn draw(&self, display: &mut BoxDisplay) -> Result<(), sdl3::Error>;
    ///draw_cl is the same as draw(), but with color!
    fn draw_cl(&self, display: &mut BoxDisplay, color: Color) -> Result<(), sdl3::Error>;
    fn draw_outline(&self, display: &mut BoxDisplay, color: Color) -> Result<(), sdl3::Error>;
}

pub struct SdlContext {
    pub sdl: Sdl,
    pub event_pump: EventPump,
    pub event_subsystem: EventSubsystem,
    pub video_subsystem: VideoSubsystem,
    pub audio_subsystem: AudioSubsystem,
}

///BoxDisplay was initially where all the render magic happened, now it is being planned to be
///transformed into a Trait for a more modular, and borrowing friendly system.
pub struct BoxDisplay {
    pub canvas: WindowCanvas,
    //    pub texture_creator: TextureCreator<WindowContext> //Never use this as a reference!
    pub fps_limit: f64,
}

///Write works like any Context of SDL with the responsibility to render texts with specific
///fonts. If you want to use multiple fonts, create various Writes.
pub struct Write<'ttf, 'r, 'render> {
    pub ttf: &'ttf ttf::Sdl3TtfContext,
    pub font: ttf::Font<'ttf, 'r>,
    pub texture_creator: &'render TextureCreator<WindowContext>,
}

pub enum SliderType {
    SliderHorizontal,
    SliderVertical,
}

///The slider is a user input element, where the user moves a pivot to control the value.
pub struct Slider<T: PrimitiveNumber> {
    value: T,
    min: T,
    max: T,
    pub x: f32,
    pub y: f32,
    length: u32,
    slider_type: SliderType,
}

pub struct Button {
    pub state: bool,
    pub x: f32,
    pub y: f32,
}

///Labels are essentially dynamic text that changes with its string.
///
/// If for some reason you want to use multiple screens in one application, keep in mind to use the
///same one in its context.
///
/// Labels are also somewhat expensive since they create a texture every single frame. If you want
///a static text, use Write directly.
///
/// Warning! Label can only be used in the same BoxDisplay that Write has been created!
pub struct Label<'render, 'w, 'ttf, 'r> {
    string: String,
    write: &'w Write<'ttf, 'r, 'render>,
    pub size: u32,
    x: f32,
    y: f32,
}

impl SdlContext {
    pub fn init_context() -> SdlContext {
        let sdl = sdl3::init().unwrap();

        SdlContext {
            event_pump: sdl.event_pump().unwrap(),
            event_subsystem: sdl.event().unwrap(),
            video_subsystem: sdl.video().unwrap(),
            audio_subsystem: sdl.audio().unwrap(),
            sdl,
        }
    }

    pub fn send_quit(&self) -> Result<(), sdl3::Error> {
        self.event_subsystem
            .push_event(Event::Quit { timestamp: 0 })
    }

    ///Returns true if there is a Quit Event and false if not.
    /// (WARNING this consumes the entire EventPoll.
    /// This is probably going to be DEPRECATED in the near future!)
    pub fn check_quit(&mut self) -> bool {
        self.event_pump
            .poll_iter()
            .any(|quit| matches!(quit, Event::Quit { .. }))
    }

    ///Returns true if there is a DropFile Event and false if not.
    /// (WARNING this consumes the entire EventPoll.
    /// This is probably going to be DEPRECATED in the near future!)
    pub fn check_file_drop(&mut self) -> bool {
        self.event_pump
            .poll_iter()
            .any(|quit| matches!(quit, Event::DropFile { .. }))
    }
}

impl BoxDisplay {
    pub fn init_display(video_subsystem: &VideoSubsystem, width: u32, height: u32) -> BoxDisplay {
        let window = video_subsystem
            .window("BOX", width, height)
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas();
        let fps_limit: f64 = 60.0;
        BoxDisplay {
            //            texture_creator: canvas.texture_creator(),
            canvas,
            fps_limit,
        }
    }

    pub fn sleep(&self) {
        let sleep_time = std::time::Duration::from_secs_f64(1.0 / self.fps_limit);
        std::thread::sleep(sleep_time)
    }

    pub fn draw_outline(&mut self, rect: &FRect) -> Result<(), sdl3::Error> {
        let outline = FRect::new(
            rect.x - 2.0,
            rect.y - 2.0,
            rect.w + 4.0,
            rect.h + 4.0,
        );
        self.canvas.set_draw_color(COLOR_WHITE);
        self.canvas.fill_rect(outline)?;
        Ok(())
    }

    pub fn draw_geometry<P: Into<FPoint>>(
        &mut self,
        pos: P,
        vertices: u8,
        size: f32,
    ) -> Result<(), sdl3::Error> {
        let pos: FPoint = pos.into();
        let mut vert: Vec<FPoint> = geometry(pos, vertices, size);

        vert.push(*vert.first().unwrap());

        self.canvas.draw_lines(vert.as_slice())?;
        Ok(())
    }

    pub fn draw_geometry_points<P: Into<FPoint>>(
        &mut self,
        pos: P,
        vertices: u8,
        size: f32,
    ) -> Result<(), sdl3::Error> {
        let pos: FPoint = pos.into();
        let mut vert: Vec<FPoint> = geometry(pos, vertices, size);

        vert.push(*vert.first().unwrap());

        self.canvas.draw_points(vert.as_slice())?;
        Ok(())
    }

    pub fn draw_angle<P: Into<FPoint>>(
        &mut self,
        pos: P,
        angle: f32,
        distance: f32,
    ) -> Result<(), sdl3::Error> {
        let pos1: FPoint = pos.into();
        let pos2: FPoint = angle_fpoint(pos1, angle, distance);

        self.canvas.draw_line(pos1, pos2)?;
        Ok(())
    }

    ///Deprecated, use draw_angle() directly — sdl3 now enforces the use of FPoint in all circumstances.
    pub fn draw_angle_float<P: Into<FPoint>>(
        &mut self,
        pos: P,
        angle: f32,
        distance: f32,
    ) -> Result<(), sdl3::Error> {
        let pos1: FPoint = pos.into();
        let pos2: FPoint = angle_fpoint(pos1, angle, distance);

        self.canvas.draw_line(pos1, pos2)?;
        Ok(())
    }

    pub fn width(&self) -> u32 {
        let window = self.canvas.window();
        window.size().0
    }

    pub fn width_center(&self) -> u32 {
        let window = self.canvas.window();
        window.size().0 / 2
    }

    pub fn height(&self) -> u32 {
        let window = self.canvas.window();
        window.size().1
    }

    pub fn height_center(&self) -> u32 {
        let window = self.canvas.window();
        window.size().1 / 2
    }

    pub fn width_f(&self) -> f32 {
        let window = self.canvas.window();
        window.size().0 as f32
    }

    pub fn width_center_f(&self) -> f32 {
        let window = self.canvas.window();
        (window.size().0 / 2) as f32
    }

    pub fn height_f(&self) -> f32 {
        let window = self.canvas.window();
        window.size().1 as f32
    }

    pub fn height_center_f(&self) -> f32 {
        let window = self.canvas.window();
        (window.size().1 / 2) as f32
    }
}

impl<'t, 'f, 'render> Write<'t, 'f, 'render> {
    pub fn init_write(
        ttf: &'t ttf::Sdl3TtfContext,
        texture_creator: &'render TextureCreator<WindowContext>,
        font: &str,
    ) -> Write<'t, 'f, 'render> {
        let mut path = PathBuf::from(get_assets_path());
        path.push(font);

        let font = match ttf.load_font(path, 32.0) {
            Ok(font_src) => font_src,
            Err(damn) => panic!("ERROR: {}", &damn),
        };
        Write {
            ttf,
            font,
            texture_creator,
        }
    }

    pub fn create_text(
        &self,
        string: &str,
        color: Color,
    ) -> Result<Texture<'render>, TextureValueError> {
        self.font
            .render(string)
            .solid(color)
            .unwrap()
            .as_texture(self.texture_creator)
    }
}

impl<T: PrimitiveNumber> Slider<T> {
    pub fn new(min: T, max: T, x: f32, y: f32, length: u32, slider_type: SliderType) -> Slider<T> {
        Slider {
            min,
            max,
            value: min,
            x,
            y,
            length,
            slider_type,
        }
    }

    pub fn set_value(&mut self, value: T)
    where
        T: PrimitiveNumber,
    {
        self.value = value;
    }

    ///Recommended for controlled values.
    pub fn set_value_limited(&mut self, value: T)
    where
        T: PrimitiveNumber,
    {
        if value < self.min {
            self.value = self.min
        } else if value > self.max {
            self.value = self.max
        } else {
            self.value = value
        };
    }

    pub fn get_value_ref(&self) -> &T {
        &self.value
    }

    pub fn get_value_mut_ref(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn from_value(&self) -> T {
        self.value
    }

    ///Mutates the given value from the value of the slider.
    pub fn mut_from_slider<F: PrimitiveNumber>(&self, value: &mut F) {
        *value = PrimitiveNumber::from_f32(self.value.as_f32());
    }

    pub fn get_type(&self) -> &SliderType {
        &self.slider_type
    }

    ///Returns how filled is the slider.
    pub fn percentage(&self) -> f32 {
        (self.value.as_f32() - self.min.as_f32()) / (self.max.as_f32() - self.min.as_f32())
    }

    ///Returns a position at the beginning of the slider.
    fn start_pos(&self) -> FPoint {
        match self.slider_type {
            SliderType::SliderHorizontal => FPoint::new(self.x - self.length as f32 / 2.0, self.y),
            SliderType::SliderVertical =>   FPoint::new(self.x, self.y - self.length as f32 / 2.0),
        }
    }

    ///Calculates and returns the position of the pivot.
    pub fn pivot(&self) -> FPoint {
        let pos = self.start_pos();
        match &self.slider_type {
            SliderType::SliderHorizontal => FPoint::new(
                pos.x + self.length as f32 * self.percentage(),
                pos.y,
            ),
            SliderType::SliderVertical => FPoint::new(
                pos.x,
                pos.y + self.length as f32 - self.length as f32 * self.percentage(),
            ),
        }
    }

    ///Same as pivot(). Kept for compatibility — pivot() now returns FPoint directly.
    #[deprecated = "Use pivot() instead. Previously this function was used to return a float point, but pivot() now returns that by default."]
    pub fn pivot_f(&self) -> FPoint {
        self.pivot()
    }

    ///Returns the FRect of the pivot.
    pub fn pivot_rect(&self) -> FRect {
        let pos = self.pivot();
        FRect::new(
            pos.x - SLIDER_PIVOT_SIZE as f32 / 2.0,
            pos.y - SLIDER_PIVOT_SIZE as f32 / 2.0,
            SLIDER_PIVOT_SIZE as f32,
            SLIDER_PIVOT_SIZE as f32,
        )
    }

    ///Returns the FRect of the Bar.
    pub fn bar_rect(&self) -> FRect {
        let pos = self.start_pos();
        match self.slider_type {
            SliderType::SliderHorizontal => FRect::new(
                pos.x,
                pos.y - SLIDER_BAR_SIZE as f32 / 2.0,
                self.length as f32,
                SLIDER_BAR_SIZE as f32,
            ),
            SliderType::SliderVertical => FRect::new(
                pos.x - SLIDER_BAR_SIZE as f32 / 2.0,
                pos.y,
                SLIDER_BAR_SIZE as f32,
                self.length as f32,
            ),
        }
    }

    ///This will be like this until I have the patience to add more SliderTypes o7
    pub fn update_from_pos<P: Into<FPoint>>(&mut self, point: P) {
        let pos: FPoint = self.start_pos();
        let point: FPoint = point.into();

        let distance: f32 = match self.slider_type {
            SliderType::SliderHorizontal => point.x - pos.x,
            SliderType::SliderVertical => (pos.y + self.length as f32) - point.y,
        }; /*^^Extra orientation calculus^^.*/

        let value: T = PrimitiveNumber::from_f32(
            //This formula makes so negative minimal values are possible.
            self.min.as_f32()
                + (distance / self.length as f32) * (self.max.as_f32() - self.min.as_f32()),
        );
        self.set_value_limited(value);
    }
}

impl<T: PrimitiveNumber> Draw for Slider<T> {
    fn draw(&self, display: &mut BoxDisplay) -> Result<(), sdl3::Error> {
        display.canvas.set_draw_color(SLIDER_BAR_DEFAULT_COLOR);
        display.canvas.fill_rect(self.bar_rect())?;
        display.canvas.set_draw_color(SLIDER_PIVOT_COLOR);
        display.canvas.fill_rect(self.pivot_rect())?;
        display.canvas.set_draw_color(DEFAULT_COLOR);
        Ok(())
    }
    fn draw_cl(&self, display: &mut BoxDisplay, color: Color) -> Result<(), sdl3::Error> {
        display.canvas.set_draw_color(color);
        display.canvas.fill_rect(self.bar_rect())?;
        display.canvas.set_draw_color(SLIDER_PIVOT_COLOR);
        display.canvas.fill_rect(self.pivot_rect())?;
        display.canvas.set_draw_color(DEFAULT_COLOR);
        Ok(())
    }
    fn draw_outline(&self, display: &mut BoxDisplay, color: Color) -> Result<(), sdl3::Error> {
        let bar_rect: FRect = self.bar_rect();
        let pivot_rect: FRect = self.pivot_rect();

        let bar_outline: FRect = FRect::new(
            bar_rect.x - 2.0,
            bar_rect.y - 2.0,
            bar_rect.w + 4.0,
            bar_rect.h + 4.0,
        );
        let pivot_outline: FRect = FRect::new(
            pivot_rect.x - 2.0,
            pivot_rect.y - 2.0,
            pivot_rect.w + 4.0,
            pivot_rect.h + 4.0,
        );

        display.canvas.set_draw_color(color);
        display.canvas.fill_rects(&[bar_outline, pivot_outline])?;
        display.canvas.set_draw_color(DEFAULT_COLOR);
        Ok(())
    }
}

impl Button {
    pub fn new(state: bool, x: f32, y: f32) -> Button {
        Button { state, x, y }
    }

    pub fn set_state(&mut self, state: bool) {
        self.state = state;
    }

    pub fn get_state(&self) -> bool {
        self.state
    }

    pub fn get_state_color(&self) -> Color {
        if self.state {
            BUTTON_STATE_TRUE
        } else {
            BUTTON_STATE_FALSE
        }
    }

    pub fn set_pos<P: Into<FPoint>>(&mut self, point: P) {
        let point: FPoint = point.into();
        self.x = point.x;
        self.y = point.y;
    }

    pub fn toggle(&mut self) {
        self.state = !self.state;
    }

    pub fn rect(&self) -> FRect {
        FRect::new(
            self.x - BUTTON_RECT_SIZE as f32 / 2.0,
            self.y - BUTTON_RECT_SIZE as f32 / 2.0,
            BUTTON_RECT_SIZE as f32,
            BUTTON_RECT_SIZE as f32,
        )
    }

    pub fn state_rect(&self) -> FRect {
        FRect::new(
            self.x - BUTTON_RECT_STATE_SIZE as f32 / 2.0,
            self.y - BUTTON_RECT_STATE_SIZE as f32 / 2.0,
            BUTTON_RECT_STATE_SIZE as f32,
            BUTTON_RECT_STATE_SIZE as f32,
        )
    }
}

impl Draw for Button {
    fn draw(&self, display: &mut BoxDisplay) -> Result<(), sdl3::Error> {
        display.canvas.set_draw_color(BUTTON_DEFAULT_COLOR);
        display.canvas.fill_rect(self.rect())?;
        display.canvas.set_draw_color(self.get_state_color());
        display.canvas.fill_rect(self.state_rect())?;
        display.canvas.set_draw_color(DEFAULT_COLOR);
        Ok(())
    }
    fn draw_cl(&self, display: &mut BoxDisplay, color: Color) -> Result<(), sdl3::Error> {
        display.canvas.set_draw_color(color);
        display.canvas.fill_rect(self.rect())?;
        display.canvas.set_draw_color(self.get_state_color());
        display.canvas.fill_rect(self.state_rect())?;
        display.canvas.set_draw_color(DEFAULT_COLOR);
        Ok(())
    }
    fn draw_outline(&self, display: &mut BoxDisplay, color: Color) -> Result<(), sdl3::Error> {
        let button_rect: FRect = self.rect();
        let outline = FRect::new(
            button_rect.x - 2.0,
            button_rect.y - 2.0,
            button_rect.w + 4.0,
            button_rect.h + 4.0,
        );
        display.canvas.set_draw_color(color);
        display.canvas.fill_rect(outline)?;
        display.canvas.set_draw_color(DEFAULT_COLOR);
        Ok(())
    }
}

impl<'render, 'w, 'ttf, 'r> Label<'render, 'w, 'ttf, 'r> {
    pub fn new(
        x: f32,
        y: f32,
        size: u32,
        write: &'w Write<'ttf, 'r, 'render>,
        string: Option<String>,
    ) -> Label<'render, 'w, 'ttf, 'r> {
        let string: String = string.unwrap_or_default();
        Label {
            string,
            write,
            size,
            x: x.round(), //For text not display incorrectly and blurry, we must round
            y: y.round(), //the numbers.
        }
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn change_write(&mut self, write: &'w Write<'ttf, 'r, 'render>) {
        self.write = write;
    }
    ///Takes ownership of the given String!
    pub fn update_text(&mut self, string: String) {
        self.string = string;
    }
    fn update_texture(&self, color: Option<Color>) -> Result<Texture<'render>, TextureValueError> {
        self.write
            .create_text(&self.string, color.unwrap_or(DEFAULT_COLOR))
    }

    fn error_texture<'t>(&'t self) -> Texture<'t> {
        self.write
            .font
            .render("!ERROR!")
            .solid(COLOR_RED)
            .unwrap()
            .as_texture(self.write.texture_creator)
            .expect("MAJOR FAIL ON MAKING TEXTURES WITH TEXT!")
    }
}

impl Draw for Label<'_, '_, '_, '_> {
    fn draw(&self, display: &mut BoxDisplay) -> Result<(), sdl3::Error> {
        let area = FRect::new(
            self.x - self.string.len() as f32 * (self.size as f32 / 2.0),
            self.y - self.size as f32 / 2.0,
            self.string.len() as f32 * self.size as f32,
            self.size as f32 * 2.0,
        );
        let texture = self.update_texture(None).unwrap_or_else(|err| {
            println!("Failed to create text Texture: {:?}", err);
            self.error_texture()
        });
        display.canvas.copy(&texture, None, area)
    }
    fn draw_cl(&self, display: &mut BoxDisplay, color: Color) -> Result<(), sdl3::Error> {
        let area = FRect::new(
            self.x - self.string.len() as f32 * (self.size as f32 / 2.0),
            self.y,
            self.string.len() as f32 * self.size as f32,
            self.size as f32 * 2.0,
        );
        let texture = self.update_texture(Some(color)).unwrap_or_else(|err| {
            println!("Failed to create text Texture: {:?}", err);
            self.error_texture()
        });
        display.canvas.copy(&texture, None, area)
    }
    ///This function will only draw a rectangle on the text.
    fn draw_outline(&self, display: &mut BoxDisplay, color: Color) -> Result<(), sdl3::Error> {
        let area = FRect::new(
            self.x - self.string.len() as f32 * (self.size as f32 / 2.0),
            self.y - self.size as f32 / 2.0 - 2.0,
            self.string.len() as f32 * self.size as f32 + 2.0,
            self.size as f32 * 2.0 + 2.0,
        );
        display.canvas.set_draw_color(color);
        display.canvas.fill_rect(area)?;
        display.canvas.set_draw_color(DEFAULT_COLOR);
        Ok(())
    }
}

///return the percentage from 0 to 100 in a value with an int.
pub fn percentage_from_int(value: &i32, max: &i32) -> u8 {
    if *value < 1 {
        return 0;
    }
    let over = *value * 100 / *max; //The over value is used to check if is bigger than 100;
    if over <= 100 { over as u8 } else { 100 }
}

///Takes a percentage from 0 to 100 and return the possible value.
pub fn int_from_percentage(value: &i32, percentage: &u8) -> i32 {
    *value * *percentage as i32 / 100
}

pub fn get_assets_path() -> String {
    let mut path: PathBuf = PathBuf::from("./");

    //I hate this code so much, but I'm too lazy to think a better way.
    path.push("main_assets");
    match path.try_exists() {
        Ok(true) => {
            //This might break in some occasions, probably will fix when I properly learn OsString.
            return path
                .into_os_string()
                .into_string()
                .expect("Could not transform OsString into String!");
        }
        Ok(false) => path.pop(),
        Err(_) => panic!("COULD NOT VERIFY PATH! POSSIBLY A PERMISSION PROBLEM!"),
    };
    path.push("src/main_assets");
    match path.try_exists() {
        Ok(true) => path
            .into_os_string()
            .into_string()
            .expect("Could not transform OsString into String!"),
        Ok(false) => panic!("THE FOLDER [main_assets] DOES NOT EXIST!"),
        Err(_) => panic!("COULD NOT VERIFY PATH! POSSIBLY A PERMISSION PROBLEM!"),
    }
}

pub fn get_asset_file(file: &str) -> Result<fs::File, io::Error> {
    let path: String = format!("{}/{}", get_assets_path(), file);
    fs::File::open(path)
}

///Return a point based mainly on angle and the distance given.
pub fn angle_point<P: Into<FPoint>>(point: P, angle: f32, distance: f32) -> FPoint {
    let point: FPoint = point.into();
    FPoint::new(
        point.x + distance * angle.to_radians().sin(),
        point.y + -(distance * angle.to_radians().cos()), //The - makes sure it stays on raster format.
    )
}

///Same as angle_point(), but with radians instead.
pub fn angler_point<P: Into<FPoint>>(point: P, angle: f32, distance: f32) -> FPoint {
    let point: FPoint = point.into();
    FPoint::new(
        point.x + distance * angle.sin(),
        point.y + -(distance * angle.cos()),
    )
}

///Same as angle_point(), but for a FPoint.
pub fn angle_fpoint<P: Into<FPoint>>(point: P, angle: f32, distance: f32) -> FPoint {
    let point: FPoint = point.into();
    FPoint::new(
        point.x + (distance * angle.to_radians().sin()),
        point.y + -(distance * angle.to_radians().cos()),
    )
}

///Same as angle_fpoint(), but with radians instead.
pub fn angler_fpoint<P: Into<FPoint>>(point: P, angle: f32, distance: f32) -> FPoint {
    let point: FPoint = point.into();
    FPoint::new(
        point.x + (distance * angle.sin()),
        point.y + -(distance * angle.cos()),
    )
}

///Return a 1.0 to -1.0 difference, being 1.0 meaning its equal and -1.0 with opposite directions.
pub fn angle_difference_cos(angle: f32, counter_angle: f32) -> f32 {
    (counter_angle - angle).to_radians().cos()
}

///Same as angle_difference_cos(), but with radians instead.
pub fn angler_difference_cos(angle: f32, counter_angle: f32) -> f32 {
    (counter_angle - angle).cos()
}

///Uses sin instead of the default cos. 1.0 means its 90 degrees clockwise and -1.0 the opposite
pub fn angle_difference_sin(angle: f32, counter_angle: f32) -> f32 {
    (counter_angle - angle).to_radians().sin()
}

///Same as angle_difference_sin(), but with radians instead.
pub fn angler_difference_sin(angle: f32, counter_angle: f32) -> f32 {
    (counter_angle - angle).sin()
}

///Creates points for a basic geometry based on the vertices. For example, 3 vertices would give a triangle.
pub fn geometry<P: Into<FPoint>>(pos: P, vertices: u8, size: f32) -> Vec<FPoint> {
    let pos: FPoint = pos.into();
    let mut edges: Vec<FPoint> = Vec::new();
    let angle_difference: f32 = (360.0 / vertices as f32).to_radians();

    for i in 0..vertices {
        edges.push(angler_fpoint(pos, angle_difference * i as f32, size));
    }
    edges
}

///This function receives an image file, (PNGs for now) and transforms them into a Texture.
///
/// Note: DO NOT ignore the error that comes from it.
pub fn texture_from_file<'t, Render>(
    file: fs::File,
    texture_creator: &'t TextureCreator<Render>,
) -> Result<Texture<'t>, String> {
    let mut reader = png_reader(file)?;

    let mut buffer = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buffer).unwrap();
    let pixel_format = translate_color_format(info.color_type);

    let mut image = texture_creator
        .create_texture_static(pixel_format, info.width, info.height)
        .unwrap();
    let pitch: usize = pixel_format.byte_size_per_pixel() * info.width as usize;
    match info.color_type {
        png::ColorType::Rgba => image.set_blend_mode(BlendMode::Blend),
        png::ColorType::Grayscale => {
            convert_from_greyscale(&mut buffer);
            image.set_blend_mode(BlendMode::Blend);
        }
        png::ColorType::GrayscaleAlpha => {
            convert_from_greyscale_alpha(&mut buffer);
            image.set_blend_mode(BlendMode::Blend);
        }
        _ => {}
    }

    image
        .update(None, &buffer, pitch)
        .map_err(|err| match err {
            UpdateTextureError::PitchOverflows(overflow) => {
                format!("Pitch has overflowed!: {overflow}")
            }
            UpdateTextureError::PitchMustBeMultipleOfTwoForFormat(num, format) => format!(
                "Pitch must be multiple of two for format! pitch: {:?} format: {:?}",
                num, format
            ),
            UpdateTextureError::XMustBeMultipleOfTwoForFormat(num, format) => format!(
                "Pitch must be multiple of two for format! pitch: {:?} format: {:?}",
                num, format
            ),
            UpdateTextureError::YMustBeMultipleOfTwoForFormat(num, format) => format!(
                "Pitch must be multiple of two for format! pitch: {:?} format: {:?}",
                num, format
            ),
            UpdateTextureError::WidthMustBeMultipleOfTwoForFormat(num, format) => format!(
                "Pitch must be multiple of two for format! pitch: {:?} format: {:?}",
                num, format
            ),
            UpdateTextureError::HeightMustBeMultipleOfTwoForFormat(num, format) => format!(
                "Pitch must be multiple of two for format! pitch: {:?} format: {:?}",
                num, format
            ),
            UpdateTextureError::SdlError(err) => err.to_string(),
        })
        .map(|()| image)
}

fn png_reader<R: io::Read>(file: R) -> Result<png::Reader<R>, String> {
    png::Decoder::new(file)
        .read_info()
        .map_err(|err| match err {
            png::DecodingError::IoError(error) => error.to_string(),
            png::DecodingError::Format(_) => String::from("The PNG format is invalid!"),
            png::DecodingError::Parameter(error) => error.to_string(),
            png::DecodingError::LimitsExceeded => String::from("LIMITS EXCEEDED!"),
        })
}

fn translate_color_format(color_type: png::ColorType) -> sdl3::pixels::PixelFormat {
    use sdl3::pixels::PixelFormat;
    use sdl3_sys::pixels::SDL_PixelFormat;

    match color_type {
        png::ColorType::Grayscale => PixelFormat::from(SDL_PixelFormat::RGB24.0 as i64),
        png::ColorType::GrayscaleAlpha => PixelFormat::from(SDL_PixelFormat::RGBA32.0 as i64),
        png::ColorType::Indexed => PixelFormat::from(SDL_PixelFormat::INDEX8.0 as i64),
        png::ColorType::Rgb => PixelFormat::from(SDL_PixelFormat::RGB24.0 as i64),
        png::ColorType::Rgba => PixelFormat::from(SDL_PixelFormat::RGBA32.0 as i64),
    }
}

fn convert_from_greyscale(vec: &mut Vec<u8>) {
    let mut buffer: Vec<u8> = Vec::with_capacity(vec.len() * 3);
    for chunk in vec.iter() {
        buffer.extend_from_slice(&[*chunk, *chunk, *chunk]);
    }
    *vec = buffer;
}

fn convert_from_greyscale_alpha(vec: &mut Vec<u8>) {
    let mut buffer: Vec<u8> = Vec::with_capacity(vec.len() * 2);
    for chunk in vec.chunks(2) {
        buffer.extend_from_slice(&[chunk[0], chunk[0], chunk[0], chunk[1]]);
    }
    *vec = buffer;
}