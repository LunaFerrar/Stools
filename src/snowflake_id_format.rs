use egui::Ui;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct SnowflakeIdFormat {
    id_to_format: String,
}

impl SnowflakeIdFormat {
    pub fn ui(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::TextEdit::multiline(&mut self.id_to_format)
                .hint_text("put the IDs to format here :)")
                .show(ui);

            let formated_text = course_format_full(&self.id_to_format);

            ui.label(formated_text);
        });
    }
}
/* Previous parsing code
pub fn course_format(list_course: &str) -> String {
    let to_format = list_course;
    let vec_of_courses: Vec<&str> = to_format.split(' ').collect();
    let mut final_string: String = String::new();
    for item in vec_of_courses {
        if final_string.is_empty() {
            final_string = final_string.to_owned() + item
        } else {
            final_string = final_string.to_owned() + ", " + item
        }
    }
    final_string
}
*/

pub fn course_format_full(list_course: &str) -> String {
    let to_format = list_course;
    let mut final_string: String= "'".to_string();

    if to_format.is_empty() {
        final_string
    } else {
        let mut previous_char = to_format.chars().nth(0).unwrap();
        for item in to_format.chars() {
            if item.is_alphanumeric() && item != ' ' {
                final_string = final_string + &item.to_string();
            } else {
                if previous_char.is_alphanumeric() {
                    if !final_string.is_empty() {
                        final_string = final_string + "', '"
                    }
                }
            }
            previous_char = item;
        }
        final_string = final_string + "'";
        final_string
    }
}
