use penrose_ui::{
    bar::widgets::Widget,
    core::{Context, TextStyle},
    Result,
};
const PADDING: f64 = 3.0;

#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    path: String, 
    width: i32,
    height: i32,
}

impl Workspaces {
    pub fn new(path: String, width: i32, height: i32) -> Self { Self { width, path, height, } }
}

impl<X: XConn> Widget<X> for Workspaces {
    fn draw(
        &mut self,
        ctx: &mut Context,
        screen: usize,
        screen_has_focus: bool,
        w: f64,
        h: f64,
    ) -> Result<()> {
        ctx.color(&self.bg_2);
        ctx.rectangle(0.0, 0.0, w, h);
        ctx.font(&self.font, self.point_size);
        ctx.translate(PADDING, 0.0);
        let (_, eh) = <Self as Widget<X>>::current_extent(self, ctx, h)?;

        for ws in self.workspaces.iter() {
            let (fg, bg) = self.ws_colors(&ws.tag, screen, screen_has_focus, ws.occupied);
            if let Some(c) = bg {
                ctx.color(c);
                ctx.rectangle(0.0, 0.0, ws.extent.0, h)?;
            }

            ctx.color(fg);
            ctx.text(&ws.tag, h - eh, (PADDING, PADDING))?;
            ctx.translate(ws.extent.0, 0.0);
        }

        self.require_draw = false;

        Ok(())
    }

    fn current_extent(&mut self, ctx: &mut Context, _h: f64) -> Result<(f64, f64)> {
        match self.extent {
            Some(extent) => Ok(extent),
            None => {
                let mut total = 0.0;
                let mut h_max = 0.0;
                for ws in self.workspaces.iter_mut() {
                    ctx.font(&self.font, self.point_size)?;
                    let (w, h) = ctx.text_extent(&ws.tag)?;
                    total += w + PADDING + PADDING;
                    h_max = if h > h_max { h } else { h_max };
                    ws.extent = (w + PADDING + PADDING, h);
                }

                let ext = (total + PADDING, h_max);
                self.extent = Some(ext);

                Ok(ext)
            }
        }
    }

    fn is_greedy(&self) -> bool {
        false
    }

    fn require_draw(&self) -> bool {
        self.require_draw
    }

    fn on_startup(&mut self, state: &mut State<X>, _: &X) -> Result<()> {
        self.update_from_state(state);

        Ok(())
    }

    fn on_refresh(&mut self, state: &mut State<X>, _: &X) -> Result<()> {
        self.update_from_state(state);

        Ok(())
    }
}
