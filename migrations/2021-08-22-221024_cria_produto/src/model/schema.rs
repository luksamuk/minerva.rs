table! {
    cliente (id) {
        id -> Int4,
        tipo -> Int2,
        nome -> Varchar,
        pj -> Bool,
        docto -> Varchar,
        ativo -> Bool,
        bloqueado -> Bool,
    }
}

table! {
    endereco (id) {
        id -> Int4,
        cliente_id -> Int4,
        tipo -> Int2,
        logradouro -> Varchar,
        numero -> Varchar,
        complemento -> Nullable<Varchar>,
        bairro -> Varchar,
        uf -> Varchar,
        cidade -> Varchar,
    }
}

table! {
    produto (id) {
        id -> Int4,
        descricao -> Varchar,
        unidsaida -> Varchar,
        qtdestoque -> Numeric,
        precovenda -> Numeric,
    }
}

joinable!(endereco -> cliente (cliente_id));

allow_tables_to_appear_in_same_query!(
    cliente,
    endereco,
    produto,
);
