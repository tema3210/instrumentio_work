use cucumber::{given, then, when, writer::out::WriteStrExt, World};
use std::{io::{BufRead, BufReader}, process::{Child, ChildStdin, ChildStdout, Stdio}, time::Duration};
use tracing::{event,Level};


const THE_NUMBER: usize = 42;

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario. 
#[derive(Debug, World)]
pub struct GameWorld {
    game: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    pid: u32,
}

type Error = std::borrow::Cow<'static, str>;

impl GameWorld {
    fn read_stdout_line(&mut self) -> Result<Option<String>, Error> {
        let mut line = String::new();
        let res = self.stdout.read_line(&mut line);
        match res {
            Ok(0) => Ok(None),
            Ok(_) => Ok(Some(line)),
            Err(_) => Err("cannot read from pipe".into())
        }
    }

    fn write_stdin_line<S: AsRef<str>>(&mut self, s: S) -> Result<(), Error> {
        event!(Level::INFO,"Write to a child thread");
        self.stdin.write_str(format!("{}\n",s.as_ref())).map_err(|_| "cannot write to child".into())
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

        let mut child = Command::new(program)
            .arg(THE_NUMBER.to_string())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to spawn a process");

        let pid = child.id();

        let stdin = child.stdin.take().expect("cannot take stdin");
        let stdout = BufReader::new(child.stdout.take().expect("cannot take stdout"));

        Self {
            game: child,
            pid,
            stdin,
            stdout
        }
    }
}

// Steps are defined with `given`, `when` and `then` attributes.
#[given("a program is running")]
fn hb(world: &mut GameWorld) -> Result<(),std::borrow::Cow<'static, str>> {
    match world.game.try_wait() {
        Ok(Some(status)) => {
            Err(format!("died: {status}").into())
        },
        Ok(None) => {
            Ok(())
        },
        Err(e) => {
            Err(format!("Error waiting: {e}").into())
        },
    }
}

#[when(expr = "we pass string: {}")]
fn pass(world: &mut GameWorld,what: String) -> Result<(),std::borrow::Cow<'static, str>> {
    world.write_stdin_line(what)?;
    // the wait here is to let the process to work
    std::thread::sleep(Duration::from_secs(2));
    Ok(())
}


// the structure of output

// #Please input your guess.
// #...
// -#You guessed: {}
// --# You win!
// --# Too small!
// --# Too big!

#[then("program ignores a line")]
fn ignores(world: &mut GameWorld) -> Result<(),std::borrow::Cow<'static, str>> {
    let line = world.read_stdout_line()?;
    assert_eq!(line, Some("Guess the number!\n".into()));

    let line = world.read_stdout_line()?;
    assert_eq!(line, Some("Please input your guess.\n".into()));

    // let line = world.read_stdout_line()?;
    // assert_eq!(line, Some("boo\n".into()));

    let line = world.read_stdout_line()?;
    assert_eq!(line, Some("Please input your guess.\n".into()));

    Ok(())
}

#[then("program produces same output")]
fn same_output(world: &mut GameWorld) -> Result<(),String> {
    let line = world.read_stdout_line()?;
    assert_eq!(line, Some("Guess the number!\n".into()));

    let line = world.read_stdout_line()?;
    assert_eq!(line, Some("Please input your guess.\n".into()));

    let line = world.read_stdout_line()?;
    assert_eq!(line, Some("You guessed: 12\n".into()));

    let line = world.read_stdout_line()?;
    assert_eq!(line, Some("Too small!\n".into()));

    let line = world.read_stdout_line()?;
    assert_eq!(line, Some("Please input your guess.\n".into()));

    let line = world.read_stdout_line()?;
    assert_eq!(line, Some("You guessed: 12\n".into()));

    let line = world.read_stdout_line()?;
    assert_eq!(line, Some("Too small!\n".into()));

    Ok(())
}

#[then("we win")]
fn we_win(world: &mut GameWorld) -> Result<(),String> {
    let line = world.read_stdout_line()?;
    assert_eq!(line, Some("Guess the number!\n".into()));

    let line = world.read_stdout_line()?;
    assert_eq!(line, Some("Please input your guess.\n".into()));

    let line = world.read_stdout_line()?;
    assert_eq!(line, Some(format!("You guessed: {}\n",THE_NUMBER).into()));

    let line = world.read_stdout_line()?;
    assert_eq!(line, Some("You win!\n".into()));

    Ok(())
}



// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)

    futures::executor::block_on(GameWorld::run("tests/features/"));
}