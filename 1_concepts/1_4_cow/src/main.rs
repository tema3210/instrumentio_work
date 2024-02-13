use std::borrow::Cow;

const DEFAULT_CONF_PATH: &'static str = "/etc/app/app.conf";

fn main() -> Result<(),&'static str> {
    let mut args = std::env::args();
    let argc: usize = args.len();

    let no_args_case = || {
        if let Ok(env) = std::env::var("APP_CONF")  {
            if env != "" {
                return Ok(Cow::<str>::Owned(env));
            };
            Err("APP_CONF value is empty")
        } else {
            Ok(Cow::<str>::Borrowed(DEFAULT_CONF_PATH))
        }
    };

    let mut window = [args.next(),args.next()];
    let mut args_case = || {
        loop {
            match window {
                [Some(ref f),Some(ref v)] => {
                    match [f.as_str(),v.as_str()] {
                        ["--conf",path] => {
                            if path != "" {
                                return Ok(Cow::Owned(path.into())) // in theory we can avoid even this one, but rust =(
                            };
                            return Err("Err: empty conf argument")
                        },
                        _ => {
                            let [ref mut fst, ref mut snd] = &mut window;
                            std::mem::swap(fst,snd);
                            *snd = args.next();
                        }
                    }
                }
                [None, None] => break,
                [None, Some(_)] => unreachable!("args iter broken...."),
                [Some(_), None] => break,
            }
        };
        no_args_case()
    };

    let path: Cow<'_, str> = if argc < 2 {
        no_args_case()?
    } else {
        args_case()?
    };

    println!("THE path chosen: {path}");
    Ok(())
}
