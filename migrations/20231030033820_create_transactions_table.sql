CREATE TABLE IF NOT EXISTS "transactions" (
    "id" TEXT PRIMARY KEY NOT NULL,
    "amount" DECIMAL(11, 2) NOT NULL,
    "description" TEXT,
    "payee" TEXT NOT NULL
);
