table! {
    document (id) {
        id -> Int8,
        label -> Varchar,
        document_type_id -> Int8,
    }
}

table! {
    document_file (id) {
        id -> Int8,
        label -> Varchar,
        document_id -> Int8,
        version -> Varchar,
        filename -> Varchar,
    }
}

table! {
    document_tag (id) {
        id -> Int8,
        tag_id -> Int8,
        document_id -> Int8,
    }
}

table! {
    document_type (id) {
        id -> Int8,
        label -> Varchar,
    }
}

table! {
    tag (id) {
        id -> Int8,
        label -> Varchar,
        color -> Varchar,
    }
}

joinable!(document -> document_type (document_type_id));
joinable!(document_file -> document (document_id));
joinable!(document_tag -> document (document_id));
joinable!(document_tag -> tag (tag_id));

allow_tables_to_appear_in_same_query!(
    document,
    document_file,
    document_tag,
    document_type,
    tag,
);
