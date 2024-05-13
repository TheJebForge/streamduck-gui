use egui::{Button, Color32, Frame, pos2, Rect, RichText, ScrollArea, Stroke, Ui, vec2};
use tokio::sync::mpsc::Sender;
use crate::ui::{UIMessage, UIState};
use crate::ui::util::send_ui_message;

pub fn stack_line(ui: &mut Ui, state: &mut UIState, sender: &Sender<UIMessage>) {
    let button_width = ui.available_height();
    let margin = 10f32;

    let all_width = ui.available_width();
    let all_height = ui.available_height();

    let stack_width = all_width - button_width - margin;

    let next_pos = ui.available_rect_before_wrap().min;

    let left_rect = Rect::from_min_size(
        next_pos,
        vec2(stack_width, all_height),
    );

    let right_rect = Rect::from_min_size(
        pos2(
            next_pos.x + stack_width + margin,
            next_pos.y,
        ),
        vec2(button_width, all_height),
    );

    ui.allocate_ui_at_rect(left_rect, |ui| {
        Frame::default()
            .fill(Color32::from_rgb(40, 40, 40))
            .rounding(10.0)
            .inner_margin(10.0)
            .show(ui, |ui| {
                ScrollArea::horizontal()
                    .show(ui, |ui| {
                        ui.add_space(10.0);
                        for (index, stack_item) in state.device_editor.stack.iter().enumerate() {
                            if index != 0 {
                                ui.add_space(6.0);
                                ui.label(RichText::new("\u{eab6}").line_height(Some(16.0)));
                            }

                            ui.label(RichText::new(stack_item).line_height(Some(19.0)));
                        }
                        ui.add_space(10.0);
                    });
            });
    });

    ui.allocate_ui_at_rect(right_rect, |ui| {
        if ui.add(
                Button::new(
                    RichText::new("\u{f148}")
                        .line_height(Some(20f32))
                        .size(16f32)
                ).min_size(ui.available_size())
                    .rounding(10f32)
            ).clicked() {
            send_ui_message(sender, UIMessage::PopScreen(state.device_editor.device.clone()));
        }
    });
}