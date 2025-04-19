use egui::Ui;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct CourseFormat {
    text_to_format: String,
}

impl CourseFormat {

    pub fn ui (&mut self, ui: &mut Ui) {
        egui::TextEdit::multiline(&mut self.text_to_format).hint_text("put the codes to format here :)").show(ui);

        let formated_text = course_format(& self.text_to_format);

        ui.label(formated_text);
    }
    
}

pub fn course_format (list_course: &str) -> String {
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