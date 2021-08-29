CREATE TABLE LOGDB (
       ID        SERIAL      PRIMARY KEY,
       TABELA    VARCHAR     NOT NULL,
       USUARIO   VARCHAR     NOT NULL,
       OPERACAO  SMALLINT    NOT NULL,
       DATAHORA  TIMESTAMPTZ NOT NULL,
       DESCRICAO VARCHAR
)
