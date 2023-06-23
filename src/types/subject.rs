use super::Subjects;

impl Subjects {
    pub fn translate_to_russian(&self)->String{
        match self {
            Subjects::Math => "Математика",
            Subjects::Geometry => "Геометрия",
            Subjects::RussianLang => "Русский язык",
            Subjects::UzbekLang => "Узбекский язык",
            Subjects::EnglishLang => "Английский язык",
            Subjects::WorldHistory => "Всемирная история",
            Subjects::HistoryOfUzbekistan => "История Узбекистана",
            Subjects::Biology => "Биология",
            Subjects::Chemistry => "Химия",
            Subjects::Drowing => "Рисование/Черчение",
            Subjects::Physics => "Физика",
            Subjects::Literature => "Чтение/Литература",
            Subjects::Economy => "Природоведение/География/Экономика",
        }.to_string()
    }
}