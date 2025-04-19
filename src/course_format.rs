use egui::Ui;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct CourseFormat {
    text_to_format: String,
}

impl CourseFormat {
    pub fn ui(&mut self, ui: &mut Ui) {
        egui::TextEdit::multiline(&mut self.text_to_format)
            .hint_text("put the codes to format here :)")
            .show(ui);

        let formated_text = course_format_full(&self.text_to_format);

        ui.label(formated_text);
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
    let mut final_string: String = String::new();

    if to_format.is_empty() {
        final_string
    } else {
        let mut previous_char = to_format.chars().nth(0).unwrap();
        for item in to_format.chars() {
            if item.is_alphanumeric() {
                final_string = final_string + &item.to_string();
            } else {
                if previous_char.is_alphabetic() {
                    if !final_string.is_empty() {
                        final_string = final_string + ", "
                    }
                }
            }
            previous_char = item;
        }

        final_string
    }
}
