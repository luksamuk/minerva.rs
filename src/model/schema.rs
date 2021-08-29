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
    logdb (id) {
        id -> Int4,
        tabela -> Varchar,
        usuario -> Varchar,
        operacao -> Int2,
        datahora -> Timestamptz,
        descricao -> Nullable<Varchar>,
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

table! {
    usuario (id) {
        id -> Int4,
        login -> Varchar,
        nome -> Varchar,
        email -> Nullable<Varchar>,
        senha_hash -> Bytea,
    }
}

joinable!(endereco -> cliente (cliente_id));

allow_tables_to_appear_in_same_query!(
    cliente,
    endereco,
    logdb,
    produto,
    usuario,
);
