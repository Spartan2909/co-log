use super::{get_user_input, text};

use chrono::Local;
use dotenvy::dotenv;
use sqlx::{query, query_as, Connection, SqliteConnection};
use std::{
    env,
    io::{self, Write},
    time::{Duration, Instant},
};

/// An answer to a question.
#[derive(Debug, PartialEq)]
enum Answer {
    A,
    B,
    C,
    D,
}

impl TryFrom<String> for Answer {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            _ => Err(()),
        }
    }
}

/// A quiz question.
#[derive(Debug)]
struct Question {
    question: &'static str,
    answer_a: &'static str,
    answer_b: &'static str,
    answer_c: &'static str,
    answer_d: &'static str,
    correct_answer: Answer,
}

impl Question {
    /// Construct a new question from the given parameters.
    const fn new(
        question: &'static str,
        a: &'static str,
        b: &'static str,
        c: &'static str,
        d: &'static str,
        correct: Answer,
    ) -> Self {
        Self {
            question,
            answer_a: a,
            answer_b: b,
            answer_c: c,
            answer_d: d,
            correct_answer: correct,
        }
    }
}

/// A struct representing a record from the database.
#[derive(sqlx::FromRow, Debug, Clone)]
pub(crate) struct TestResult {
    num_correct: i64,
    name: String,
    time_taken: f64,
    date: chrono::NaiveDateTime,
}

/// An attribute by which to sort.
#[derive(Debug, Clone, Copy)]
enum Attribute {
    NumCorrect,
    Time,
}

/// The questions used in the quiz.
const QUESTIONS: [Question; 5] = [
    Question::new(
        "What is the predicate in the following statement: 'John is the brother of Jack.'?",
        "John",
        "brother",
        "Jack",
        "None of the above",
        Answer::B,
    ),
    Question::new(
        "What kind of clause is the following statement: 'A hamster is a mammal'?",
        "Fact",
        "Rule",
        "Query",
        "None of the above",
        Answer::A,
    ),
    Question::new(
        "What kind of clause is the following statement: 'X is the sibling of Y if Z is the parent of X and Z is the parent of Y and X is not Y.'?",
        "Fact",
        "Rule",
        "Query",
        "None of the above",
        Answer::B
    ),
    Question::new(
        "What is the arity of 'brother' in the following statement: 'Jack is the brother of Jill.'?",
        "0",
        "1",
        "2",
        "3",
        Answer::C
    ),
    Question::new(
        "Who is the sister of Bob in the following system: 
'Alice is female.
Jane is female.

Bob is male.
Jack is male.
John is male.

Alice is the parent of Bob.
Jack is the parent of Bob.
Alice is the parent of Jane.
Jack is the parent of John.

X is the sibling of Y if Z is the parent of X and Z is the parent of Y and X is not Y.
X is the sister of Y if X is the sibling of Y and X is female.'?",
        "Alice",
        "Jane",
        "Jack",
        "John",
        Answer::A
    ),
];

/// Compares two instances of TestResult on the given attribute.
fn gt(left: &TestResult, right: &TestResult, attribute: Attribute) -> bool {
    match attribute {
        Attribute::NumCorrect => left.num_correct > right.num_correct,
        Attribute::Time => left.time_taken > right.time_taken,
    }
}

/// Sorts a Vec of TestResult by the given attribute.
fn insertion_sort(items: &mut Vec<TestResult>, start: usize, end: usize, attribute: Attribute) {
    let mut i = start;
    while i < end {
        let mut j = i;
        while j > start && gt(&items[j - 1], &items[j], attribute) {
            (items[j], items[j - 1]) = (items[j - 1].clone(), items[j].clone());
            j = j - 1;
        }
        i = i + 1;
    }
}

/// Saves a result composed from the given parameters to the database.
pub(crate) async fn save_result(
    num_correct: u8,
    name: &str,
    time_taken: Duration,
) -> Result<(), sqlx::Error> {
    dotenv().expect("failed to read environment variables");

    let database_url = env::var("DATABASE_URL").unwrap();
    let mut conn = SqliteConnection::connect(&database_url).await?;

    let date = format!("{}", Local::now());
    let time_taken = time_taken.as_secs_f32();

    query!(
        "INSERT INTO Result (num_correct, name, time_taken, date)
        VALUES (?, ?, ?, ?)",
        num_correct,
        name,
        time_taken,
        date
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}

/// Reads the leaderboard from the database and returns it.
pub(crate) async fn get_leaderboard() -> Result<Vec<TestResult>, sqlx::Error> {
    dotenv().expect("failed to read environment variables");

    let database_url = env::var("DATABASE_URL").unwrap();
    let mut conn = SqliteConnection::connect(&database_url).await?;

    let mut results = query_as!(
        TestResult,
        "SELECT num_correct, name, time_taken, date
        FROM Result"
    )
    .fetch_all(&mut conn)
    .await?;
    let num_results = results.len();

    insertion_sort(&mut results, 0, num_results, Attribute::NumCorrect);
    results.reverse();

    let mut current_correct = results[0].num_correct;
    let mut start = 0;
    let mut sections = vec![];
    for (i, result) in results.iter().enumerate() {
        if result.num_correct != current_correct {
            sections.push((start, i));
            start = i;
            current_correct = result.num_correct;
        }
    }
    sections.push((start, num_results));

    for (start, end) in sections {
        insertion_sort(&mut results, start, end, Attribute::Time);
    }

    Ok(results)
}

/// Formats a time in seconds into the format 'xh + ym + zs'.
fn format_time(seconds: f64) -> String {
    let minutes = (seconds / 60.0).floor();
    let seconds = seconds - minutes * 60f64;

    let hours = (minutes / 60.0).floor();
    let minutes = minutes - hours * 60.0;

    if minutes == 0.0 && hours == 0.0 {
        format!("{seconds}s")
    } else if hours == 0.0 {
        format!("{minutes}m {seconds}s")
    } else {
        format!("{hours}h {minutes}m {seconds}s")
    }
}

pub async fn test() -> Result<(), sqlx::Error> {
    print!("{}", text::TEST_START_TEXT);

    println!("Please enter your name:");
    let name = get_user_input();

    let mut num_correct = 0u8;

    let start_time = Instant::now();

    for question in QUESTIONS {
        println!(
            "{}
A - {}
B - {}
C - {}
D - {}",
            question.question,
            question.answer_a,
            question.answer_b,
            question.answer_c,
            question.answer_d,
        );

        let answer;
        loop {
            print!("> ");
            let _ = io::stdout().flush();
            match Answer::try_from(get_user_input()) {
                Ok(a) => {
                    answer = a;
                    break;
                }
                Err(_) => println!("invalid response"),
            }

            println!();
        }

        if answer == question.correct_answer {
            num_correct = num_correct + 1;
        }
    }

    let time_taken = start_time.elapsed();

    save_result(num_correct, &name, time_taken).await?;

    let results = get_leaderboard().await?;
    println!(
        "{: <10} | {: <15} | {: <15} | {: <10}",
        "Date", "Name", "Correct Answers", "Time taken"
    );
    for result in results {
        println!(
            "{: <10} | {: <15} | {: <15} | {: <10}",
            &result.date.to_string()[..10],
            result.name,
            result.num_correct,
            format_time(result.time_taken.round())
        );
    }

    Ok(())
}
