use cucumber::{given, then, when, writer::out::WriteStrExt, World};
use std::{io::Write, process::{Child, Stdio}, time::Duration};
use tracing::{event,Level};

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario. 
#[derive(Debug, World)]
pub struct GameWorld {
    the_number: u32,
    game: Child,
}

impl GameWorld {
    fn read_stdin(&mut self) -> Option<String> {
        
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
        let the_number = 42u32; //should be random

        let child = Command::new(program)
            .arg(the_number.to_string())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to spawn a process");

        Self {
            game: child,
            the_number 
        }
    }
}

// Steps are defined with `given`, `when` and `then` attributes.
#[given("a program is running")]
fn hb(world: &mut GameWorld) -> Result<(),std::borrow::Cow<'static, str>> {
    match world.game.try_wait() {
        Ok(Some(status)) => Err(format!("died: {status}").into()),
        Ok(None) => Ok(()),
        Err(e) => Err(format!("Error waiting: {e}").into()),
    }
}

#[when("we pass same number")]
fn consistent(world: &mut GameWorld) -> Result<(),std::borrow::Cow<'static, str>> {
    Err("unimplemented!".into())
}

#[when("we guess a number right")]
fn winnable(world: &mut GameWorld) -> Result<(),std::borrow::Cow<'static, str>> {

    if let Some(pipe_in) = &mut world.game.stdin {
        event!(Level::INFO,"Write to a child thread");
        pipe_in.write_str(format!("{}\n",world.the_number)).map_err(|_| "cannot write to child".into())
    } else {
        Err("cannot open childs pipe".into())
    }
}

#[then("we win")]
fn we_win(world: &mut GameWorld) -> Result<(),String> {
    if let Some(pipe_in) = &mut world.game.stdout {
        let mut buf = Vec::new();
        event!(Level::INFO,"Read to a child thread");
        std::io::Read::read_to_end(pipe_in, &mut buf).map_err(|_| "failed to read to end into buf")?;
        let s = String::from_utf8(buf).map_err(|_| "cannot read buf to string")?;

        if s.contains("You win!")  {
            Ok(())
        } else {
            Err(format!("game is not winnable: {}",s).into())
        }
    } else {
        Err("cannot open childs pipe".into())
    }
}

#[when("we pass not a number input")]
fn sane(world: &mut GameWorld) -> Result<(),std::borrow::Cow<'static, str>> {
    if let Some(pipe_in) = &mut world.game.stdin {
        event!(Level::INFO,"Write to a child thread");
        pipe_in.write_str(format!("{}\n","My stomach ruubmbluuuuuues!")).map_err(|_| "cannot write to child".into())
    } else {
        Err("cannot open childs pipe".into())
    }
}

#[then("program ignores a line")]
fn ignores(world: &mut GameWorld) -> Result<(),std::borrow::Cow<'static, str>> {
    if let Some(pipe_out) = &mut world.game.stdout {
        let mut buf = Vec::new();
        event!(Level::INFO,"Read to a child thread");
        std::io::Read::read_to_end(pipe_out, &mut buf).map_err(|_| "failed to read to end into buf")?;
        let s = String::from_utf8(buf).map_err(|_| "cannot read buf to string")?;

        Err(format!("debug: {s}").into())
        
    } else {
        Err("cannot open childs pipe".into())
    }
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)

    futures::executor::block_on(GameWorld::run("tests/features/"));
}