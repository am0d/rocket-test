-- Your SQL goes here
CREATE TABLE PeriodCategory (
    PeriodId INTEGER NOT NULL,
    CategoryId INTEGER NOT NULL,
    BudgetedAmount INTEGER NOT NULL DEFAULT 0,
    RemainingAmount INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (PeriodId, CategoryId),
    FOREIGN KEY (PeriodId) REFERENCES Period (Id),
    FOREIGN KEY (CategoryId) REFERENCES Category (Id)
);

CREATE TABLE Transaction (
    Id SERIAL PRIMARY KEY,
    Description VARCHAR NOT NULL DEFAULT '',
    TransactionDate TIMESTAMP,
    Amount INTEGER NOT NULL DEFAULT 0, -- Stored as cents, negative / positive
    PeriodId INTEGER REFERENCES Period (Id) NOT NULL,
    CategoryId INTEGER REFERENCES Category (Id) NOT NULL,
    FOREIGN KEY (PeriodId, CategoryId) REFERENCES PeriodCategory (PeriodId, CategoryId)
);