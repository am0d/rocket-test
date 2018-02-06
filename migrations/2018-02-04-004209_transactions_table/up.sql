-- Your SQL goes here
CREATE TABLE PeriodCategory (
    period_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    budgeted_amount INTEGER NOT NULL DEFAULT 0,
    remaining_amount INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (period_id, category_id),
    FOREIGN KEY (period_id) REFERENCES Period (id),
    FOREIGN KEY (category_id) REFERENCES Category (id)
);

CREATE TABLE Transaction (
    id SERIAL PRIMARY KEY,
    description VARCHAR NOT NULL DEFAULT '',
    transaction_date TIMESTAMP,
    amount INTEGER NOT NULL DEFAULT 0, -- Stored as cents, negative / positive
    period_id INTEGER REFERENCES Period (Id) NOT NULL,
    category_id INTEGER REFERENCES Category (Id) NOT NULL,
    FOREIGN KEY (period_id, category_id) REFERENCES PeriodCategory (period_id, category_id)
);