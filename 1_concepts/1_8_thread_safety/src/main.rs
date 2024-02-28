use std::rc::Rc;

struct AssertSend<T: Send>(T);

struct AssertSync<T: Sync>(T);

struct AssertBoth<T: Send + Sync>(T);



fn main() {
    struct Data (String);
    let mut instance = Data("a".into());
    {
        AssertSend(&instance);
        // AssertSend(Rc::new(&instance));
    };
    {
        AssertSync(&mut instance);
        // AssertSync(Rc::new(&mut instance));
    };
    {
        AssertBoth(&mut instance);
        AssertBoth(&instance);
    };
    {
        thread_local! {
            static IDK: u64 = 500;
        };

        AssertBoth(&IDK) // THIS IS NOT CORRECT:
        // the address of a thread local is the same but the value can be different, due to 
        // this is usually implemented paging
        // we still can send a pointer to this location and access the same location concurently
        // YET the value behind it is thread loca - this is one of older footguns of rust.
    };
}
