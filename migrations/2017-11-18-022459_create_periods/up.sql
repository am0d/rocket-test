-- Your SQL goes here
CREATE TABLE Period (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE,
    previous_period_id INTEGER,
    CONSTRAINT fk_previous_period FOREIGN KEY (previous_period_id)
      REFERENCES public.Period (id)
      ON UPDATE CASCADE ON DELETE SET NULL
);