#![allow(non_snake_case)]

use super::{get_user_input, text};

use chrono::Local;
use dotenv::dotenv;
use sqlx::{query, query_as, Connection, SqliteConnection};
use std::{
    env,
    time::{Duration, Instant},
};

#[derive(PartialEq)]
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

struct Question {
    question: &'static str,
    answer_a: &'static str,
    answer_b: &'static str,
    answer_c: &'static str,
    answer_d: &'static str,
    correct_answer: Answer,
}

impl Question {
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

#[derive(sqlx::FromRow)]
struct TestResult {
    numCorrect: i64,
    name: String,
    timeTaken: f64,
    date: chrono::NaiveDateTime,
}

const QUESTIONS: [Question; 4] = [
    Question::new(
        "What is the predicate in the following statement: 'John is the brother of Jack.'?",
        "John",
        "brother",
        "Jack",
        "None of the above",
        Answer::B,
    ),
    Question::new(
        "What kind of clause is 'mammal' in the following statement: 'A hamster is a mammal'?",
        "Fact",
        "Rule",
        "Query",
        "None of the above",
        Answer::A,
    ),
    Question::new(
        "What kind of clause is 'sibling' in the following statement: 'X is the sibling of Y if Z is the parent of X and Z is the parent of Y and X is not Y.'?",
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
];

async fn save_result(num_correct: u8, name: &str, time_taken: Duration) -> Result<(), sqlx::Error> {
    let database_url = env::vars().find(|x| x.0 == "DATABASE_URL").unwrap().1;
    let mut conn = SqliteConnection::connect(&database_url).await?;

    let date = format!("{}", Local::now());
    let time_taken = time_taken.as_secs_f32();

    query!(
        "INSERT INTO Result (numCorrect, name, timeTaken, date)
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

async fn get_leaderboard() -> Result<Vec<TestResult>, sqlx::Error> {
    let database_url = env::vars().find(|x| x.0 == "DATABASE_URL").unwrap().1;
    let mut conn = SqliteConnection::connect(&database_url).await?;

    query_as!(
        TestResult,
        "SELECT numCorrect, name, timeTaken, date
        FROM Result"
    )
    .fetch_all(&mut conn)
    .await
}

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
    dotenv().ok();

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
            result.numCorrect,
            format_time(result.timeTaken.round())
        );
    }

    Ok(())
}
