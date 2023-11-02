DROP TABLE transactions;

CREATE TABLE IF NOT EXISTS "transactions" (
    "id" TEXT PRIMARY KEY NOT NULL,
    "amount" TEXT NOT NULL,
    "description" TEXT,
    "payee" TEXT NOT NULL,
    "timestamp" TEXT NOT NULL
) STRICT;
