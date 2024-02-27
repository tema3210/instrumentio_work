use cucumber::{given, then, when, writer::out::WriteStrExt, World};
use std::{
    fmt::format,
    io::{BufRead, BufReader},
    process::{Child, ChildStdin, ChildStdout, Stdio},
    time::Duration,
};
use tracing::{event, Level};

#[derive(Debug, World)]
pub struct GameWorld {
    the_number: usize,
    game: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,

    /// the data we carry through scenarios
    strings_we_wrote: Vec<String>,
}

type Error = std::borrow::Cow<'static, str>;

impl GameWorld {
    fn read_stdout_line(&mut self) -> Result<Option<String>, Error> {
        let mut line = String::new();
        let res = self.stdout.read_line(&mut line);
        match res {
            Ok(0) => Ok(None),
            Ok(_) => Ok(Some(line)),
            Err(_) => Err("cannot read from pipe".into()),
        }
    }

    fn write_stdin_line<S: AsRef<str>>(&mut self, s: S) -> Result<(), Error> {
        event!(Level::INFO, "Write to a child thread");
        self.stdin
            .write_str(format!("{}\n", s.as_ref()))
            .map_err(|_| "cannot write to child".into())
    }
}

impl Drop for GameWorld {
    fn drop(&mut self) {
        let _ = self.game.kill();
    }
}

impl Default for GameWorld {
    fn default() -> Self {
        use std::process::Command;

        let program = env!("CARGO_BIN_EXE_step_3_1");

        let the_number: usize = 42;

        let mut child = Command::new(program)
            .arg(the_number.to_string())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to spawn a process");

        let stdin = child.stdin.take().expect("cannot take stdin");
        let stdout = BufReader::new(child.stdout.take().expect("cannot take stdout"));

        Self {
            game: child,
            the_number,
            stdin,
            stdout,
            strings_we_wrote: vec![],
        }
    }
}

#[given("a program is running")]
fn hb(world: &mut GameWorld) -> Result<(), std::borrow::Cow<'static, str>> {
    match world.game.try_wait() {
        Ok(Some(status)) => Err(format!("died: {status}").into()),
        Ok(None) => Ok(()),
        Err(e) => Err(format!("Error waiting: {e}").into()),
    }
}

#[when(expr = "player passes string: {}")]
fn pass(world: &mut GameWorld, what: String) -> Result<(), std::borrow::Cow<'static, str>> {
    world.write_stdin_line(&what)?;
    world.strings_we_wrote.push(what);
    Ok(())
}

#[when("player passes winning number")]
fn try_win(world: &mut GameWorld) -> Result<(), std::borrow::Cow<'static, str>> {
    world.write_stdin_line(world.the_number.to_string())?;
    world.strings_we_wrote.push(world.the_number.to_string());
    Ok(())
}

#[then("program ignores a line")]
fn ignores(world: &mut GameWorld) -> Result<(), std::borrow::Cow<'static, str>> {
    let line = world.read_stdout_line()?;
    assert_eq!(line, Some("Guess the number!\n".into()));

    let line = world.read_stdout_line()?;
    assert_eq!(line, Some("Please input your guess.\n".into()));

    let line = world.read_stdout_line()?;
    assert_eq!(line, Some("Please input your guess.\n".into()));

    Ok(())
}

#[then("program works as intended")]
fn works(world: &mut GameWorld) -> Result<(), String> {
    let line = world.read_stdout_line()?;
    assert_eq!(line, Some("Guess the number!\n".into()));

    for s in world.strings_we_wrote.clone() {
        let line = world.read_stdout_line()?;
        assert_eq!(line, Some("Please input your guess.\n".into()));

        if let Ok(num) = s.parse::<usize>() {
            let line = world.read_stdout_line()?;
            assert_eq!(line, Some(format!("You guessed: {}\n", num)));

            if num < world.the_number {
                let line = world.read_stdout_line()?;
                assert_eq!(line, Some("Too small!\n".into()));
            } else if num > world.the_number {
                let line = world.read_stdout_line()?;
                assert_eq!(line, Some("Too big!\n".into()));
            } else {
                let line = world.read_stdout_line()?;
                assert_eq!(line, Some("You win!\n".into()));
                return Ok(());
            }
        } // there's no else branch bc if we enter not a number program doesn't print anything special
    }

    Ok(())
}

#[then("we win")]
fn we_win(world: &mut GameWorld) -> Result<(), String> {
    let line = world.read_stdout_line()?;
    assert_eq!(line, Some("Guess the number!\n".into()));

    for s in world.strings_we_wrote.clone() {
        let line = world.read_stdout_line()?;
        assert_eq!(line, Some("Please input your guess.\n".into()));

        if let Ok(num) = s.parse::<usize>() {
            let line = world.read_stdout_line()?;
            assert_eq!(line, Some(format!("You guessed: {}\n", num)));

            if num == world.the_number {
                let line = world.read_stdout_line()?;
                assert_eq!(line, Some("You win!\n".into()));
                return Ok(());
            } else {
                let _ = world.read_stdout_line()?;
            }
        };
    }

    Err("didn't win".into())
}

fn main() {
    futures::executor::block_on(GameWorld::run("tests/features/"));
}
