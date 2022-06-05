CREATE TYPE quiz_question_result AS (
    word_id uuid,
    num_correct int,
    num_incorrect int
);

CREATE TABLE quiz_results (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  
    results quiz_question_result[] NOT NULL DEFAULT '{}',
    created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
);
