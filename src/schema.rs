// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role"))]
    pub struct Role;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "valid_lessons"))]
    pub struct ValidLessons;
}

diesel::table! {
    assignments (id) {
        id -> Int4,
        name -> Citext,
        subject_id -> Int4,
        deadline -> Timestamp,
    }
}

diesel::table! {
    enrols (student_id, level_id) {
        student_id -> Int4,
        level_id -> Int4,
    }
}

diesel::table! {
    grades (teacher_id, assignment_id, reduction_id) {
        teacher_id -> Int4,
        assignment_id -> Int4,
        reduction_id -> Int4,
    }
}

diesel::table! {
    levels (id) {
        id -> Citext,
        year_id -> Int4,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        content -> Text,
        points -> Int4,
        assignment_id -> Int4,
    }
}

diesel::table! {
    reductions (id) {
        id -> Int4,
        content -> Citext,
        points -> Int4,
        question_id -> Int4,
    }
}

diesel::table! {
    sections (id) {
        id -> Citext,
        level_id -> Citext,
    }
}

diesel::table! {
    subjects (id) {
        id -> Int4,
        name -> Citext,
        level_id -> Citext,
    }
}

diesel::table! {
    submits (student_id, assignment_id) {
        url -> Citext,
        token -> Nullable<Citext>,
        assignment_id -> Int4,
        student_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ValidLessons;

    teaches (teacher_id, lesson_type, section_id) {
        teacher_id -> Int4,
        lesson_type -> ValidLessons,
        section_id -> Citext,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Role;

    users (id) {
        id -> Int4,
        email -> Citext,
        role -> Role,
    }
}

diesel::table! {
    years (id) {
        id -> Int4,
    }
}

diesel::joinable!(assignments -> subjects (subject_id));
diesel::joinable!(grades -> assignments (assignment_id));
diesel::joinable!(grades -> reductions (reduction_id));
diesel::joinable!(grades -> users (teacher_id));
diesel::joinable!(levels -> years (year_id));
diesel::joinable!(questions -> assignments (assignment_id));
diesel::joinable!(reductions -> questions (question_id));
diesel::joinable!(sections -> levels (level_id));
diesel::joinable!(subjects -> levels (level_id));
diesel::joinable!(submits -> assignments (assignment_id));
diesel::joinable!(submits -> users (student_id));
diesel::joinable!(teaches -> sections (section_id));
diesel::joinable!(teaches -> users (teacher_id));

diesel::allow_tables_to_appear_in_same_query!(
    assignments,
    enrols,
    grades,
    levels,
    questions,
    reductions,
    sections,
    subjects,
    submits,
    teaches,
    users,
    years,
);
